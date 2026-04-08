use iced::{Element, Length, widget::container};
use crate::message::Message;
use crate::state::App;
use super::{
    activity_bar::activity_bar,
    assistant_panel::assistant_panel,
    editor_panel::editor_panel,
    explorer_panel::explorer_panel,
    status_bar::status_bar,
    top_bar::top_bar,
};

/// Main shell that composes all UI components
pub fn shell(app: &App) -> Element<'_, Message> {
    // Determine if AI panel should be visible
    let ai_panel_visible = matches!(app.active_activity, crate::state::Activity::Ai) || app.ai_panel_visible;
    
    // Build panels
    let top_bar = container(top_bar(app))
        .width(Length::Fill)
        .height(Length::Fixed(48.0));
    
    let activity_bar = container(activity_bar(app))
        .width(Length::Fixed(48.0))
        .height(Length::Fill);
    
    let explorer_panel = container(explorer_panel(app))
        .width(Length::Fixed(280.0))
        .height(Length::Fill);
    
    let editor_panel = container(editor_panel(app))
        .width(Length::Fill)
        .height(Length::Fill);
    
    // Conditionally include AI panel
    let main_content = if ai_panel_visible {
        let assistant_panel = container(assistant_panel(app))
            .width(Length::Fixed(320.0))
            .height(Length::Fill);
        
        iced::widget::row![
            activity_bar,
            explorer_panel,
            editor_panel,
            assistant_panel,
        ]
        .height(Length::Fill)
    } else {
        iced::widget::row![
            activity_bar,
            explorer_panel,
            editor_panel,
        ]
        .height(Length::Fill)
    };
    
    let status_bar = container(status_bar(app))
        .width(Length::Fill)
        .height(Length::Fixed(28.0));
    
    // Combine everything
    iced::widget::column![
        top_bar,
        main_content,
        status_bar,
    ]
    .height(Length::Fill)
    .into()
}
