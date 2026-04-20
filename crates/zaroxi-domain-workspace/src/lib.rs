//! Workspace domain models for Zaroxi.

pub mod workspace;
pub mod file_tree;

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::workspace::*;
    pub use super::file_tree::*;
}
