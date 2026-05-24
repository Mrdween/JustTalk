use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use tokio;
use std::env;


async fn list_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<Item>>, (StatusCode, String)> {
    let items = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, from_user, to_chat, content, is_read, is_replied, created_at
        FROM items
        ORDER BY id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(items))
}

async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<RequestItem>
) -> Result<(StatusCode, Json<Item>), (StatusCode, String)> {
    let item = sqlx::query_as::<_, Item>(
        r#"
        INSERT INTO (from_user, to_chat, content, is_read, is_replied, created_at)
        VALUES ($1, $1, $3, $4, $5, $6)
        RETURNING id, from_user, to_chat, content, is_read, is_replied, created_at AS description
        "#,
    )
    .bind(&payload.from_user)
    .bind(&payload.to_chat)
    .bind(&payload.content)
    .bind(&payload.is_read)
    .bind(&payload.is_replied)
    .bind(&payload.created_at)
    .frtch_one(&state.db)
    .await
    .map_err(iternal_error)?;

    Ok((StatusCode::CREATED, Json(Item)))
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let state = Appstate { db };

    let app = Router::new()
        .route("/", get(root))
        .route("/items/{id}", get(list_items).post(create_item))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}