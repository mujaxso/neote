mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;
mod message;
mod state;
mod theme;
mod update;
mod view;
mod explorer;
mod settings;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    // Check if we're in a Wayland environment
    let wayland_display = std::env::var("WAYLAND_DISPLAY").is_ok();
    let xdg_session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    
    println!("WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY").ok());
    println!("XDG_SESSION_TYPE: {}", xdg_session_type);
    
    // If we're on Wayland and having file picker issues, suggest using X11
    if wayland_display || xdg_session_type == "wayland" {
        println!("Note: If file picker doesn't work, try running with: WINIT_UNIX_BACKEND=x11 cargo run --bin desktop");
    }
    
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            visible: true,
            position: iced::window::Position::Centered,
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        // Use JetBrains Mono Nerd Font as the default font for better programming experience with icons
        default_font: iced::font::Font::with_name("JetBrainsMono Nerd Font"),
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    };
    
    App::run(settings)
}
