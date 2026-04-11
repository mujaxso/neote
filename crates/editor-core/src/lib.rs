//! Editor core engine for Neote IDE.
//! 
//! This crate provides the domain model and editing operations for the Neote editor,
//! separate from UI concerns. It's designed to be future-proof for LSP integration,
//! syntax highlighting, and other advanced IDE features.

pub mod document;
pub mod editor;
pub mod cursor;
pub mod selection;
pub mod viewport;
pub mod commands;
pub mod events;

// Re-export main types for convenience
pub use document::Document;
pub use editor::EditorState;
pub use cursor::{Cursor, CursorMovement};
pub use selection::Selection;
pub use viewport::Viewport;
pub use commands::EditorCommand;
pub use events::EditorEvent;
