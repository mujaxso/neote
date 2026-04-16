use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::ui::icons::Icon;
use crate::settings::editor::EditorTypographySettings;

pub fn ai_panel<'a>(prompt_input: &'a str) -> Element<'a, Message> {
    column![
        container(
            row![
                text("AI ASSISTANT").size(11)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                horizontal_space(),
                button("⋯")
                    .on_press(Message::PromptInputChanged("AI options".to_string()))
                    .padding([6, 8])
                    .style(iced::theme::Button::Secondary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([14, 16])
        .width(Length::Fill),
        iced::widget::horizontal_rule(1),
        scrollable(
            column![
                // Welcome card - More solid for IDE
                {
                    struct WelcomeCardStyle;
                    impl iced::widget::container::StyleSheet for WelcomeCardStyle {
                        type Style = iced::Theme;
                
                        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
                            iced::widget::container::Appearance {
                                background: Some(iced::Color::from_rgb(0.12, 0.12, 0.16).into()),
                                border: iced::Border {
                                    color: iced::Color::from_rgb(0.25, 0.45, 0.85),
                                    width: 1.0,
                                    radius: 8.0.into(),
                                },
                                shadow: Default::default(),
                                text_color: None,
                            }
                        }
                    }
                    container(
                        column![
                            row![
                                Icon::Robot.render_with_color(
                                    &EditorTypographySettings::default(),
                                    iced::Color::from_rgb8(220, 220, 255),
                                    Some(20)
                                ),
                                text("Qyzer Studio AI").size(16).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 255))),
                            ]
                            .spacing(8)
                            .align_items(Alignment::Center),
                            text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                                .size(13)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 190, 210))),
                        ]
                        .spacing(10)
                        .padding(20)
                    )
                    .style(iced::theme::Container::Custom(Box::new(WelcomeCardStyle)))
                },
                // Quick actions
                container(
                    column![
                        text("Quick Actions").size(13)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                        column![
                            button("Explain this file")
                                .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Refactor selection")
                                .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Find bugs")
                                .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Write tests")
                                .on_press(Message::PromptInputChanged("Write unit tests for this code".to_string()))
                                .padding([10, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                        ]
                        .spacing(6),
                    ]
                    .spacing(12)
                    .padding(16)
                )
                .style(iced::theme::Container::Box),
                // Info note
                container(
                    text("AI features are coming soon. This is a placeholder for the AI assistant interface.")
                        .size(11)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 160)))
                )
                .padding(16),
            ]
            .spacing(16)
            .padding([0, 16])
        )
        .height(Length::Fill),
        iced::widget::horizontal_rule(1),
        container(
            row![
                text_input("Ask Qyzer Studio AI...", prompt_input)
                    .on_input(Message::PromptInputChanged)
                    .padding([12, 14])
                    .width(Length::Fill),
                button("Send")
                    .on_press(Message::SendPrompt)
                    .padding([12, 18])
                    .style(iced::theme::Button::Primary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill),
    ]
    .height(Length::Fill)
    .into()
}

// Also provide assistant_panel function for compatibility
pub fn assistant_panel(app: &crate::state::App) -> Element<'_, Message> {
    // Use the same implementation as ai_panel but with app context
    ai_panel(&app.prompt_input)
}
