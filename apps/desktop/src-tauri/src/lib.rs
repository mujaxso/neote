mod app_state;
mod commands;
mod events;
mod services;
mod adapters;

use tauri::Manager;
use app_state::AppState;

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::workspace::open_workspace,
            commands::workspace::get_file_tree,
            commands::editor::open_file,
            commands::editor::save_file,
            commands::explorer::get_node_children,
        ])
        .setup(|app| {
            // Initialize app state
            let state = app.state::<AppState>();
            state.initialize()?;
            
            // Listen to window events
            let main_window = app.get_webview_window("main").unwrap();
            
            // You can set up global shortcuts here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
