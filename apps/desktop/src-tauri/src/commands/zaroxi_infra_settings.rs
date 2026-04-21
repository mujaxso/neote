/// Settings commands for Tauri.

use tauri::command;
use zaroxi_theme::{ThemeSettings, ZaroxiTheme};

#[command]
pub async fn load_settings() -> Result<serde_json::Value, String> {
    // TODO: Implement actual settings loading
    Ok(serde_json::json!({
        "theme": "system",
        "editor": {
            "font_size": 14,
            "line_height": 1.5
        }
    }))
}

#[command]
pub async fn save_settings(settings: serde_json::Value) -> Result<(), String> {
    // TODO: Implement actual settings saving
    println!("Saving settings: {:?}", settings);
    Ok(())
}

#[command]
pub async fn load_theme_settings() -> Result<ThemeSettings, String> {
    // TODO: Implement actual settings loading from disk
    // For now, return default (System theme)
    Ok(ThemeSettings::default())
}

#[command]
pub async fn save_theme_settings(settings: ThemeSettings) -> Result<(), String> {
    // TODO: Implement actual settings saving to disk
    println!("Saving theme settings: {:?}", settings);
    Ok(())
}

#[command]
pub async fn get_current_theme() -> Result<ZaroxiTheme, String> {
    // TODO: Load from actual settings
    Ok(ZaroxiTheme::System)
}

#[command]
pub async fn set_theme(theme: ZaroxiTheme) -> Result<(), String> {
    // TODO: Save to settings
    println!("Setting theme to: {:?}", theme);
    Ok(())
}
