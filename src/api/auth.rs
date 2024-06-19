use super::{error::Error, error::Result, ResponseModel};
use crate::{
    api::{
        error::ClientError,
        payloads::auth::{AuthPayload, AuthTokenPayload},
        AuthModel, ResponseModelUser,
    },
    config,
    crypt::{
        passwd::{gen_salt, passwd_encrypt, verify_encrypted_passwd},
        token::Token,
    },
    db, keys,
    util::redis::RedisConnection,
    AppState,
};
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/refresh-token", post(refresh_token_handler))
        .with_state(state)
}

/// Receives a payload of format: { username, pwd }
/// Checks if user exists and it's password is correct
/// If everything goes fine, generates a token pair for the user and returns it
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Logged in successfully", body = ResponseModelUser,
            example = json!(ResponseModelUser { success: true, data: None, error: None })),
        (status = 401, description = "Credentials incorrect, login failed", body = ResponseModelUser,
            example = json!(ResponseModelUser {success: false, data: None, error: Some(ClientError::LOGIN_FAIL.as_ref().to_string())})),
        (status = 500, description = "Something went wrong during login", body = ResponseModelUser,
            example = json!(ResponseModelUser {success: false, data: None, error: Some(ClientError::SERVICE_ERROR.as_ref().to_string())}))
    )
)]
async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("LOGIN HANDLER");

    let AuthPayload { username, pwd } = payload;

    let mut redis_conn = RedisConnection::from_app_state(&state).await?;

    let user = match db::user::first_by_username(&state, &username).await {
        Ok(u) => u.ok_or(Error::UserNotFound)?,
        Err(_) => return Err(Error::DbSelectFailed),
    };

    let is_passwd_correct = verify_encrypted_passwd(pwd, user.passwd.as_str())?;

    if !is_passwd_correct {
        return Err(Error::IncorrectPasswd);
    }

    let (access_token, refresh_token) = Token::new_token_pair(&user.id).await?;

    redis_conn
        .setex(
            refresh_token.identifier.clone(),
            user.id,
            config().refresh_token_duration_secs,
        )
        .await?;

    Ok(Json(json!(ResponseModel::<AuthModel> {
        success: true,
        data: Some(AuthModel {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string()
        }),
        error: None
    })))
}

#[utoipa::path(
        post,
        path = "/api/signup",
        request_body = AuthPayload,
        responses
        (
            (status = 200, description = "Singup successfull", body = ResponseModel,
            example = json!(ResponseModel ::<()> { success: true, data: None, error: None })),
        (status = 500, description = "Somethings unexpected happened", body = ResponseModel,
            example = json!(ResponseModel::<()> {success: false, data: None, error: Some(ClientError::SERVICE_ERROR.as_ref().to_string())})),
        (status = 409, description = "User already exists", body = ResponseModelUser,
            example = json!(ResponseModel ::<()> {success: false, data: None, error: Some(ClientError::USERNAME_ALREADY_USED.as_ref().to_string())}))
        )
)]
async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("SIGNUP HANDLER");

    let AuthPayload { username, pwd } = payload;

    let user_already_exist = match db::user::first_by_username(&state, &username).await {
        Ok(u) => u.is_some(),
        Err(_) => return Err(Error::DbSelectFailed),
    };

    if user_already_exist {
        return Err(Error::UserAlreadyExists);
    }

    let hashed_pwd = passwd_encrypt(pwd, gen_salt())?;

    db::user::create_new_user(&state, &username, &hashed_pwd)
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}

#[utoipa::path(
    post,
    path = "/api/refresh-token",
    request_body = AuthTokenPayload,
    responses
        (
        (status = 200, description = "Refresh token successfull", body = ResponseModelUser,
            example = json!(ResponseModelUser ::<()> { success: true, data: Some, error: None })),
        (status = 400, description = "Somethings unexpected happened", body = ResponseModelUser,
            example = json!(ResponseModelUser ::<()> {success: false, data: None, error: Some(ClientError::INVALID_BODY.as_ref().to_string())})))
       
)]

async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthTokenPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("REFRESH TOKEN HANDLER");

    let mut redis_conn = RedisConnection::from_app_state(&state).await?;

    let AuthTokenPayload { auth_token } = payload;

    let token: Token = auth_token.parse()?;
    token.validate(&keys().verifying_key)?;

    if token.is_access_token().is_ok() {
        return Err(Error::InvalidRefreshToken);
    }

    let user_id = redis_conn
        .getdel(token.identifier)
        .await
        .map_err(|_| Error::InvalidRefreshToken)?;

    let (access_token, refresh_token) = Token::new_token_pair(&user_id).await?;

    redis_conn
        .setex(
            refresh_token.identifier.clone(),
            user_id,
            config().refresh_token_duration_secs,
        )
        .await?;

    Ok(Json(json!(ResponseModel::<AuthModel> {
        success: true,
        data: Some(AuthModel {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string()
        }),
        error: None
    })))
}
