use axum::{
    extract::Path, http::{self, Method}, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use serde::Deserialize;
use reqwest::{Error, StatusCode};
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
        .route("/migration/{migration_blob}", get(migration_data))
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
pub struct MigrationPayload {
    pub data: String,
}

#[axum::debug_handler]
pub async fn handle_migration(
    Json(payload): Json<MigrationPayload>,
) -> impl IntoResponse {
    println!("yabadaba: {}", payload.data);
    let res = upload_to_walrus(payload.data).await;
    println!("res: {:#?}", res);
}

async fn upload_to_walrus(payload: String) -> Result<(), Error> {
    let url = "https://publisher.walrus-testnet.walrus.space/v1/blobs?epochs=5";

    let client = reqwest::Client::new();
    let res = client
        .put(url)
        .body(payload)
        .send()
        .await?;

    println!("Status: {}", res.status());
    let text = res.text().await?;
    println!("Body: {}", text);

    Ok(())
}

pub async fn migration_data(
    Path(migration_blob): Path<String>,
) -> impl IntoResponse {
    println!("bazinga: {}", migration_blob);
    
    match get_data_from_walrus(migration_blob).await {
        Ok(body) => (
            StatusCode::OK,
            body,
        ).into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch: {}", e),
        ).into_response(),
    }


}

async fn get_data_from_walrus(blob_id: String) -> Result<String, Error> {
    let url = format!(
        "https://aggregator.walrus-testnet.walrus.space/v1/blobs/{}",
        blob_id
    );
    
    let response = reqwest::get(&url).await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(body)
}
