use axum::{
    routing::get,
    Router,
    Json,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use tower_http::services::fs::ServeDir;
use askama_axum::{Template};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    items: Vec<i32>,
}

async fn index() -> impl IntoResponse {
    let items = vec![1,2,3,4,5];
    let template = IndexTemplate {
        name: "Jacob".to_string(),
        items: items,
    };
    (StatusCode::OK, Html(template.render().unwrap()))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
