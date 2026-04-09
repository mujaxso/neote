use iced::{Element, Length, widget::{column, container}, Color};
use crate::message::Message;
use crate::state::{App, Activity};
use super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::{Icon, icon_button};

pub fn activity_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    let activities = [
        (Activity::Explorer, Icon::Folder, "Explorer"),
        (Activity::Search, Icon::Search, "Search"),
        (Activity::Ai, Icon::Robot, "AI"),
        (Activity::SourceControl, Icon::Git, "Git"),
        (Activity::Settings, Icon::Settings, "Settings"),
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
            
            // Create a custom style sheet for the button
            struct ActivityButtonStyle {
                is_active: bool,
                colors: SemanticColors,
            }
            
            impl iced::widget::button::StyleSheet for ActivityButtonStyle {
                type Style = iced::Theme;
                
                fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
                    if self.is_active {
                        iced::widget::button::Appearance {
                            background: Some(self.colors.accent_soft_background.into()),
                            border: iced::Border {
                                color: self.colors.accent,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            text_color: self.colors.accent,
                            ..Default::default()
                        }
                    } else {
                        iced::widget::button::Appearance {
                            background: Some(Color::TRANSPARENT.into()),
                            border: iced::Border::default(),
                            text_color: self.colors.text_muted,
                            ..Default::default()
                        }
                    }
                }
                
                fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
                    if self.is_active {
                        iced::widget::button::Appearance {
                            background: Some(self.colors.accent_soft_background.into()),
                            border: iced::Border {
                                color: self.colors.accent_hover,
                                width: 1.0,
                                radius: 0.0.into(),
                            },
                            text_color: self.colors.accent_hover,
                            ..Default::default()
                        }
                    } else {
                        iced::widget::button::Appearance {
                            background: Some(self.colors.hover_background.into()),
                            border: iced::Border::default(),
                            text_color: self.colors.text_primary,
                            ..Default::default()
                        }
                    }
                }
            }
            
            let button_style = ActivityButtonStyle {
                is_active,
                colors: style.colors,
            };
            
            // Use icon_button helper for consistent icon rendering
            let button = icon_button(
                *icon,
                &app.editor_typography,
                &style,
                Some(message),
                Some(16),
            )
            .width(Length::Fill)
            .height(Length::Fixed(40.0)) // Compact height
            .style(iced::theme::Button::Custom(Box::new(button_style)));
            
            button.into()
        })
        .collect();
    
    struct ActivityBarContainerStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for ActivityBarContainerStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.panel_background.into()),
                border: iced::Border {
                    color: self.colors.border,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let container_style = ActivityBarContainerStyle {
        colors: style.colors,
    };
    
    container(
        column(activity_buttons)
            .spacing(0)
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(container_style)))
    .into()
}
