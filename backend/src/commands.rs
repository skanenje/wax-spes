use crate::state::AppState;
use std::sync::Arc;

pub async fn get_all_tools(state: Arc<AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().unwrap();
    let tools = db.get_all_tools().map_err(|e| e.to_string())?;
    Ok(tools.into_iter().map(|t| serde_json::to_value(t).unwrap()).collect())
}

pub async fn get_all_sessions(state: Arc<AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().unwrap();
    let sessions = db.get_all_sessions().map_err(|e| e.to_string())?;
    Ok(sessions.into_iter().map(|s| serde_json::to_value(s).unwrap()).collect())
}

pub async fn create_session(state: Arc<AppState>, tool_id: String) -> Result<serde_json::Value, String> {
    let db = state.db.lock().unwrap();
    let session = db.create_session(tool_id).map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(session).unwrap())
}

pub async fn update_session_activity(state: Arc<AppState>, session_id: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.update_session_activity(session_id).map_err(|e| e.to_string())
}