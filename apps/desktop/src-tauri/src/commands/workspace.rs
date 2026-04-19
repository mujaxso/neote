use crate::app_state::AppState;
use tauri::State;

#[tauri::command]
pub async fn open_workspace(
    path: String,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let workspace_service = state.workspace_service()?;
    
    // For now, return a mock response
    // In Phase 2, integrate with actual workspace-model crate
    Ok(serde_json::json!({
        "name": "Workspace",
        "path": path,
        "rootPath": path,
        "projects": []
    }))
}

#[tauri::command]
pub async fn get_file_tree(
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    // Mock file tree for initial scaffolding
    Ok(vec![
        serde_json::json!({
            "path": "/home/user/project/src",
            "name": "src",
            "isDir": true,
            "children": [
                {
                    "path": "/home/user/project/src/main.rs",
                    "name": "main.rs",
                    "isDir": false,
                    "children": []
                }
            ]
        }),
        serde_json::json!({
            "path": "/home/user/project/Cargo.toml",
            "name": "Cargo.toml",
            "isDir": false,
            "children": []
        })
    ])
}
