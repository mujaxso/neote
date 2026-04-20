//! AI context collection, ranking, packing, and prompt construction.

pub mod context;
pub mod ranking;
pub mod packing;
pub mod prompt;

/// Prelude for convenient imports.
pub mod prelude {
    pub use super::context::*;
    pub use super::ranking::*;
    pub use super::packing::*;
    pub use super::prompt::*;
}
