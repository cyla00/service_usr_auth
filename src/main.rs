pub mod routes;
use routes::*;

mod mongo;

#[allow(unused_imports)]
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc};

use std::net::SocketAddr;

#[derive(Serialize, Clone)]
struct User{
    username: String,
    email: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = mongo::database_connection();

    let user:User = User{
        username: "testUser".to_owned(),
        email: "email@mail.com".to_owned(),
    };

    let _ = db.await.collection("testCollection").insert_one(user.clone(), None).await;
    println!("email: {} added to database", user.email.clone());


    // API routes
    let app = Router::new()
        .route("/login", post(route_login))
        .route("/registration", post(route_registration));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

