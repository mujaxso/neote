use crate::message::Message;
use crate::state::App;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::ThemeChanged(theme) => {
            app.theme_preference = theme;
            app.update_current_theme();
            app.status_message = format!("Theme changed to {}", theme.display_name());
            
            // Save settings
            match crate::settings::persistence::save_settings(&app.editor_typography, app.theme_preference) {
                Ok(_) => {
                    app.error_message = None;
                }
                Err(e) => {
                    app.error_message = Some(e);
                }
            }
            Command::none()
        }
        Message::FontFamilyChanged(font_family) => {
            app.editor_typography.font_family = font_family;
            app.status_message = format!("Font changed to {}", font_family.to_family_string());
            Command::none()
        }
        Message::FontSizeChanged(size) => {
            app.editor_typography.font_size = size;
            app.editor_typography.validate();
            app.status_message = format!("Font size changed to {}px", size);
            Command::none()
        }
        Message::LineHeightChanged(line_height) => {
            app.editor_typography.line_height = line_height;
            app.editor_typography.validate();
            app.status_message = format!("Line height changed to {:.1}", line_height);
            Command::none()
        }
        Message::LetterSpacingChanged(spacing) => {
            app.editor_typography.letter_spacing = spacing;
            app.editor_typography.validate();
            app.status_message = format!("Letter spacing changed to {:.1}px", spacing);
            Command::none()
        }
        Message::LigaturesToggled(enabled) => {
            app.editor_typography.ligatures_enabled = enabled;
            app.status_message = if enabled {
                "Ligatures enabled".to_string()
            } else {
                "Ligatures disabled".to_string()
            };
            Command::none()
        }
        Message::IconModeChanged(icon_mode) => {
            app.editor_typography.icon_mode = icon_mode;
            app.status_message = format!("Icon mode changed to {}", icon_mode);
            Command::none()
        }
        Message::PreferNerdFontsToggled(enabled) => {
            app.editor_typography.prefer_nerd_fonts = enabled;
            app.status_message = if enabled {
                "Prefer Nerd Fonts enabled".to_string()
            } else {
                "Prefer Nerd Fonts disabled".to_string()
            };
            Command::none()
        }
        Message::ZoomIn => {
            app.editor_typography.zoom_in();
            app.status_message = format!("Zoomed in to {}px", app.editor_typography.font_size);
            Command::none()
        }
        Message::ZoomOut => {
            app.editor_typography.zoom_out();
            app.status_message = format!("Zoomed out to {}px", app.editor_typography.font_size);
            Command::none()
        }
        Message::ResetZoom => {
            app.editor_typography.reset_zoom();
            app.status_message = format!("Zoom reset to {}px", app.editor_typography.font_size);
            Command::none()
        }
        Message::ResetTypographyToDefaults => {
            app.editor_typography.reset_to_defaults();
            app.status_message = "Typography reset to defaults".to_string();
            Command::none()
        }
        Message::SaveTypographySettings => {
            match crate::settings::persistence::save_settings(&app.editor_typography, app.theme_preference) {
                Ok(_) => {
                    app.status_message = "Settings saved".to_string();
                    app.error_message = None;
                }
                Err(e) => {
                    app.error_message = Some(e);
                    app.status_message = "Failed to save settings".to_string();
                }
            }
            Command::none()
        }
        Message::TypographySettingsLoaded(result) => {
            match result {
                Ok((typography, theme_preference)) => {
                    app.editor_typography = typography;
                    app.theme_preference = theme_preference;
                    app.update_current_theme();
                    app.status_message = "Settings loaded".to_string();
                    app.error_message = None;
                }
                Err(e) => {
                    app.error_message = Some(e);
                    app.status_message = "Failed to load settings".to_string();
                }
            }
            Command::none()
        }
        Message::FontLoaded => {
            Command::none()
        }
        Message::FontLoadFailed => {
            Command::none()
        }
        _ => Command::none(),
    }
}
