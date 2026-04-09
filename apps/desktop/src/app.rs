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
        let font_commands = vec![
            // Try to load Noto Sans first
            iced::font::load(include_bytes!("../../assets/fonts/NotoSans-Regular.ttf"))
                .map(|_| Message::FontLoaded)
                .map_err(|_| Message::FontLoadFailed),
            // Try to load Noto Emoji for emoji icons
            iced::font::load(include_bytes!("../../assets/fonts/NotoEmoji-Regular.ttf"))
                .map(|_| Message::FontLoaded)
                .map_err(|_| Message::FontLoadFailed),
        ];
        
        (app, Command::batch(font_commands).then(|| command))
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
