use iced::{Color, Theme};
use iced::widget::{button, container, text, text_input};
use iced::theme::{self as iced_theme, Button, Container, TextInput};

use crate::theme::{current_colors, NeoteTheme, SemanticColors};

/// Get current theme colors from app state
pub fn colors(theme: NeoteTheme) -> SemanticColors {
    current_colors(theme)
}

/// Style helpers for UI components
pub struct StyleHelpers {
    pub colors: SemanticColors,
    pub tokens: crate::theme::DesignTokens,
}

impl StyleHelpers {
    pub fn new(theme: NeoteTheme) -> Self {
        Self {
            colors: colors(theme),
            tokens: crate::theme::DesignTokens::default(),
        }
    }
    
    /// Panel container style
    pub fn panel_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.panel_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: self.tokens.border_width,
                radius: self.tokens.radius_sm.into(),
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
    
    /// Elevated panel container style
    pub fn elevated_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.elevated_panel_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: self.tokens.border_width,
                radius: self.tokens.radius_md.into(),
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
    
    /// Status bar container style
    pub fn status_bar_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.status_bar_background.into()),
            border: iced::Border {
                color: self.colors.divider,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
    
    /// Primary button style
    pub fn primary_button(&self) -> button::Appearance {
        button::Appearance {
            background: Some(self.colors.accent.into()),
            border: iced::Border {
                color: self.colors.accent,
                width: self.tokens.border_width,
                radius: self.tokens.radius_sm.into(),
            },
            text_color: self.colors.text_on_accent,
            shadow: Default::default(),
        }
    }
    
    /// Secondary button style
    pub fn secondary_button(&self) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border {
                color: self.colors.border,
                width: self.tokens.border_width,
                radius: self.tokens.radius_sm.into(),
            },
            text_color: self.colors.text_secondary,
            shadow: Default::default(),
        }
    }
    
    /// Text input style
    pub fn text_input(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: self.colors.elevated_panel_background.into(),
            border: iced::Border {
                color: self.colors.border,
                width: self.tokens.border_width,
                radius: self.tokens.radius_sm.into(),
            },
            icon_color: self.colors.text_muted,
        }
    }
    
    /// Primary text style
    pub fn text_primary(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.text_primary),
        }
    }
    
    /// Secondary text style
    pub fn text_secondary(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.text_secondary),
        }
    }
    
    /// Muted text style
    pub fn text_muted(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.text_muted),
        }
    }
    
    /// Success text style
    pub fn text_success(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.success),
        }
    }
    
    /// Warning text style
    pub fn text_warning(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.warning),
        }
    }
    
    /// Error text style
    pub fn text_error(&self) -> text::Appearance {
        text::Appearance {
            color: Some(self.colors.error),
        }
    }
}

/// Custom theme implementations
pub mod custom_theme {
    use iced::theme::{Button, Container, TextInput};
    
    use super::StyleHelpers;
    use crate::theme::NeoteTheme;
    
    /// Create a custom button style
    pub fn primary_button(theme: NeoteTheme) -> Button {
        let helpers = StyleHelpers::new(theme);
        Button::Custom(Box::new(move |_theme| helpers.primary_button()))
    }
    
    pub fn secondary_button(theme: NeoteTheme) -> Button {
        let helpers = StyleHelpers::new(theme);
        Button::Custom(Box::new(move |_theme| helpers.secondary_button()))
    }
    
    /// Create a custom container style
    pub fn panel_container(theme: NeoteTheme) -> Container {
        let helpers = StyleHelpers::new(theme);
        Container::Custom(Box::new(move |_theme| helpers.panel_container()))
    }
    
    pub fn elevated_container(theme: NeoteTheme) -> Container {
        let helpers = StyleHelpers::new(theme);
        Container::Custom(Box::new(move |_theme| helpers.elevated_container()))
    }
    
    pub fn status_bar_container(theme: NeoteTheme) -> Container {
        let helpers = StyleHelpers::new(theme);
        Container::Custom(Box::new(move |_theme| helpers.status_bar_container()))
    }
    
    /// Create a custom text input style
    pub fn custom_text_input(theme: NeoteTheme) -> TextInput {
        let helpers = StyleHelpers::new(theme);
        TextInput::Custom(Box::new(move |_theme| helpers.text_input()))
    }
}
