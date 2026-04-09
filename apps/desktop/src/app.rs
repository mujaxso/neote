// Re-export the main types from the crate root modules
pub use crate::message::Message;
pub use crate::state::App;
pub use crate::update::update;
pub use crate::view::view;

use iced::{Element, Command};

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (app, command) = App::new();
        
        // Load custom fonts for icon support
        // We'll try to load multiple fonts to ensure icons are visible
        let mut font_commands = Vec::new();
        
        // Try to load programming fonts from various possible locations
        let possible_font_dirs = [
            "apps/desktop/assets/fonts",
            "assets/fonts",
            "../assets/fonts",
        ];
        
        // Fonts to load in order of preference
        let font_files = [
            ("JetBrainsMono-Regular.ttf", "JetBrains Mono"),
            ("FiraCode-Regular.ttf", "Fira Code"),
            ("NotoColorEmoji.ttf", "Noto Color Emoji"),
        ];
        
        for dir in &possible_font_dirs {
            for (file, _name) in &font_files {
                let path = format!("{}/{}", dir, file);
                if std::path::Path::new(&path).exists() {
                    if let Ok(bytes) = std::fs::read(&path) {
                        font_commands.push(
                            iced::font::load(bytes)
                                .map(|_| Message::FontLoaded)
                        );
                    }
                }
            }
        }
        
        // If no fonts found in standard locations, try current directory
        if font_commands.is_empty() {
            for (file, _name) in &font_files {
                if std::path::Path::new(file).exists() {
                    if let Ok(bytes) = std::fs::read(file) {
                        font_commands.push(
                            iced::font::load(bytes)
                                .map(|_| Message::FontLoaded)
                        );
                    }
                }
            }
        }
        
        // If no fonts were loaded, we'll just use system fonts
            (app, command)
        } else {
            // Combine font loading commands with the initial app command
            let mut all_commands = font_commands;
            all_commands.push(command);
            (app, Command::batch(all_commands))
        }
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn theme(&self) -> iced::Theme {
        self.theme.to_iced_theme()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.subscription()
    }
}
