use axum::{
    http::{self, Method},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = create_listener("0.0.0.0:8080").await;
    let router = create_router().await;
    if let Ok(local_address) = listener.local_addr() {
        println!("Listening on: {}", local_address);
    }

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    return Ok(());
}


pub async fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(Any);
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/migrationdata", post(handle_migration))
        .layer(cors);
    return app;
}

pub async fn create_listener(address: &str) -> TcpListener {
    let listener = TcpListener::bind(address).await.unwrap();
    return listener;
}

async fn hello_world() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "data": "Hello World!"
    });

    Json(json_response)
}

#[derive(Debug, Deserialize)]
pub struct MigrationData {
    pub data: String,
}

#[axum::debug_handler]
pub async fn handle_migration(
    Json(payload): Json<MigrationData>,
) -> impl IntoResponse {
    println!("yabadaba: {}", payload.data);

}
