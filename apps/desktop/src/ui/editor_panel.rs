use iced::{Element, Length, widget::{column, container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        container(
            row![
                text("📄").size(12),
                text(file_name)
                    .size(12)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                iced::widget::horizontal_space(),
                if app.is_dirty {
                    text("●").size(10)
                        .style(iced::theme::Text::Color(style.colors.warning))
                } else {
                    text("✓").size(10)
                        .style(iced::theme::Text::Color(style.colors.success))
                }
            ]
            .spacing(6)
            .align_items(iced::Alignment::Center)
        )
        .padding([8, 12])
        .width(Length::Fill)
    } else {
        container(
            text("No file selected")
                .size(12)
                .style(iced::theme::Text::Color(style.colors.text_muted))
        )
        .padding([8, 12])
        .width(Length::Fill)
    };
    
    let editor_content = if let Some(_) = &app.active_file_path {
        // Use the existing editor component with typography settings
        editor::editor(&app.text_editor, &app.editor_typography)
    } else {
        // Welcome screen
        container(
            column![
                text("Neote").size(32)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                text("AI‑first IDE").size(16)
                    .style(iced::theme::Text::Color(style.colors.text_secondary)),
                container(iced::widget::horizontal_rule(1)).width(150),
                column![
                    text("Open a file from the explorer")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                    text("Ask AI about the workspace")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                ]
                .spacing(8)
                .padding(16),
            ]
            .spacing(16)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };
    
    container(
        column![
            header,
            container(editor_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(style.colors.editor_background.into()),
                        border: iced::Border::default(),
                        ..Default::default()
                    }
                }))),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
        container::Appearance {
            background: Some(style.colors.panel_background.into()),
            border: iced::Border {
                color: style.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
