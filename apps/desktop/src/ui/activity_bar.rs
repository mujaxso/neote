use iced::{Element, Length, widget::{button, column, container, text}};
use crate::message::Message;
use crate::state::{App, Activity};

pub fn activity_bar(app: &App) -> Element<'_, Message> {
    let activities = [
        (Activity::Explorer, "📁", "Explorer"),
        (Activity::Search, "🔍", "Search"),
        (Activity::Ai, "🤖", "AI"),
        (Activity::SourceControl, "🔄", "Git"),
        (Activity::Settings, "⚙️", "Settings"),
    ];
    
    let activity_buttons: Vec<Element<_>> = activities
        .iter()
        .map(|(activity, icon, _tooltip)| {
            let is_active = app.active_activity == *activity;
            
            let message = if *activity == Activity::Ai {
                Message::ToggleAiPanel
            } else {
                Message::ActivitySelected(*activity)
            };
            
            let button_style = if is_active {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            };
            
            let button = button(
                container(
                    text(*icon).size(18)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
            )
            .width(Length::Fill)
            .height(Length::Fixed(48.0))
            .on_press(message)
            .style(button_style);
            
            button.into()
        })
        .collect();
    
    container(
        column(activity_buttons)
            .spacing(0)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Box)
    .into()
}
