use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

let app = Router::new()
    .route("/plain_text", get(plain_text))
    .route("/json", get(json));