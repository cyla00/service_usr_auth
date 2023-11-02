mod routes;
mod structs;
mod password_manager;
mod jwt_verification;
mod mailer;

#[allow(unused_imports)]
use axum::{routing::{get, post, delete, put}, Router, extract::State};
use std::net::SocketAddr;
#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};
use dotenv::dotenv;
use std::env;

#[allow(unused_imports)]
use toml;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _db_host = env::var("DB_HOST").unwrap();
    let _db_port = env::var("DB_PORT").unwrap();
    let _db_name = env::var("DB_NAME").unwrap();

    let client = Client::with_uri_str(format!("mongodb://{_db_host}:{_db_port}")).await.unwrap();
    let db = client.database("{_db_name}");
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

