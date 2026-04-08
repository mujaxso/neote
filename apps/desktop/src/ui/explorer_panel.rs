use iced::{Element, Length, widget::{button, column, container, row, scrollable, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn explorer_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = container(
        row![
            text("EXPLORER")
                .size(11)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            button(
                text("⟳").size(14)
            )
            .on_press(Message::RefreshWorkspace)
            .padding([4, 8])
            .style(iced::theme::Button::Secondary)
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([12, 16])
    .width(Length::Fill);
    
    let content: Element<_> = if app.file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(iced::theme::Button::Secondary)
            ]
            .spacing(12)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        let entries: Vec<Element<_>> = app.file_entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let is_selected = app.active_file_path.as_ref() == Some(&entry.path);
                
                let icon = if entry.is_dir { "📁" } else { "📄" };
                let text_color = if is_selected {
                    style.colors.text_on_accent
                } else if entry.is_dir {
                    style.colors.text_primary
                } else {
                    style.colors.text_secondary
                };
                
                let row_content = row![
                    text(icon).size(14),
                    text(&entry.name)
                        .size(13)
                        .style(iced::theme::Text::Color(text_color)),
                ]
                .spacing(8)
                .align_items(iced::Alignment::Center);
                
                let message = if entry.is_dir {
                    Message::ToggleDirectory(entry.path.clone())
                } else {
                    Message::FileSelected(idx)
                };
                
                let button_style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                };
                
                container(
                    button(row_content)
                        .on_press(message)
                        .padding([6, 12])
                        .width(Length::Fill)
                        .style(button_style)
                )
                .into()
            })
            .collect();
        
        scrollable(
            column(entries)
                .spacing(0)
                .width(Length::Fill)
        )
        .height(Length::Fill)
        .into()
    };
    
    container(
        column![
            header,
            container(content)
                .height(Length::Fill)
                .width(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Box)
    .into()
}
