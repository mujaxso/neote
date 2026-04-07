use iced::{
    widget::{button, column, container, scrollable, text},
    Element, Length,
};

use crate::app::Message;

pub fn file_list(file_entries: &[crate::core_types::workspace::DirectoryEntry]) -> Element<Message> {
    if file_entries.is_empty() {
        container(text("No files found").size(16))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    } else {
        let items: Element<_> = scrollable(
            column(
                file_entries
                    .iter()
                    .enumerate()
                    .map(|(i, entry)| {
                        let label = if entry.is_dir {
                            format!("📁 {}", entry.name)
                        } else {
                            format!("📄 {}", entry.name)
                        };
                        button(text(label).size(14))
                            .on_press(Message::FileSelected(i))
                            .width(Length::Fill)
                            .padding(8)
                            .into()
                    })
                    .collect(),
            )
            .spacing(5),
        )
        .height(Length::Fill)
        .into();

        container(items).width(Length::Fill).height(Length::Fill).into()
    }
}
