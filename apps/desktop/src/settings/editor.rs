//! Editor typography settings model.
//!
//! Defines the structure and validation rules for editor font settings,
//! including font family, size, line height, and ligature support.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Available monospace font families optimized for coding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontFamily {
    JetBrainsMono,
    FiraCode,
    CascadiaCode,
    Iosevka,
    SourceCodePro,
}

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontFamily::JetBrainsMono => write!(f, "JetBrains Mono"),
            FontFamily::FiraCode => write!(f, "Fira Code"),
            FontFamily::CascadiaCode => write!(f, "Cascadia Code"),
            FontFamily::Iosevka => write!(f, "Iosevka"),
            FontFamily::SourceCodePro => write!(f, "Source Code Pro"),
        }
    }
}

impl FontFamily {
    /// Get the CSS/iced font family string for this font.
    pub fn to_family_string(&self) -> &'static str {
        match self {
            FontFamily::JetBrainsMono => "JetBrains Mono",
            FontFamily::FiraCode => "Fira Code",
            FontFamily::CascadiaCode => "Cascadia Code",
            FontFamily::Iosevka => "Iosevka",
            FontFamily::SourceCodePro => "Source Code Pro",
        }
    }

    /// Get the fallback font stack for this font family.
    pub fn fallback_stack(&self) -> Vec<&'static str> {
        match self {
            FontFamily::JetBrainsMono => vec![
                "JetBrains Mono",
                "Fira Code",
                "Cascadia Code",
                "monospace",
            ],
            FontFamily::FiraCode => vec![
                "Fira Code",
                "JetBrains Mono",
                "Cascadia Code",
                "monospace",
            ],
            FontFamily::CascadiaCode => vec![
                "Cascadia Code",
                "JetBrains Mono",
                "Fira Code",
                "monospace",
            ],
            FontFamily::Iosevka => vec![
                "Iosevka",
                "JetBrains Mono",
                "Fira Code",
                "monospace",
            ],
            FontFamily::SourceCodePro => vec![
                "Source Code Pro",
                "JetBrains Mono",
                "Fira Code",
                "monospace",
            ],
        }
    }

    /// Get all available font families.
    pub fn all() -> Vec<FontFamily> {
        vec![
            FontFamily::JetBrainsMono,
            FontFamily::FiraCode,
            FontFamily::CascadiaCode,
            FontFamily::Iosevka,
            FontFamily::SourceCodePro,
        ]
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::JetBrainsMono
    }
}

/// Editor typography settings optimized for coding readability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorTypographySettings {
    /// Selected font family
    pub font_family: FontFamily,
    /// Font size in pixels
    pub font_size: u16,
    /// Line height multiplier (e.g., 1.5)
    pub line_height: f32,
    /// Whether ligatures are enabled (for fonts that support them)
    pub ligatures_enabled: bool,
    /// Letter spacing in pixels (can be negative or positive)
    pub letter_spacing: f32,
}

impl Default for EditorTypographySettings {
    fn default() -> Self {
        Self {
            font_family: FontFamily::default(),
            font_size: 14, // Optimal for coding readability
            line_height: 1.6, // Balanced for scanning code
            ligatures_enabled: false, // Off by default for clarity
            letter_spacing: 0.0, // Monospace fonts typically don't need extra spacing
        }
    }
}

impl EditorTypographySettings {
    /// Create new settings with validation
    pub fn new(
        font_family: FontFamily,
        font_size: u16,
        line_height: f32,
        ligatures_enabled: bool,
        letter_spacing: f32,
    ) -> Self {
        let font_size = font_size.clamp(10, 24);
        let line_height = line_height.clamp(1.2, 2.0);
        let letter_spacing = letter_spacing.clamp(-0.2, 0.2);
        
        Self {
            font_family,
            font_size,
            line_height,
            ligatures_enabled,
            letter_spacing,
        }
    }

    /// Increase font size (zoom in)
    pub fn zoom_in(&mut self) {
        self.font_size = (self.font_size + 1).clamp(10, 24);
    }

    /// Decrease font size (zoom out)
    pub fn zoom_out(&mut self) {
        self.font_size = (self.font_size.saturating_sub(1)).clamp(10, 24);
    }

    /// Reset font size to default
    pub fn reset_zoom(&mut self) {
        self.font_size = 14;
    }

    /// Reset all settings to defaults
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Validate settings are within reasonable bounds
    pub fn validate(&mut self) {
        self.font_size = self.font_size.clamp(10, 24);
        self.line_height = self.line_height.clamp(1.2, 2.0);
        self.letter_spacing = self.letter_spacing.clamp(-0.2, 0.2);
    }

    /// Get the effective line height in pixels
    pub fn line_height_pixels(&self) -> f32 {
        self.font_size as f32 * self.line_height
    }
}
