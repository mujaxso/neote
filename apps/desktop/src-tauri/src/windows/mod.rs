use tauri::{WindowEvent, Manager};

pub fn handle_window_event<R: tauri::Runtime>(window: &tauri::Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            // You can prevent closing here if needed
            // api.prevent_close();
        }
        _ => {}
    }
}

pub fn setup_window<R: tauri::Runtime>(window: &tauri::Window<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Remove native window decorations to use our custom title bar
    // Note: This is already set in tauri.conf.json, but we keep it here for safety
    window.set_decorations(false)?;
    
    // Make the window background transparent for custom styling
    // This helps with custom title bar integration
    window.set_transparent(true)?;
    
    Ok(())
}
