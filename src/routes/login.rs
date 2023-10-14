use super::{error::Error, error::Result, gen_and_set_token_cookie};
use crate::{crypt::passwd::verify_encrypted_passwd, helpers, AppState};
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .with_state(state)
}

/// Receives a payload of format: { username, passwd }
/// Checks if user exists and it's password is correct
/// If everything goes fine, generates an access token for said user and stores it on the cookies
async fn login_handler(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("LOGIN HANDLER");

    let LoginPayload { username, pwd } = payload;

    let user = match helpers::user::first_by_username(&state, &username).await {
        Ok(u) => u.ok_or(Error::UserNotFound)?,
        Err(_) => return Err(Error::DbQueryFailed),
    };

    let is_passwd_correct = verify_encrypted_passwd(pwd, user.passwd.as_str())?;

    if !is_passwd_correct {
        return Err(Error::IncorrectPasswd);
    }

    gen_and_set_token_cookie(&cookies, &user.id).await?;

    Ok(Json(json!({
            "result": {
                "success": true
            }
        }
    )))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
