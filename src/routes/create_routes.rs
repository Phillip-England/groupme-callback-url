use axum::{http::Method, routing::get, Router};
use mongodb::Client;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes(client: Client) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    return Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(cors)
}