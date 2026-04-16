use iced::{
    widget::{container, text},
    Element, Length,
};
use crate::message::Message;

pub fn terminal_panel<'a>() -> Element<'a, Message> {
    container(
        text("Terminal placeholder")
            .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
    )
    .center_y()
    .center_x()
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
