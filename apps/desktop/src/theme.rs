use iced::{Color, Theme};

/// Design system tokens for Zaroxi Studio
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
pub fn current_colors(theme: ZaroxiTheme) -> SemanticColors {
    theme.colors()
}

/// Semantic color roles for Zaroxi Studio - Premium dark theme
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
    pub accent_soft: Color,              // rgba(76, 111, 255, 0.2)
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

/// Theme variants for Zaroxi
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ZaroxiTheme {
    Dark,
    Light,
    System,
}

impl std::fmt::Display for ZaroxiTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZaroxiTheme::Dark => write!(f, "Dark"),
            ZaroxiTheme::Light => write!(f, "Light"),
            ZaroxiTheme::System => write!(f, "System"),
        }
    }
}

impl ZaroxiTheme {
    /// Get the semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            ZaroxiTheme::Dark => SemanticColors::dark(),
            ZaroxiTheme::Light => SemanticColors::light(),
            ZaroxiTheme::System => {
                // For now, default to dark theme
                // In a real implementation, we'd detect system preference
                SemanticColors::dark()
            }
        }
    }
    
    /// Convert to iced::Theme
    pub fn to_iced_theme(&self) -> Theme {
        match self {
            ZaroxiTheme::Dark => Theme::Dark,
            ZaroxiTheme::Light => Theme::Light,
            ZaroxiTheme::System => {
                // Default to dark for system
                Theme::Dark
            }
        }
    }
    
    /// Get all available theme variants
    pub fn all() -> Vec<Self> {
        vec![ZaroxiTheme::System, ZaroxiTheme::Light, ZaroxiTheme::Dark]
    }
    
    /// Get display name for the theme
    pub fn display_name(&self) -> &'static str {
        match self {
            ZaroxiTheme::System => "System",
            ZaroxiTheme::Light => "Light",
            ZaroxiTheme::Dark => "Dark",
        }
    }
}

impl SemanticColors {
    /// Premium professional dark theme for Qyzer Studio
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
            accent_soft: Color::from_rgba(0.329, 0.584, 0.988, 0.2),   // #5495FC with 0.2 alpha
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
    
    /// Light theme semantic colors - Professional developer-focused light theme optimized for readability
    pub fn light() -> Self {
        Self {
            // Background surfaces - Soft, comfortable light gray for reduced eye strain
            app_background: Color::from_rgb(0.973, 0.976, 0.980),      // #F8F9FA - Very soft off-white
            shell_background: Color::from_rgb(0.961, 0.965, 0.969),    // #F5F6F7 - Slightly darker
            panel_background: Color::from_rgb(0.980, 0.982, 0.984),    // #FAFBFC - Almost white with blue tint
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0), // #FFFFFF - Pure white for elevation
            editor_background: Color::from_rgb(0.992, 0.992, 0.990),   // #FDFDFC - Very slightly warm off-white
            input_background: Color::from_rgb(1.0, 1.0, 1.0),          // #FFFFFF - Clean white for inputs
            status_bar_background: Color::from_rgb(0.961, 0.965, 0.969), // #F5F6F7 - Matches shell
            
            // Text colors - Professional, readable dark grays with better contrast
            text_primary: Color::from_rgb(0.18, 0.20, 0.23),           // #2E333A - Slightly darker for better contrast
            text_secondary: Color::from_rgb(0.40, 0.43, 0.47),         // #666E78 - Darker medium gray
            text_muted: Color::from_rgb(0.55, 0.58, 0.62),             // #8C949E - Darker soft gray
            text_faint: Color::from_rgb(0.70, 0.72, 0.75),             // #B3B8BF - Darker very light gray
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),            // #FFFFFF - Pure white
            
            // UI elements - Clean, subtle borders
            border: Color::from_rgb(0.88, 0.89, 0.91),                 // #E0E3E8 - Light border
            divider: Color::from_rgb(0.88, 0.89, 0.91),                // #E0E3E8 - Same as border
            accent: Color::from_rgb(0.06, 0.53, 0.98),                 // #1087FA - Professional blue
            accent_hover: Color::from_rgb(0.04, 0.47, 0.88),           // #0A78E0 - Slightly darker
            accent_soft: Color::from_rgba(0.06, 0.53, 0.98, 0.12),     // #1087FA with 0.12 alpha
            accent_soft_background: Color::from_rgba(0.06, 0.53, 0.98, 0.06), // Very subtle
            
            // States - Subtle but noticeable
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.04),   // Subtle black overlay
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.06),  // Slightly more
            selected_background: Color::from_rgba(0.06, 0.53, 0.98, 0.1), // Accent tint for selection
            
            // Status colors - Clear and professional
            success: Color::from_rgb(0.16, 0.65, 0.33),                // #29A652 - Clean green
            warning: Color::from_rgb(0.91, 0.58, 0.07),                // #E89412 - Warm amber
            error: Color::from_rgb(0.91, 0.26, 0.26),                  // #E84242 - Clear red
            info: Color::from_rgb(0.06, 0.53, 0.98),                   // #1087FA - Matches accent
            
            // Focus - Clear but not distracting
            focus_ring: Color::from_rgba(0.06, 0.53, 0.98, 0.25),      // Clear focus ring
            
            // Syntax highlighting colors - Professional IDE palette optimized for readability
            // High contrast, clear differentiation, comfortable for long sessions
            syntax_comment: Color::from_rgb(0.45, 0.52, 0.60),         // #738599 - Darker blue-gray for better contrast
            syntax_string: Color::from_rgb(0.75, 0.12, 0.12),          // #BF1F1F - Darker red for better contrast
            syntax_keyword: Color::from_rgb(0.62, 0.12, 0.68),         // #9E1FAD - Slightly darker purple
            syntax_function: Color::from_rgb(0.00, 0.40, 0.70),        // #0066B3 - Darker professional blue
            syntax_type: Color::from_rgb(0.80, 0.35, 0.00),            // #CC5900 - Darker warm orange
            syntax_variable: Color::from_rgb(0.20, 0.22, 0.25),        // #33383F - Primary text color (good contrast)
            syntax_constant: Color::from_rgb(0.62, 0.12, 0.68),        // #9E1FAD - Same as keyword for consistency
            syntax_number: Color::from_rgb(0.12, 0.58, 0.25),          // #1F9440 - Darker green for better contrast
            syntax_operator: Color::from_rgb(0.40, 0.43, 0.47),        // #666E78 - Darker for better contrast
            syntax_punctuation: Color::from_rgb(0.55, 0.58, 0.62),     // #8C949E - Darker muted text
            syntax_attribute: Color::from_rgb(0.62, 0.12, 0.68),       // #9E1FAD - Same as keyword
            syntax_macro: Color::from_rgb(0.80, 0.35, 0.00),           // #CC5900 - Same as type
            syntax_builtin: Color::from_rgb(0.00, 0.40, 0.70),         // #0066B3 - Same as function
            syntax_plain: Color::from_rgb(0.20, 0.22, 0.25),           // #33383F - Primary text color (not pure black)
        }
    }
}
