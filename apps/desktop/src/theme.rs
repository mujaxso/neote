use iced::{Color, Theme};

/// Design system tokens for Qyzer IDE
#[derive(Debug, Clone, Copy)]
pub struct DesignTokens {
    // Spacing scale (in pixels)
    pub spacing_xxs: f32,
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub spacing_lg: f32,
    pub spacing_xl: f32,
    pub spacing_xxl: f32,
    
    // Border radius
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    
    // Border widths
    pub border_width: f32,
    pub border_width_thick: f32,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            spacing_xxs: 2.0,
            spacing_xs: 4.0,
            spacing_sm: 8.0,
            spacing_md: 12.0,
            spacing_lg: 16.0,
            spacing_xl: 24.0,
            spacing_xxl: 32.0,
            
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
            
            border_width: 1.0,
            border_width_thick: 2.0,
        }
    }
}

/// Helper to get current theme colors from app state
pub fn current_colors(theme: QyzerTheme) -> SemanticColors {
    theme.colors()
}

/// Semantic color roles for Qyzer IDE - Premium dark theme
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Background surfaces - Premium dark palette
    pub app_background: Color,           // #161821
    pub shell_background: Color,         // #1B1D27
    pub panel_background: Color,         // #1E2130
    pub elevated_panel_background: Color, // #232637
    pub editor_background: Color,        // #171923
    pub input_background: Color,         // #141722
    pub status_bar_background: Color,    // #1B1D27
    
    // Text colors
    pub text_primary: Color,             // #E6EAF2
    pub text_secondary: Color,           // #B7C0D1
    pub text_muted: Color,               // #8892A6
    pub text_faint: Color,               // #687086
    pub text_on_accent: Color,           // #FFFFFF
    
    // UI elements
    pub border: Color,                   // #2B3040
    pub divider: Color,                  // #2B3040
    pub accent: Color,                   // #4C6FFF
    pub accent_hover: Color,             // #5A7BFF
    pub accent_soft_background: Color,   // rgba(76, 111, 255, 0.16)
    
    // States
    pub hover_background: Color,         // #2A2E42
    pub active_background: Color,        // #2A2E42
    pub selected_background: Color,      // #2D3A73
    
    // Status colors
    pub success: Color,                  // #35C46B
    pub warning: Color,                  // #F0B24B
    pub error: Color,                    // #F05D6C
    pub info: Color,                     // #6EA8FF
    
    // Focus
    pub focus_ring: Color,               // rgba(92, 122, 255, 0.45)
    
    // Syntax highlighting colors - Premium dark theme palette
    pub syntax_comment: Color,           // #687086 - subtle, secondary
    pub syntax_string: Color,            // #35C46B - warm, natural green
    pub syntax_keyword: Color,           // #4C6FFF - elegant blue accent
    pub syntax_function: Color,          // #6EA8FF - soft cyan
    pub syntax_type: Color,              // #F0B24B - muted gold
    pub syntax_variable: Color,          // #E6EAF2 - primary text
    pub syntax_constant: Color,          // #D67FFF - soft magenta
    pub syntax_number: Color,            // #D67FFF - same as constant
    pub syntax_operator: Color,          // #B7C0D1 - quiet but visible
    pub syntax_punctuation: Color,       // #B7C0D1 - same as operator
    pub syntax_attribute: Color,         // #5A7BFF - lighter blue accent
    pub syntax_macro: Color,             // #6EA8FF - same as function
    pub syntax_builtin: Color,           // #F0B24B - same as type
    pub syntax_plain: Color,             // #E6EAF2 - primary text
}

/// Theme variants for Qyzer
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QyzerTheme {
    Dark,
    Light,
    System,
}

impl std::fmt::Display for QyzerTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QyzerTheme::Dark => write!(f, "Dark"),
            QyzerTheme::Light => write!(f, "Light"),
            QyzerTheme::System => write!(f, "System"),
        }
    }
}

impl QyzerTheme {
    /// Get the semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            QyzerTheme::Dark => SemanticColors::dark(),
            QyzerTheme::Light => SemanticColors::light(),
            QyzerTheme::System => {
                // For now, default to dark theme
                // In a real implementation, we'd detect system preference
                SemanticColors::dark()
            }
        }
    }
    
    /// Convert to iced::Theme
    pub fn to_iced_theme(&self) -> Theme {
        match self {
            QyzerTheme::Dark => Theme::Dark,
            QyzerTheme::Light => Theme::Light,
            QyzerTheme::System => {
                // Default to dark for system
                Theme::Dark
            }
        }
    }
    
    /// Get all available theme variants
    pub fn all() -> Vec<Self> {
        vec![QyzerTheme::System, QyzerTheme::Light, QyzerTheme::Dark]
    }
    
    /// Get display name for the theme
    pub fn display_name(&self) -> &'static str {
        match self {
            QyzerTheme::System => "System",
            QyzerTheme::Light => "Light",
            QyzerTheme::Dark => "Dark",
        }
    }
}

