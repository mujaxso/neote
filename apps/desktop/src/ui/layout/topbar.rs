use iced::{
    widget::{container, horizontal_space, row, text},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::theme::ZaroxiTheme;
use crate::ui::style::StyleHelpers;

pub fn top_bar<'a>(workspace_path: &'a str, theme: ZaroxiTheme) -> Element<'a, Message> {
    // Create style helpers to get theme colors
    let style = StyleHelpers::new(theme);
    
    container(
        row![
            // Branding - minimal and elegant
            container(
                row![
                    text("Z").size(18).style(iced::theme::Text::Color(style.colors.accent)),
                    text("aroxi Studio").size(14).style(iced::theme::Text::Color(style.colors.text_primary)),
                ]
                .spacing(2)
                .align_items(Alignment::Center)
            )
            .padding([0, 16]),
            
            // Subtle divider
            container(iced::widget::Space::with_width(1.0))
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(style.colors.border.into()),
                        ..Default::default()
                    }
                })))
                .height(Length::Fixed(20.0))
                .width(Length::Fixed(1.0)),
            
            // Workspace path display - clean and non-interactive
            if !workspace_path.is_empty() {
                Element::from(container(
                    text(workspace_path)
                        .size(13)
                        .style(iced::theme::Text::Color(style.colors.text_secondary))
                )
                .padding([8, 12]))
            } else {
                Element::from(container(
                    text("No workspace open")
                        .size(13)
                        .style(iced::theme::Text::Color(style.colors.text_muted))
                )
                .padding([8, 12]))
            },
            
            horizontal_space(),
            
            // Right side reserved for potential window controls or other premium IDE elements
            // Currently kept empty for a clean, professional look
        ]
        .align_items(Alignment::Center)
    )
    .padding([8, 16])
    .width(Length::Fill)
    .height(Length::Fixed(40.0)) // More compact height
    .style(iced::theme::Container::Custom(Box::new(TopBarContainerStyle { style })))
    .into()
}

struct TopBarContainerStyle {
    style: StyleHelpers,
}

impl iced::widget::container::StyleSheet for TopBarContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        container::Appearance {
            background: Some(self.style.colors.panel_background.into()),
            border: iced::Border {
                color: self.style.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}
