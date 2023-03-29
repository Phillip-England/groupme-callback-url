use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

mod routes;
use routes::create_routes::create_routes;

#[tokio::main]
async fn main() {
    // welcome message
    println!("Starting server on port 8000");

    // loading env
    dotenv().ok();

    //connecting to mongo db
    let mongo_uri = env::var("MONGO_URI").unwrap();
    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.max_pool_size = Some(10);
    let client = Client::with_options(client_options).unwrap();

    // setting up port
    let port = env::var("PORT").unwrap();
    let addr = format!("0.0.0.0:{}", port);

    // setting up router
    let app = create_routes(client);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
