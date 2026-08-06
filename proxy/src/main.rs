use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::{Html, IntoResponse, Json, Sse},
    routing::{get, post},
    Router,
};
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::Stream;

#[derive(Serialize, Deserialize, Debug)]
struct CustomPayload {
    message: String,
    sender: Option<String>,
}

#[derive(Serialize)]
struct ServerResponse {
    server_id: String,
    received_payload: Option<CustomPayload>,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // 1. Backend Server 1 (Port 3001)
    tokio::spawn(async {
        let app = Router::new()
            .route("/server1/events", get(sse_handler_server1))
            .route("/server1/status", get(get_handler_server1))
            .route("/server1/data", post(post_handler_server1));
        
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
        println!("🚀 Server 1 running on http://127.0.0.1:3001");
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Backend Server 2 (Port 3002)
    tokio::spawn(async {
        let app = Router::new()
            .route("/server2/events", get(sse_handler_server2))
            .route("/server2/status", get(get_handler_server2))
            .route("/server2/data", post(post_handler_server2));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
        println!("🚀 Server 2 running on http://127.0.0.1:3002");
        axum::serve(listener, app).await.unwrap();
    });

    // 3. Backend Server 3 (Port 3003)
    tokio::spawn(async {
        let app = Router::new()
            .route("/server3/events", get(sse_handler_server3))
            .route("/server3/status", get(get_handler_server3))
            .route("/server3/data", post(post_handler_server3));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3003").await.unwrap();
        println!("🚀 Server 3 running on http://127.0.0.1:3003");
        axum::serve(listener, app).await.unwrap();
    });

    // Shared Client for Proxy
    let client = Client::new();

    // 4. Main Reverse Proxy Server (Port 3000)
    let proxy_app = Router::new()
        .route("/", get(index_handler))
        // Proxy routes for Server 1
        .route("/server1/*path", get(proxy_to_server1).post(proxy_to_server1))
        // Proxy routes for Server 2
        .route("/server2/*path", get(proxy_to_server2).post(proxy_to_server2))
        // Proxy routes for Server 3
        .route("/server3/*path", get(proxy_to_server3).post(proxy_to_server3))
        .with_state(client);

    let proxy_listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("\n========================================================");
    println!("🌐 Reverse Proxy Dashboard: http://127.0.0.1:3000");
    println!("========================================================\n");

    axum::serve(proxy_listener, proxy_app).await.unwrap();
}

// =========================================================================
// Backend 1 Handlers (SSE, GET, POST)
// =========================================================================

async fn sse_handler_server1() -> Sse<impl Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    println!("➔ [Server 1] SSE client connected");
    let mut count: u64 = 0;
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
        count += 1;
        Ok(axum::response::sse::Event::default()
            .event("message")
            .data(format!("[Server 1] Tick #{}", count)))
    });
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn get_handler_server1() -> Json<ServerResponse> {
    Json(ServerResponse {
        server_id: "Server 1".into(),
        received_payload: None,
        timestamp: chrono_now(),
    })
}

async fn post_handler_server1(Json(payload): Json<CustomPayload>) -> Json<ServerResponse> {
    println!("➔ [Server 1] Received POST payload: {:?}", payload);
    Json(ServerResponse {
        server_id: "Server 1".into(),
        received_payload: Some(payload),
        timestamp: chrono_now(),
    })
}

// =========================================================================
// Backend 2 Handlers (SSE, GET, POST)
// =========================================================================

async fn sse_handler_server2() -> Sse<impl Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    println!("➔ [Server 2] SSE client connected");
    let mut count: u64 = 0;
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
        count += 1;
        Ok(axum::response::sse::Event::default()
            .event("message")
            .data(format!("[Server 2] Tick #{}", count)))
    });
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn get_handler_server2() -> Json<ServerResponse> {
    Json(ServerResponse {
        server_id: "Server 2".into(),
        received_payload: None,
        timestamp: chrono_now(),
    })
}

async fn post_handler_server2(Json(payload): Json<CustomPayload>) -> Json<ServerResponse> {
    println!("➔ [Server 2] Received POST payload: {:?}", payload);
    Json(ServerResponse {
        server_id: "Server 2".into(),
        received_payload: Some(payload),
        timestamp: chrono_now(),
    })
}