impl SemanticColors {
    /// Premium professional dark theme for Qyzer IDE
    pub fn dark() -> Self {
        Self {
            // Background surfaces - Clean, deep charcoal with subtle blue undertones
            app_background: Color::from_rgb(0.098, 0.110, 0.137),      // #191C23 - Deep slate
            shell_background: Color::from_rgb(0.118, 0.129, 0.157),    // #1E2128 - Slightly lighter
            panel_background: Color::from_rgb(0.137, 0.149, 0.176),    // #23262D - Elegant panel
            elevated_panel_background: Color::from_rgb(0.157, 0.169, 0.196), // #282B32 - Subtle elevation
            editor_background: Color::from_rgb(0.098, 0.110, 0.137),   // #191C23 - Matches app for seamless look
            input_background: Color::from_rgb(0.137, 0.149, 0.176),    // #23262D - Consistent with panels
            status_bar_background: Color::from_rgb(0.118, 0.129, 0.157), // #1E2128 - Matches shell
            
            // Text colors - Clean, high contrast but not harsh
            text_primary: Color::from_rgb(0.941, 0.949, 0.961),        // #F0F2F5 - Soft white
            text_secondary: Color::from_rgb(0.784, 0.800, 0.824),      // #C8CCD2 - Muted gray
            text_muted: Color::from_rgb(0.627, 0.647, 0.678),          // #A0A5AD - Softer gray
            text_faint: Color::from_rgb(0.471, 0.486, 0.518),          // #787C84 - Very subtle
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),            // #FFFFFF - Pure white
            
            // UI elements - Professional, subtle
            border: Color::from_rgb(0.196, 0.208, 0.235),              // #32353C - Very subtle border
            divider: Color::from_rgb(0.196, 0.208, 0.235),             // #32353C - Same as border
            accent: Color::from_rgb(0.329, 0.584, 0.988),              // #5495FC - Professional blue
            accent_hover: Color::from_rgb(0.400, 0.639, 1.0),          // #66A3FF - Brighter on hover
            accent_soft_background: Color::from_rgba(0.329, 0.584, 0.988, 0.12), // Subtle accent background
            
            // States - Subtle but noticeable
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.05),   // Very subtle white overlay
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),  // Slightly more active
            selected_background: Color::from_rgba(0.329, 0.584, 0.988, 0.15), // Accent tint for selection
            
            // Status colors - Professional, not too vibrant
            success: Color::from_rgb(0.298, 0.843, 0.596),             // #4CD798 - Clean green
            warning: Color::from_rgb(0.988, 0.729, 0.298),             // #FCBA4C - Warm amber
            error: Color::from_rgb(0.988, 0.447, 0.447),               // #FC7272 - Soft red
            info: Color::from_rgb(0.329, 0.584, 0.988),                // #5495FC - Matches accent
            
            // Focus - Very subtle
            focus_ring: Color::from_rgba(0.329, 0.584, 0.988, 0.25),   // Subtle focus ring
            
            // Syntax highlighting colors - Professional IDE palette
            syntax_comment: Color::from_rgb(0.627, 0.647, 0.678),      // #A0A5AD - Muted gray
            syntax_string: Color::from_rgb(0.298, 0.843, 0.596),       // #4CD798 - Success green
            syntax_keyword: Color::from_rgb(0.788, 0.435, 0.949),      // #C96FF2 - Elegant purple
            syntax_function: Color::from_rgb(0.329, 0.584, 0.988),     // #5495FC - Accent blue
            syntax_type: Color::from_rgb(0.988, 0.729, 0.298),         // #FCBA4C - Warning amber
            syntax_variable: Color::from_rgb(0.941, 0.949, 0.961),     // #F0F2F5 - Primary text
            syntax_constant: Color::from_rgb(0.949, 0.518, 0.659),     // #F284A8 - Soft pink
            syntax_number: Color::from_rgb(0.949, 0.518, 0.659),       // #F284A8 - Same as constant
            syntax_operator: Color::from_rgb(0.784, 0.800, 0.824),     // #C8CCD2 - Secondary text
            syntax_punctuation: Color::from_rgb(0.784, 0.800, 0.824),  // #C8CCD2 - Same as operator
            syntax_attribute: Color::from_rgb(0.788, 0.435, 0.949),    // #C96FF2 - Same as keyword
            syntax_macro: Color::from_rgb(0.329, 0.584, 0.988),        // #5495FC - Same as function
            syntax_builtin: Color::from_rgb(0.988, 0.729, 0.298),      // #FCBA4C - Same as type
            syntax_plain: Color::from_rgb(0.941, 0.949, 0.961),        // #F0F2F5 - Primary text
        }
    }
    
    /// Light theme semantic colors - Professional light theme
    pub fn light() -> Self {
        Self {
            // Background surfaces - Clean, light gray with blue undertones
            app_background: Color::from_rgb(0.961, 0.965, 0.973),      // #F5F6F8 - Very light gray
            shell_background: Color::from_rgb(0.941, 0.945, 0.953),    // #F0F1F3 - Slightly darker
            panel_background: Color::from_rgb(0.980, 0.980, 0.984),    // #FAFAFB - Almost white
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0), // #FFFFFF - Pure white
            editor_background: Color::from_rgb(1.0, 1.0, 1.0),         // #FFFFFF - Pure white for editor
            input_background: Color::from_rgb(0.980, 0.980, 0.984),    // #FAFAFB - Matches panel
            status_bar_background: Color::from_rgb(0.941, 0.945, 0.953), // #F0F1F3 - Matches shell
            
            // Text colors - Dark but not pure black
            text_primary: Color::from_rgb(0.157, 0.176, 0.208),        // #282D35 - Dark slate
            text_secondary: Color::from_rgb(0.392, 0.416, 0.471),      // #646A78 - Medium gray
            text_muted: Color::from_rgb(0.549, 0.569, 0.620),          // #8C919E - Lighter gray
            text_faint: Color::from_rgb(0.706, 0.718, 0.753),          // #B4B7C0 - Very light gray
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),            // #FFFFFF - Pure white
            
            // UI elements - Subtle borders
            border: Color::from_rgb(0.863, 0.871, 0.886),              // #DCDEE2 - Very light border
            divider: Color::from_rgb(0.863, 0.871, 0.886),             // #DCDEE2 - Same as border
            accent: Color::from_rgb(0.329, 0.584, 0.988),              // #5495FC - Same professional blue
            accent_hover: Color::from_rgb(0.259, 0.514, 0.929),        // #4283ED - Slightly darker
            accent_soft_background: Color::from_rgba(0.329, 0.584, 0.988, 0.08), // Very subtle
            
            // States - Subtle overlays
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.03),   // Very subtle black overlay
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.05),  // Slightly more
            selected_background: Color::from_rgba(0.329, 0.584, 0.988, 0.1), // Accent tint
            
            // Status colors - Professional, muted
            success: Color::from_rgb(0.180, 0.729, 0.490),             // #2EBA7D - Muted green
            warning: Color::from_rgb(0.949, 0.627, 0.180),             // #F2A02E - Muted amber
            error: Color::from_rgb(0.949, 0.376, 0.376),               // #F26060 - Muted red
            info: Color::from_rgb(0.329, 0.584, 0.988),                // #5495FC - Same accent
            
            // Focus - Subtle
            focus_ring: Color::from_rgba(0.329, 0.584, 0.988, 0.3),    // Subtle focus ring
            
            // Syntax highlighting colors - Professional light theme
            syntax_comment: Color::from_rgb(0.706, 0.718, 0.753),      // #B4B7C0 - Faint text
            syntax_string: Color::from_rgb(0.180, 0.729, 0.490),       // #2EBA7D - Success green
            syntax_keyword: Color::from_rgb(0.729, 0.337, 0.886),      // #BA56E2 - Elegant purple
            syntax_function: Color::from_rgb(0.329, 0.584, 0.988),     // #5495FC - Accent blue
            syntax_type: Color::from_rgb(0.949, 0.627, 0.180),         // #F2A02E - Warning amber
            syntax_variable: Color::from_rgb(0.157, 0.176, 0.208),     // #282D35 - Primary text
            syntax_constant: Color::from_rgb(0.886, 0.376, 0.549),     // #E2608C - Soft pink
            syntax_number: Color::from_rgb(0.886, 0.376, 0.549),       // #E2608C - Same as constant
            syntax_operator: Color::from_rgb(0.392, 0.416, 0.471),     // #646A78 - Secondary text
            syntax_punctuation: Color::from_rgb(0.392, 0.416, 0.471),  // #646A78 - Same as operator
            syntax_attribute: Color::from_rgb(0.729, 0.337, 0.886),    // #BA56E2 - Same as keyword
            syntax_macro: Color::from_rgb(0.329, 0.584, 0.988),        // #5495FC - Same as function
            syntax_builtin: Color::from_rgb(0.949, 0.627, 0.180),      // #F2A02E - Same as type
            syntax_plain: Color::from_rgb(0.157, 0.176, 0.208),        // #282D35 - Primary text
        }
    }
}
