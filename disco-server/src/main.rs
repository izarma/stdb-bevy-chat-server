use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rand::{Rng, rng};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex}, time::{Duration, Instant},
};

#[derive(Clone)]
struct AppState {
    cache: Arc<Mutex<HashMap<String, (String, Instant)>>>,
}

struct AuthResponse {
    code: String,
    csrf: String,
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:42069";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}

fn router() -> Router {
    let state = AppState {
        cache: Arc::new(Mutex::new(HashMap::new())),
    };
    Router::new()
        .route("/", get(hello_world))
        .route("/csrf/{identity}", get(get_csrf))
        .with_state(state)
}

async fn hello_world() -> &'static str {
    "Hello from Axum!"
}

async fn get_csrf(State(state): State<AppState>, Path(identity): Path<String>) -> String {
    let mut cache = state.cache.lock().unwrap();
    let now = Instant::now();
    if let Some((csrf, timestamp)) = cache.get(&identity) {
        if now.duration_since(*timestamp) < Duration::from_secs(3600) {
            return csrf.to_string();
        }
    }
    let csrf = generate_csrf();
    cache.insert(identity, (csrf.clone(), now));
    csrf
}

fn generate_csrf() -> String {
    let random_bytes: Vec<u8> = (0..16).map(|_| rng().random::<u8>()).collect();
    BASE64_URL_SAFE_NO_PAD.encode(random_bytes)
}
