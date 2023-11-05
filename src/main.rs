mod routes;
mod structs;
mod password_manager;
mod jwt_verification;
mod mailer;

#[allow(unused_imports)]
use axum::{routing::{get, post, delete, put}, Router, extract::{State, FromRef}};
use std::net::SocketAddr;
#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};
use structs::{Data};
use dotenv::dotenv;
use std::env;
use std::fs;
use toml;

#[tokio::main]
async fn main() {

    // loading env vars
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    // db connection
    let client = Client::with_uri_str(format!("mongodb://{}:{}", db_host, db_port)).await.unwrap();
    let db = client.database(format!("{}", db_name).as_str());
    println!("database connected");

    let config_filename = "config.toml";
    let configs = match fs::read_to_string(config_filename){
        Ok(c) => c,
        Err(_) => {
            return println!("{} not loaded or not existent", config_filename);
        }
    };

    // push this data into routes as State
    let _data: Data = match toml::from_str(&configs){
        Ok(config_content) => config_content,
        Err(_) => {
            return println!("Unable to load data from {}", config_filename)
        }
    };
    
    tracing_subscriber::fmt::init();

    // API routes
    let app = Router::new()
        .route("/login", post(routes::route_login))
        .route("/registration", post(routes::route_registration))
        .route("/jwt-auth", post(routes::jwt_auth))
        .route("/verify-account", post(routes::verify_account))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

