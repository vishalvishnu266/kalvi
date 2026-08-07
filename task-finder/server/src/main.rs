use axum::{
    extract::Query,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

// Update this version string when you prepare a new bundle zip
const LATEST_VERSION: &str = "1.0.1";

// 10.0.2.2 points to host machine from Android Emulator
const BUNDLE_URL: &str = "http://10.0.2.2:3000/bundles/v1.0.1.zip";

#[derive(Deserialize)]
struct UpdateQuery {
    version: Option<String>,
}

#[derive(Serialize)]
struct UpdateResponse {
    update_available: bool,
    version: String,
    url: Option<String>,
}

async fn check_update(Query(params): Query<UpdateQuery>) -> impl IntoResponse {
    let client_version = params.version.unwrap_or_else(|| "1.0.0".to_string());
    let update_available = client_version != LATEST_VERSION;

    println!("[Server] Check request: Client v{}, Latest v{}", client_version, LATEST_VERSION);

    Json(UpdateResponse {
        update_available,
        version: LATEST_VERSION.to_string(),
        url: if update_available { Some(BUNDLE_URL.to_string()) } else { None },
    })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/check-update", get(check_update))
        // Serves zipped assets stored in the root ../bundles folder
        .nest_service("/bundles", ServeDir::new("../bundles"))
        .layer(cors);

    // Bind to 0.0.0.0 so the emulator can connect
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Axum OTA Server listening on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}