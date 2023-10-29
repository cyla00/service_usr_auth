mod routes;
mod structs;
mod password_manager;
mod jwt_verification;

#[allow(unused_imports)]
use axum::{routing::{get, post, delete, put}, Router, extract::State};
use std::net::SocketAddr;
#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};

#[tokio::main]
async fn main() {

    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = client.database("testDb");
    println!("database connected");
    
    tracing_subscriber::fmt::init();

    // API routes
    let app = Router::new()
        .route("/login", post(routes::route_login))
        .route("/registration", post(routes::route_registration))
        .route("/test", post(routes::test_jwt))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

