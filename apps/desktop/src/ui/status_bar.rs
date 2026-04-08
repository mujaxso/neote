use iced::{Element, Length, widget::{container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn status_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let left_status = if let Some(error) = &app.error_message {
        row![
            text("⚠").size(12).style(iced::theme::Text::Color(style.colors.error)),
            text(error).size(11).style(iced::theme::Text::Color(style.colors.error)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            text("✓").size(12).style(iced::theme::Text::Color(style.colors.success)),
            text(&app.status_message).size(11).style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    };
    
    let center_status = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        row![
            text("📄").size(12),
            text(file_name).size(11).style(iced::theme::Text::Color(style.colors.text_muted)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            text("No file open").size(11).style(iced::theme::Text::Color(style.colors.text_muted)),
        ]
        .align_items(iced::Alignment::Center)
    };
    
    let right_status = row![
        text(format!("{} files", app.file_entries.len())).size(11).style(iced::theme::Text::Color(style.colors.text_muted)),
        text("Ln 1, Col 1").size(11).style(iced::theme::Text::Color(style.colors.text_muted)),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center);
    
    container(
        row![
            container(left_status).padding([0, 12]),
            iced::widget::horizontal_space(),
            container(center_status),
            iced::widget::horizontal_space(),
            container(right_status).padding([0, 12]),
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Box)
    .into()
}
