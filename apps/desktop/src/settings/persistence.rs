//! Persistence for editor typography settings.
//!
//! Handles saving and loading editor font settings to/from disk.

use std::fs;
use std::path::PathBuf;
use crate::settings::editor::EditorTypographySettings;

const SETTINGS_FILE_NAME: &str = "editor_typography.json";

/// Get the path to the settings file in the user's config directory.
fn settings_path() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| "Could not find config directory".to_string())?;
    
    path.push("neote");
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    path.push(SETTINGS_FILE_NAME);
    Ok(path)
}

/// Save editor typography settings to disk.
pub fn save_settings(settings: &EditorTypographySettings) -> Result<(), String> {
    let path = settings_path()?;
    
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write settings to {}: {}", path.display(), e))?;
    
    Ok(())
}

/// Load editor typography settings from disk.
/// Returns default settings if file doesn't exist or can't be read.
pub fn load_settings() -> Result<EditorTypographySettings, String> {
    let path = settings_path()?;
    
    if !path.exists() {
        return Ok(EditorTypographySettings::default());
    }
    
    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read settings from {}: {}", path.display(), e))?;
    
    let settings: EditorTypographySettings = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse settings JSON: {}", e))?;
    
    // Validate loaded settings
    let mut validated = settings;
    validated.validate();
    
    Ok(validated)
}
