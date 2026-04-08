use iced::{Element, Length, widget::{button, column, container, text}, Color};
use crate::message::Message;
use crate::state::{App, Activity};
use super::style::StyleHelpers;

pub fn activity_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
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
            
            // Active state styling
            let button_appearance = if is_active {
                iced::widget::button::Appearance {
                    background: Some(style.colors.accent_soft_background.into()),
                    border: iced::Border {
                        color: style.colors.accent,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    text_color: style.colors.accent,
                    ..Default::default()
                }
            } else {
                iced::widget::button::Appearance {
                    background: Some(Color::TRANSPARENT.into()),
                    border: iced::Border::default(),
                    text_color: style.colors.text_muted,
                    ..Default::default()
                }
            };
            
            let button = button(
                container(
                    text(*icon).size(16)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
            )
            .width(Length::Fill)
            .height(Length::Fixed(40.0)) // Compact height
            .on_press(message)
            .style(iced::theme::Button::Custom(Box::new(move |_| button_appearance)));
            
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
    .style(iced::theme::Container::Custom(Box::new(move |_theme| {
        container::Appearance {
            background: Some(style.colors.panel_background.into()),
            border: iced::Border {
                color: style.colors.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
