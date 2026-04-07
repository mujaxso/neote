use iced::{
    widget::{scrollable, column, container, row, text, text_editor},
    Element, Length, Font,
};

use crate::app::Message;

pub fn editor<'a>(editor_content: &'a iced::widget::text_editor::Content) -> Element<'a, Message> {
    // Count lines to show line numbers
    let content_text = editor_content.text();
    let line_count = content_text.lines().count().max(1);
    
    // Limit the number of lines to render to prevent performance issues
    let max_lines_to_render = 1000;
    let lines_to_render = line_count.min(max_lines_to_render);
    
    let line_numbers: Vec<Element<_>> = (1..=lines_to_render)
        .map(|i| {
            container(
                text(format!("{:>4}", i))
                    .size(14)
                    .font(Font::MONOSPACE)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 100, 100)))
            )
            .padding([0, 8, 0, 0])
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
            .into()
        })
        .collect();
    
    let line_numbers_column = column(line_numbers)
        .spacing(0)
        .width(Length::Fixed(60.0));
    
    // Create a text editor with proper height
    let editor = text_editor::TextEditor::new(editor_content)
        .on_action(Message::EditorContentChanged)
        .font(Font::MONOSPACE)
        .height(Length::Fill);
    
    // Create a scrollable container for the editor
    let editor_container = container(editor)
        .padding(16)
        .width(Length::Fill)
        .height(Length::Fill);
    
    let scrollable_editor = scrollable(editor_container)
        .height(Length::Fill);
    
    row![
        container(line_numbers_column)
            .style(iced::theme::Container::Box)
            .height(Length::Fill),
        scrollable_editor,
    ]
    .height(Length::Fill)
    .into()
}
