use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

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

#[utoipauto(paths = "
    ./src/api/mod.rs from ripfy_server,
    ./src/api/auth.rs from ripfy_server,
    ./src/api/song.rs from ripfy_server,
    ./src/api/playlist.rs from ripfy_server,
    ./entity/src/user.rs from entity,
    ./entity/src/song.rs from entity,
    ./entity/src/playlist.rs from entity
")]
#[derive(OpenApi)]
#[openapi(info(description = "
## About
This REST API uses JSON to share and manipulate resources, mainly music tracks.
Most routes are protected and require an access token appended to the request.

To get an access token, please login using the route at the auth routes section.
The access token will be appended as a cookie at the response headers.

## Response Format and Errors
All responses have the following format:

### On a successfuly query:
```
{
    'success': true,
    'data': {Response}
}
```

### Or just:
```
{
    'success': true
}
```

### On a failed query:
```
{
    'success': false,
    'error': {Type}
}
```
"))]
struct ApiDoc;
