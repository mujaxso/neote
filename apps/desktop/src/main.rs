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
    println!("Starting Neote...");
    
    // Don't force any backend - let winit choose the appropriate one
    // This should work with both X11 and Wayland
    
    // Increase memory limits for large files
    // This might help with scrolling crashes
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            visible: true, // Ensure window is visible
            position: iced::window::Position::Centered, // Center the window
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        // Use JetBrains Mono Nerd Font as the default font for better programming experience with icons
        default_font: iced::font::Font::with_name("JetBrainsMono Nerd Font"),
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    };
    
    println!("Running App::run...");
    App::run(settings)
}
