use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};

pub mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() {
    let durl = std::env::var("DATABASE_URL").expect("set database env variable");

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database");

    let app = Router::new()
        .route("/", get(root))
        .route("/todo", post(controllers::auth::register))
        .layer(cors)
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn root() -> &'static str {
    "Hello, World !"
}
