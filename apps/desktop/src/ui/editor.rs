use iced::{
    widget::{button, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::app::Message;

pub fn header<'a>(active_file_path: Option<&'a String>, is_dirty: bool) -> Element<'a, Message> {
    row![
        text(
            active_file_path
                .map(|p| format!("Editing: {}", p))
                .unwrap_or_else(|| "No file selected".to_string())
        )
        .size(16),
        if is_dirty {
            text(" (modified)").size(16).style(iced::Color::from_rgb8(255, 165, 0))
        } else {
            text(" (saved)").size(16).style(iced::Color::from_rgb8(0, 128, 0))
        },
        button("Save").on_press(Message::SaveFile).padding(8),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

pub fn editor<'a>(editor_content: &'a str) -> Element<'a, Message> {
    scrollable(
        text_input("", editor_content)
            .on_input(Message::EditorContentChanged)
            .padding(10)
    )
    .height(Length::Fill)
    .into()
}
