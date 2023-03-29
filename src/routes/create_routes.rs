use axum::{
  http::Method,
  routing::post, 
  Router, Json
};
use mongodb::Client;
use tower_http::cors::{Any, CorsLayer};
use serde_json::{json, Value};


pub async fn create_routes(client: Client) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(Any);

    return Router::new()
    .route("/", post(|Json(payload): Json<serde_json::Value>| async { 
      println!("HIT");
      dbg!(payload);
    }))
    .layer(cors)
}
