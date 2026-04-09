//! Semantic icon system for Neote IDE.
//!
//! Provides a clean, maintainable way to use icons throughout the UI,
//! with support for developer glyphs, Nerd Fonts, and graceful fallbacks.

use iced::widget::text;
use iced::{Color, Element};

use crate::settings::editor::{EditorTypographySettings, IconMode};
use crate::ui::style::StyleHelpers;

/// Semantic icon identifiers for the IDE.
/// These represent UI concepts rather than specific glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    // File system
    File,
    Folder,
    FolderOpen,
    
    // Navigation
    ChevronRight,
    ChevronDown,
    ArrowRight,
    ArrowLeft,
    
    // Actions
    Search,
    Settings,
    Refresh,
    Add,
    Edit,
    Delete,
    Save,
    Close,
    
    // Version control
    Git,
    GitBranch,
    GitCommit,
    GitPullRequest,
    
    // Development
    Terminal,
    Debug,
    Run,
    Build,
    
    // Status
    Success,
    Warning,
    Error,
    Info,
    
    // AI/Assistant
    Robot,
    Sparkles,
    
    // UI elements
    Menu,
    MoreHorizontal,
    MoreVertical,
}

impl Icon {
    /// Get the Unicode fallback character for this icon.
    /// Used when Nerd Fonts are not available.
    pub fn unicode_fallback(&self) -> &'static str {
        match self {
            // File system
            Icon::File => "📄",
            Icon::Folder => "📁",
            Icon::FolderOpen => "📂",
            
            // Navigation
            Icon::ChevronRight => "›",
            Icon::ChevronDown => "⌄",
            Icon::ArrowRight => "→",
            Icon::ArrowLeft => "←",
            
            // Actions
            Icon::Search => "🔍",
            Icon::Settings => "⚙",
            Icon::Refresh => "↻",
            Icon::Add => "+",
            Icon::Edit => "✎",
            Icon::Delete => "🗑",
            Icon::Save => "💾",
            Icon::Close => "×",
            
            // Version control
            Icon::Git => "🔄",
            Icon::GitBranch => "",
            Icon::GitCommit => "●",
            Icon::GitPullRequest => "",
            
            // Development
            Icon::Terminal => ">_",
            Icon::Debug => "🐛",
            Icon::Run => "▶",
            Icon::Build => "🔨",
            
            // Status
            Icon::Success => "✓",
            Icon::Warning => "⚠",
            Icon::Error => "✗",
            Icon::Info => "ℹ",
            
            // AI/Assistant
            Icon::Robot => "🤖",
            Icon::Sparkles => "✨",
            
            // UI elements
            Icon::Menu => "☰",
            Icon::MoreHorizontal => "⋯",
            Icon::MoreVertical => "⋮",
        }
    }

    /// Get the Nerd Font glyph for this icon.
    /// Returns the appropriate Unicode code point for Nerd Fonts.
    pub fn nerd_font_glyph(&self) -> &'static str {
        match self {
            // File system
            Icon::File => "󰈙",        // nf-md-file_document (updated)
            Icon::Folder => "󰉋",      // nf-md-folder (updated)
            Icon::FolderOpen => "󰉌",  // nf-md-folder_open (updated)
            
            // Navigation
            Icon::ChevronRight => "", // nf-fa-chevron_right
            Icon::ChevronDown => "",  // nf-fa-chevron_down
            Icon::ArrowRight => "",   // nf-fa-arrow_right
            Icon::ArrowLeft => "",    // nf-fa-arrow_left
            
            // Actions
            Icon::Search => "󰍉",       // nf-md-magnify (updated)
            Icon::Settings => "󰒓",     // nf-md-cog (updated)
            Icon::Refresh => "󰑐",      // nf-md-refresh (updated)
            Icon::Add => "󰅻",          // nf-md-plus (updated)
            Icon::Edit => "󰏫",         // nf-md-pencil (updated)
            Icon::Delete => "󰅙",       // nf-md-close (updated)
            Icon::Save => "󰆓",         // nf-md-content_save (updated)
            Icon::Close => "󰅖",        // nf-md-close (updated)
            
            // Version control
            Icon::Git => "󰊢",         // nf-md-git (updated)
            Icon::GitBranch => "󰘬",   // nf-md-source_branch (updated)
            Icon::GitCommit => "󰡚",   // nf-md-source_commit (updated)
            Icon::GitPullRequest => "󰤙", // nf-md-source_pull (updated)
            
            // Development
            Icon::Terminal => "󰆍",     // nf-md-terminal (updated)
            Icon::Debug => "󰚧",       // nf-md-bug (updated)
            Icon::Run => "󰐊",         // nf-md-play (updated)
            Icon::Build => "󰛶",       // nf-md-hammer (updated)
            
            // Status
            Icon::Success => "󰄬",     // nf-md-check_circle (updated)
            Icon::Warning => "󰀪",     // nf-md-alert (updated)
            Icon::Error => "󰅖",       // nf-md-close_circle (updated)
            Icon::Info => "󰋼",        // nf-md-information (updated)
            
            // AI/Assistant
            Icon::Robot => "󰚩",       // nf-md-robot (updated)
            Icon::Sparkles => "󰠮",    // nf-md-auto_fix (updated)
            
            // UI elements
            Icon::Menu => "󰍜",        // nf-md-menu (updated)
            Icon::MoreHorizontal => "󰇘", // nf-md-dots_horizontal (updated)
            Icon::MoreVertical => "󰇙", // nf-md-dots_vertical (updated)
        }
    }

    /// Get the appropriate font for icon rendering.
    fn get_font(typography: &EditorTypographySettings) -> iced::Font {
        // For icons, we need to use a font that contains the icon glyphs
        // The icon font stack is defined in EditorTypographySettings
        // We'll use the first font from the icon font stack
        let icon_stack = typography.icon_font_stack();
        if let Some(first_font) = icon_stack.first() {
            iced::Font::with_name(first_font)
        } else {
            // Fallback to the selected font family
            let font_name = typography.font_family.to_family_string();
            iced::Font::with_name(font_name)
        }
    }

    /// Render this icon as a text element with appropriate styling.
    pub fn render<'a, Message>(
        &self,
        typography: &EditorTypographySettings,
        style: &StyleHelpers,
        size: Option<u16>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let icon_size = size.unwrap_or(typography.font_size);
        let icon_char = match typography.icon_mode {
            IconMode::NerdFonts => self.nerd_font_glyph(),
            IconMode::Unicode => self.unicode_fallback(),
            IconMode::Disabled => " ",
        };

        text(icon_char)
            .size(icon_size)
            .font(Self::get_font(typography))
            .style(iced::theme::Text::Color(style.text_secondary()))
            .into()
    }

    /// Render this icon as a text element with custom color.
    pub fn render_with_color<'a, Message>(
        &self,
        typography: &EditorTypographySettings,
        color: Color,
        size: Option<u16>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let icon_size = size.unwrap_or(typography.font_size);
        let icon_char = match typography.icon_mode {
            IconMode::NerdFonts => self.nerd_font_glyph(),
            IconMode::Unicode => self.unicode_fallback(),
            IconMode::Disabled => " ",
        };

        text(icon_char)
            .size(icon_size)
            .font(Self::get_font(typography))
            .style(iced::theme::Text::Color(color))
            .into()
    }
}

/// Helper to create an icon button with consistent styling.
pub fn icon_button<'a, Message>(
    icon: Icon,
    typography: &EditorTypographySettings,
    style: &StyleHelpers,
    on_press: Option<Message>,
    size: Option<u16>,
) -> iced::widget::Button<'a, Message>
where
    Message: Clone + 'a,
{
    let button = iced::widget::button(
        icon.render(typography, style, size)
    );

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
    .style(iced::theme::Button::Text)
    .padding(4)
}
