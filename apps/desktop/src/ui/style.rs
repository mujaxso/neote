use iced::{Color, Vector, widget::{button, container, text, text_input}};
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
            shadow_offset: iced::Vector::default(),
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
            shadow_offset: iced::Vector::default(),
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
