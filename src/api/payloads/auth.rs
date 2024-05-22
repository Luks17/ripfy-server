use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthPayload {
    pub username: String,
    pub pwd: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthTokenPayload {
    pub auth_token: String,
}
