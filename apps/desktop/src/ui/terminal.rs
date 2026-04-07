use iced::{
    widget::{column, container, row, scrollable, text, text_input, button},
    Element, Length, theme,
};
use crate::app::Message;

pub fn terminal<'a>(_content: &'a str) -> Element<'a, Message> {
    column![
        row![
            text("TERMINAL").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
            iced::widget::horizontal_space(),
            button("Clear")
                .on_press(Message::PromptInputChanged("clear".to_string()))
                .style(theme::Button::Secondary),
            button("Run")
                .on_press(Message::PromptInputChanged("cargo run".to_string()))
                .style(theme::Button::Primary),
        ]
        .padding([8, 16])
        .align_items(iced::Alignment::Center),
        iced::widget::horizontal_rule(1),
        container(
            scrollable(
                text("$ cargo build\n   Compiling neote v0.1.0\n    Finished dev [unoptimized + debuginfo] target(s) in 0.98s")
                    .font(iced::Font::MONOSPACE)
                    .size(14)
            )
            .height(Length::Fill)
        )
        .padding(16)
        .height(Length::Fill),
        iced::widget::horizontal_rule(1),
        row![
            text("❯").size(14).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 0))),
            text_input("Type a command...", "")
                .on_input(|cmd| Message::PromptInputChanged(format!("terminal: {}", cmd)))
                .padding(8)
                .width(Length::Fill),
        ]
        .padding([8, 16])
        .align_items(iced::Alignment::Center),
    ]
    .height(Length::Fill)
    .into()
}
