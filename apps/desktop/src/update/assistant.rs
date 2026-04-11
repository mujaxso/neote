use crate::message::Message;
use crate::state::App;
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::PromptInputChanged(text) => {
            app.prompt_input = text;
            Command::none()
        }
        Message::SendPrompt => {
            // Placeholder for AI prompt
            app.status_message = "AI feature coming soon".to_string();
            app.prompt_input.clear();
            Command::none()
        }
        _ => Command::none(),
    }
}