// =========================================================================
// Backend 3 Handlers (SSE, GET, POST)
// =========================================================================

async fn sse_handler_server3() -> Sse<impl Stream<Item = Result<axum::response::sse::Event, Infallible>>> {
    println!("➔ [Server 3] SSE client connected");
    let mut count: u64 = 0;
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
        count += 1;
        Ok(axum::response::sse::Event::default()
            .event("message")
            .data(format!("[Server 3] Tick #{}", count)))
    });
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn get_handler_server3() -> Json<ServerResponse> {
    Json(ServerResponse {
        server_id: "Server 3".into(),
        received_payload: None,
        timestamp: chrono_now(),
    })
}

async fn post_handler_server3(Json(payload): Json<CustomPayload>) -> Json<ServerResponse> {
    println!("➔ [Server 3] Received POST payload: {:?}", payload);
    Json(ServerResponse {
        server_id: "Server 3".into(),
        received_payload: Some(payload),
        timestamp: chrono_now(),
    })
}

// =========================================================================
// Proxy Dispatchers
// =========================================================================

async fn proxy_to_server1(
    State(client): State<Client>,
    Path(path): Path<String>,
    req: Request,
) -> impl IntoResponse {
    let target_url = format!("http://127.0.0.1:3001/server1/{}", path);
    forward_request(client, target_url, req).await
}

async fn proxy_to_server2(
    State(client): State<Client>,
    Path(path): Path<String>,
    req: Request,
) -> impl IntoResponse {
    let target_url = format!("http://127.0.0.1:3002/server2/{}", path);
    forward_request(client, target_url, req).await
}

async fn proxy_to_server3(
    State(client): State<Client>,
    Path(path): Path<String>,
    req: Request,
) -> impl IntoResponse {
    let target_url = format!("http://127.0.0.1:3003/server3/{}", path);
    forward_request(client, target_url, req).await
}

async fn forward_request(client: Client, target_url: String, req: Request) -> impl IntoResponse {
    let method = req.method().clone();
    
    // Read body bytes to forward in case of POST/PUT requests
    let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to read request body: {}", err),
            )
                .into_response();
        }
    };

    let mut proxy_req = client.request(method, &target_url).body(body_bytes);
    
    // Set headers (e.g. Content-Type)
    proxy_req = proxy_req.header("Content-Type", "application/json");

    let response = match proxy_req.send().await {
        Ok(res) => res,
        Err(err) => {
            return (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("Proxy error reaching {}: {}", target_url, err),
            )
                .into_response();
        }
    };

    let mut builder = axum::response::Response::builder().status(response.status());

    for (name, value) in response.headers() {
        builder = builder.header(name, value);
    }

    let stream = response.bytes_stream();
    let body = Body::from_stream(stream);

    builder.body(body).unwrap_or_else(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to assemble proxy response",
        )
            .into_response()
    })
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    format!("{}s", since_the_epoch.as_secs())
}

// =========================================================================
// Frontend Interface
// =========================================================================

