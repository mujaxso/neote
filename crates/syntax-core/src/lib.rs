//! Syntax layer for Neote IDE.
//!
//! This crate provides Tree-sitter-based syntax parsing, highlighting,
//! and language support for the editor. It's designed to be:
//! - Incremental: updates syntax trees efficiently after edits
//! - Modular: clean separation between parsing, highlighting, and UI
//! - Extensible: easy to add new languages and features
//! - Performant: minimal overhead for large files and frequent edits

pub mod document;
pub mod language;
pub mod parser;
pub mod highlight;
pub mod snapshot;
pub mod manager;
pub mod runtime;
pub mod metadata;

// Re-export main types for convenience
pub use document::SyntaxDocument;
pub use language::{LanguageId, LanguageRegistry};
pub use highlight::{HighlightConfiguration, Highlight, HighlightSpan};
pub use snapshot::SyntaxSnapshot;
pub use manager::SyntaxManager;
pub use runtime::Runtime;
pub use metadata::LanguageMetadata;

/// Error type for syntax operations
#[derive(Debug, thiserror::Error)]
pub enum SyntaxError {
    #[error("Language not supported: {0}")]
    LanguageNotSupported(String),
    #[error("Parser error: {0}")]
    ParserError(String),
    #[error("Query error: {0}")]
    QueryError(String),
    #[error("Invalid edit range")]
    InvalidEditRange,
    #[error("Document not found")]
    DocumentNotFound,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Grammar load error: {0}")]
    GrammarLoadError(String),
    #[error("Metadata error: {0}")]
    MetadataError(String),
    #[error("Runtime path error: {0}")]
    RuntimePathError(String),
}
