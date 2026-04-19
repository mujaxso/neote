use crate::app_state::AppState;
use tauri::State;

#[tauri::command]
pub async fn open_file(
    path: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    // Mock implementation
    Ok(serde_json::json!({
        "id": "tab_1",
        "path": path,
        "name": path.split('/').last().unwrap_or("untitled"),
        "content": "// File content will be loaded here\n",
        "language": "rust",
        "isDirty": false
    }))
}

#[tauri::command]
pub async fn save_file(
    tab_id: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    // Mock save
    println!("Saving tab {} with content length {}", tab_id, content.len());
    Ok(true)
}
