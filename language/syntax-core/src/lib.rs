//! Syntax highlighting for Zaroxi

use tree_sitter::Parser;

/// Syntax highlighter
pub struct Highlighter {
    _parser: Parser,
}

impl Highlighter {
    /// Create a new highlighter
    pub fn new() -> Self {
        let parser = Parser::new();
        // TODO: Initialize with a language
        Self { _parser: parser }
    }
}
