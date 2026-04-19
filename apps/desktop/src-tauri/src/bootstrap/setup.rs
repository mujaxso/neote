use tauri::AppHandle;

pub fn on_app_ready(_app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // App is ready, start services
    Ok(())
}
