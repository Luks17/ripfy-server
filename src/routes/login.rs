use super::{error::Error, error::Result, mw::AUTH_TOKEN};
use crate::{helpers, util::crypt::verify_encrypted_passwd, AppState};
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .with_state(state)
}

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

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sig"));

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
