use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app: Router = Router::new()
        .merge(SwaggerUi::new("/swagger").url("/docs/api_spec.json", ApiDoc::openapi()));

    tracing::info!("Successfully generated api_spec.json!");

    let socket_address = SocketAddr::from(([0, 0, 0, 0], 17000));
    let listener = TcpListener::bind(&socket_address).await?;

    tracing::info!("Swagger is listening on port 17000 at /swagger...");

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    info(description = "
## About
This REST API uses JSON to share and manipulate resources, mainly music tracks.
Most routes are protected and require an access token appended to the request.

To get an access token, please login using the route at the auth routes section.
The access token will be appended as a cookie at the response headers.

## Response Format and Errors
All responses have the following format:

On a successfuly query:
```
{
    'data': {Response}
}
```

On a failed query:
```
{
    'error': {
        'type': {Type}
    }
}
```
"),
    paths(ripfy_server::api::auth::login_handler),
    components(schemas(ripfy_server::api::auth::AuthPayload))
)]
struct ApiDoc;
