use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::app::Message;

pub fn layout(
    workspace_path: &str,
    file_entries: &[crate::core_types::workspace::DirectoryEntry],
    active_file_path: Option<&String>,
    editor_content: &str,
    is_dirty: bool,
    status_message: &str,
    error_message: Option<&String>,
) -> Element<Message> {
    let workspace_controls = row![
        text_input("Workspace path", workspace_path)
            .on_input(Message::WorkspacePathChanged)
            .padding(10)
            .width(Length::FillPortion(3)),
        button("Open Workspace")
            .on_press(Message::OpenWorkspace)
            .padding(10),
        button("Refresh")
            .on_press(Message::RefreshWorkspace)
            .padding(10),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let file_list = super::sidebar::file_list(file_entries);

    let editor_header = super::editor::header(active_file_path, is_dirty);
    let editor = super::editor::editor(editor_content);

    let status_bar = row![
        text(status_message).size(14),
        if let Some(err) = error_message {
            text(format!(" | Error: {}", err)).size(14).style(iced::Color::from_rgb8(255, 0, 0))
        } else {
            text("").size(14)
        }
    ]
    .spacing(10);

    let content = column![
        workspace_controls,
        iced::widget::horizontal_rule(1),
        row![
            container(file_list).width(Length::FillPortion(2)),
            container(
                column![
                    editor_header,
                    iced::widget::horizontal_rule(1),
                    editor,
                ]
                .spacing(10)
            )
            .width(Length::FillPortion(5))
            .padding(10),
        ]
        .height(Length::Fill),
        iced::widget::horizontal_rule(1),
        status_bar,
    ]
    .spacing(10)
    .padding(10)
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
