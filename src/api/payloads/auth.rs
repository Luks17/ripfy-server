use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AuthPayload {
    pub username: String,
    pub pwd: String,
}
