pub mod chat;
pub mod editor;
pub mod layout;
pub mod sidebar;
pub mod terminal;

// Re-export commonly used components
pub use layout::layout;
pub use editor::{editor, header};
pub use sidebar::file_list;
pub use chat::chat;
pub use terminal::terminal;
