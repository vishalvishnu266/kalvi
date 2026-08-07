use axum::{
    extract::Query,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

// The folder where `npm run bundle:ota` drops zipped Vue builds
const BUNDLES_DIR: &str = "../bundles";
// Public base URL used by the Android emulator (10.0.2.2 == host machine)
const PUBLIC_BASE_URL: &str = "http://10.0.2.2:3000";

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

#[derive(Deserialize, Serialize, Default)]
struct LatestManifest {
    latest: String,
    file: String,
    #[serde(default)]
    created_at: String,
}

/// Look up the newest bundle available on disk.
/// Prefers `bundles/latest.json` (written by scripts/build-bundle.mjs) and
/// falls back to picking the most recently modified `v*.zip` file.
fn find_latest_bundle() -> Option<(String, String)> {
    let dir = PathBuf::from(BUNDLES_DIR);

    // 1) Prefer explicit manifest
    let manifest_path = dir.join("latest.json");
    if let Ok(bytes) = fs::read(&manifest_path) {
        if let Ok(m) = serde_json::from_slice::<LatestManifest>(&bytes) {
            if !m.latest.is_empty() && !m.file.is_empty() {
                return Some((m.latest, m.file));
            }
        }
    }

    // 2) Fallback: newest v*.zip on disk
    let entries = fs::read_dir(&dir).ok()?;
    let mut newest: Option<(std::time::SystemTime, String)> = None;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name()?.to_string_lossy().to_string();
        if !name.starts_with('v') || !name.ends_with(".zip") { continue; }
        let modified = entry.metadata().and_then(|m| m.modified()).ok()?;
        if newest.as_ref().map_or(true, |(t, _)| modified > *t) {
            newest = Some((modified, name));
        }
    }
    newest.map(|(_, file)| {
        // Strip leading 'v' and trailing '.zip' to derive the version
        let version = file.trim_start_matches('v').trim_end_matches(".zip").to_string();
        (version, file)
    })
}

async fn check_update(Query(params): Query<UpdateQuery>) -> impl IntoResponse {
    let client_version = params.version.unwrap_or_else(|| "0.0.0".to_string());

    let Some((latest_version, latest_file)) = find_latest_bundle() else {
        println!("[Server] No bundles found in {}", BUNDLES_DIR);
        return Json(UpdateResponse {
            update_available: false,
            version: client_version,
            url: None,
        });
    };

    let update_available = client_version != latest_version;
    println!(
        "[Server] Check: client=v{} latest=v{} -> update_available={}",
        client_version, latest_version, update_available
    );

    Json(UpdateResponse {
        update_available,
        version: latest_version.clone(),
        url: if update_available {
            Some(format!("{}/bundles/{}", PUBLIC_BASE_URL, latest_file))
        } else {
            None
        },
    })
}

async fn health() -> impl IntoResponse {
    match find_latest_bundle() {
        Some((v, f)) => Json(serde_json::json!({ "ok": true, "latest": v, "file": f })),
        None => Json(serde_json::json!({ "ok": true, "latest": null })),
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/check-update", get(check_update))
        .route("/health", get(health))
        .nest_service("/bundles", ServeDir::new(BUNDLES_DIR))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Axum OTA Server listening on http://0.0.0.0:3000");
    println!("   Bundles served from: {}", BUNDLES_DIR);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
