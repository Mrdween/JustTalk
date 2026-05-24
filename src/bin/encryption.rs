mod sockets;

use axum::extract::{Path, Query, Json};
use std::collections::HashMap;

async fn path(Path(user_id): Path<u32>) {}
// create the path for message
async fn query(Query(params): Query<HashMap<String, String>>) {}
// form the query for message on `127.0.0.1:3000`
async fn json(Json(payload): Json<serde_json::Value>) {}
// generate resonse for query

