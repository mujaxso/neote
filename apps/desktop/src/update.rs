//! 
//! This module delegates to domain-specific update modules in the `update` directory.
//! Each domain (workspace, explorer, editor, etc.) handles its own messages.

mod update;

pub use update::update;
