pub mod routes;
use routes::*;

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
use mongodb::{Client, options::ClientOptions};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/login", post(route_login))
        .route("/registration", post(route_registration));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

