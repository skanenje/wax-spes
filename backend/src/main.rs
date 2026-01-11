mod db;
mod commands;
mod state;

use axum::{
    extract::State,
    http::Method,
    routing::{get, post, put},
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;

use db::Database;
use commands::*;

type AppState = Arc<state::AppState>;

#[tokio::main]
async fn main() {
    // Initialize database
    let db_path = std::env::current_dir()
        .unwrap()
        .join("wax-space.db");
    
    let db = Database::new(db_path).expect("Failed to create database");
    db.init_schema().expect("Failed to initialize schema");
    
    let app_state = Arc::new(state::AppState {
        db: std::sync::Mutex::new(db),
    });

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_origin(Any)
        .allow_headers(Any);

    // Setup routes
    let app = Router::new()
        .route("/api/tools", get(get_all_tools_handler))
        .route("/api/sessions", get(get_all_sessions_handler))
        .route("/api/sessions", post(create_session_handler))
        .route("/api/sessions/:id/activity", put(update_session_activity_handler))
        .layer(cors)
        .with_state(app_state);

    println!("Server running on http://localhost:3001");
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_all_tools_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    match get_all_tools(state).await {
        Ok(tools) => Json(serde_json::json!(tools)),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

async fn get_all_sessions_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    match get_all_sessions(state).await {
        Ok(sessions) => Json(serde_json::json!(sessions)),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

async fn create_session_handler(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let tool_id = payload["tool_id"].as_str().unwrap_or("");
    match create_session(state, tool_id.to_string()).await {
        Ok(session) => Json(serde_json::json!(session)),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

async fn update_session_activity_handler(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    match update_session_activity(state, id).await {
        Ok(_) => Json(serde_json::json!({"success": true})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}