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
        
        // Try to load additional fonts for better icon support
        // We'll use system fonts, so we don't need to load font files
        // The font settings in main.rs should handle it
        (app, command)
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
