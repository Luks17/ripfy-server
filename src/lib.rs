pub mod api;
pub mod conf;
pub mod context;
pub mod crypt;
pub mod db;
pub mod util;

pub use conf::config;
pub use conf::keys;

use axum::middleware;
use axum::Router;
use sea_orm::DatabaseConnection;
use tower_cookies::CookieManagerLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

// The difference between layers and route_layers used here is that route_layers apply only when
// the user matches with a route that exists
pub fn build_app(state: AppState) -> Router {
    let routes_rest = Router::new()
        .merge(api::song::router(state.clone()))
        .merge(api::playlist::router(state.clone()))
        .route_layer(middleware::from_fn(api::mw::ctx::ctx_require_auth));

    Router::new()
        .nest("/api", api::auth::router(state.clone()))
        .nest("/api", routes_rest)
        .merge(api::stream::router(state.clone()))
        .layer(middleware::from_fn(api::mw::ctx::ctx_resolver))
        .layer(CookieManagerLayer::new())
}
