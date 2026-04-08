use iced::{
    widget::{container, text_editor},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a iced::widget::text_editor::Content) -> Element<'a, Message> {
    // Create a text editor with its own built-in scrolling
    // The text_editor widget handles scrolling internally, so we should NOT wrap it
    // in an outer scrollable container to avoid conflicts that cause crashes
    let editor = text_editor::TextEditor::new(editor_content)
        .on_action(Message::EditorContentChanged)
        .font(Font::MONOSPACE)
        .height(Length::Fill);
    
    // Place the editor in a container with padding
    // The text_editor widget will handle its own scrolling
    container(editor)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
