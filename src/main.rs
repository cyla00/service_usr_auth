mod mongo;
mod routes;
mod structs;
mod password_manager;

#[allow(unused_imports)]
use axum::{routing::{get, post, delete, put}, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // API routes
    let app = Router::new()
        .route("/login", post(routes::route_login))
        .route("/registration", post(routes::route_registration));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

