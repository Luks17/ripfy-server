use super::{error::Error, error::Result, mw::AUTH_TOKEN};
use crate::AppState;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .with_state(state)
}

async fn login_handler(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    tracing::debug!("LOGIN HANDLER");

    if payload.username != "user" || payload.pwd != "passwd" {
        return Err(Error::LoginFailed);
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