async fn index_handler() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>3-Server Sticky Path Load Balancer Test</title>
        <style>
            body { font-family: sans-serif; background: #121212; color: #e0e0e0; margin: 20px; }
            .grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 15px; }
            .card { border: 1px solid #333; padding: 15px; border-radius: 8px; background: #1e1e1e; }
            .logs { height: 200px; overflow-y: auto; background: #000; padding: 10px; border-radius: 4px; border: 1px solid #222; font-family: monospace; font-size: 12px; margin-top: 10px; }
            .actions { margin-bottom: 10px; display: flex; flex-direction: column; gap: 5px; }
            button { cursor: pointer; padding: 6px 12px; background: #2b2b2b; color: #fff; border: 1px solid #555; border-radius: 4px; }
            button:hover { background: #3b3b3b; }
            .response-box { background: #111; border: 1px solid #333; padding: 5px; min-height: 40px; font-family: monospace; font-size: 11px; white-space: pre-wrap; margin-top: 5px; word-break: break-all; }
        </style>
    </head>
    <body>
        <h2>Sticky Route Proxy Dashboard (3 Servers)</h2>
        <p>Testing path-isolated <strong>SSE</strong>, <strong>GET</strong>, and <strong>POST</strong> endpoints through Proxy (Port 3000).</p>

        <div class="grid">
            <!-- Server 1 Card -->
            <div class="card">
                <h3>Server 1 (<code>/server1/*</code>)</h3>
                <div class="actions">
                    <button onclick="fetchGet('/server1/status', 'get1')">GET Status</button>
                    <div id="get1" class="response-box">...</div>
                    
                    <button onclick="sendPost('/server1/data', 'Server 1 Payload', 'post1')">POST Data</button>
                    <div id="post1" class="response-box">...</div>

                    <button onclick="connectSse('/server1/events', 'sse1')">Start SSE Stream</button>
                    <button onclick="disconnectSse('sse1')">Stop SSE Stream</button>
                </div>
                <div id="sse1" class="logs"></div>
            </div>

            <!-- Server 2 Card -->
            <div class="card">
                <h3>Server 2 (<code>/server2/*</code>)</h3>
                <div class="actions">
                    <button onclick="fetchGet('/server2/status', 'get2')">GET Status</button>
                    <div id="get2" class="response-box">...</div>

                    <button onclick="sendPost('/server2/data', 'Server 2 Payload', 'post2')">POST Data</button>
                    <div id="post2" class="response-box">...</div>

                    <button onclick="connectSse('/server2/events', 'sse2')">Start SSE Stream</button>
                    <button onclick="disconnectSse('sse2')">Stop SSE Stream</button>
                </div>
                <div id="sse2" class="logs"></div>
            </div>

            <!-- Server 3 Card -->
            <div class="card">
                <h3>Server 3 (<code>/server3/*</code>)</h3>
                <div class="actions">
                    <button onclick="fetchGet('/server3/status', 'get3')">GET Status</button>
                    <div id="get3" class="response-box">...</div>

                    <button onclick="sendPost('/server3/data', 'Server 3 Payload', 'post3')">POST Data</button>
                    <div id="post3" class="response-box">...</div>

                    <button onclick="connectSse('/server3/events', 'sse3')">Start SSE Stream</button>
                    <button onclick="disconnectSse('sse3')">Stop SSE Stream</button>
                </div>
                <div id="sse3" class="logs"></div>
            </div>
        </div>

        <script>
            const activeSources = {};

            async function fetchGet(url, outputId) {
                try {
                    const res = await fetch(url);
                    const json = await res.json();
                    document.getElementById(outputId).innerText = JSON.stringify(json, null, 2);
                } catch (e) {
                    document.getElementById(outputId).innerText = 'Error: ' + e;
                }
            }

            async function sendPost(url, msgText, outputId) {
                try {
                    const res = await fetch(url, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ message: msgText, sender: "Proxy-Client" })
                    });
                    const json = await res.json();
                    document.getElementById(outputId).innerText = JSON.stringify(json, null, 2);
                } catch (e) {
                    document.getElementById(outputId).innerText = 'Error: ' + e;
                }
            }

            function connectSse(path, logId) {
                disconnectSse(logId);
                const logDiv = document.getElementById(logId);
                logDiv.innerHTML += `<div style="color: yellow">Connecting to ${path}...</div>`;
                
                const evtSource = new EventSource(path);

                evtSource.onmessage = (e) => {
                    logDiv.innerHTML += `<div style="color: #00ff66">[${new Date().toLocaleTimeString()}] ${e.data}</div>`;
                    logDiv.scrollTop = logDiv.scrollHeight;
                };

                evtSource.onerror = () => {
                    logDiv.innerHTML += `<div style="color: red">Connection Error</div>`;
                };

                activeSources[logId] = evtSource;
            }

            function disconnectSse(logId) {
                if (activeSources[logId]) {
                    activeSources[logId].close();
                    delete activeSources[logId];
                    document.getElementById(logId).innerHTML += `<div style="color: #888">Disconnected</div>`;
                }
            }
        </script>
    </body>
    </html>
    "#)
}
