use super::{error::Error, error::Result, gen_and_set_token_cookie, ResponseModel};
use crate::{
    api::{error::ClientError, payloads::auth::AuthPayload, ResponseModelUser},
    crypt::passwd::{gen_salt, passwd_encrypt, verify_encrypted_passwd},
    db, AppState,
};
use axum::{extract::State, routing::post, Json, Router};
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .with_state(state)
}

/// Receives a payload of format: { username, passwd }
/// Checks if user exists and it's password is correct
/// If everything goes fine, generates an access token for said user and stores it on the cookies
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
    cookies: Cookies,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("LOGIN HANDLER");

    let AuthPayload { username, pwd } = payload;

    let user = match db::user::first_by_username(&state, &username).await {
        Ok(u) => u.ok_or(Error::UserNotFound)?,
        Err(_) => return Err(Error::DbSelectFailed),
    };

    let is_passwd_correct = verify_encrypted_passwd(pwd, user.passwd.as_str())?;

    if !is_passwd_correct {
        return Err(Error::IncorrectPasswd);
    }

    gen_and_set_token_cookie(&cookies, &user.id).await?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}

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
