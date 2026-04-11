use crate::message::Message;
use crate::state::{App, Activity};
use iced::Command;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::ToggleAiPanel => {
            app.workbench_layout.toggle_auxiliary_sidebar();
            Command::none()
        }
        Message::ActivitySelected(activity) => {
            match activity {
                Activity::Primary(primary_view) => {
                    app.workbench_layout.set_active_primary_view(primary_view);
                }
                Activity::Auxiliary(auxiliary_view) => {
                    app.workbench_layout.set_auxiliary_view(auxiliary_view);
                }
            }
            Command::none()
        }
        Message::ActivityHovered(activity) => {
            app.workbench_layout.hovered_activity = activity;
            Command::none()
        }
        Message::WindowResized(width, height) => {
            app.window_width = width;
            app.window_height = height;
            app.update_layout_mode();
            Command::none()
        }
        _ => Command::none(),
    }
}
