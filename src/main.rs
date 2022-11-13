use axum::Extension;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

use axum::{routing::post, Router};

mod routes;

#[tokio::main]
async fn main() {
    let db = Arc::new(Pool::<Sqlite>::connect("sqlite://umami.db").await.unwrap());

    let app = Router::new()
        .route("/api/v1/receive", post(routes::receive_message))
        .layer(Extension(db));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
