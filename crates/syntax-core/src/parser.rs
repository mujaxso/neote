//! Incremental parser management for Tree-sitter.

use tree_sitter::{Parser, Tree, InputEdit};
use ropey::Rope;
use std::sync::Arc;
use parking_lot::Mutex;

use crate::language::LanguageId;
use crate::SyntaxError;

/// A syntax tree with its associated text and language
pub struct SyntaxTree {
    /// The Tree-sitter parse tree
    tree: Tree,
    /// The text content as a rope for efficient editing
    text: Rope,
    /// Language of this tree
    language: LanguageId,
    /// Parser instance (kept for incremental parsing)
    parser: Arc<Mutex<Parser>>,
}

impl SyntaxTree {
    /// Create a new syntax tree by parsing text
    pub fn new(
        parser: Arc<Mutex<Parser>>,
        text: &str,
        language: LanguageId,
    ) -> Result<Self, SyntaxError> {
        let mut parser_guard = parser.lock();
        let tree = parser_guard
            .parse(text, None)
            .ok_or_else(|| SyntaxError::ParserError("Failed to parse text".to_string()))?;
        
        drop(parser_guard);
        
        Ok(Self {
            tree,
            text: Rope::from_str(text),
            language,
            parser,
        })
    }

    /// Update the syntax tree with an edit
    pub fn edit(&mut self, start_byte: usize, old_end_byte: usize, new_end_byte: usize, 
                start_position: tree_sitter::Point, old_end_position: tree_sitter::Point, 
                new_end_position: tree_sitter::Point) {
        let edit = InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position,
            old_end_position,
            new_end_position,
        };
        
        self.tree.edit(&edit);
        // Note: The text rope is updated in SyntaxDocument::edit, not here
        // This is to keep the text in sync between SyntaxDocument and SyntaxTree
    }

    /// Reparse the tree incrementally after edits
    pub fn reparse(&mut self) -> Result<(), SyntaxError> {
        let mut parser = self.parser.lock();
        let text_str = self.text.to_string();
        
        // Parse with the old tree for incremental parsing
        let new_tree = parser
            .parse(&text_str, Some(&self.tree))
            .ok_or_else(|| SyntaxError::ParserError("Failed to reparse".to_string()))?;
        
        // Replace the old tree with the new one
        self.tree = new_tree;
        
        Ok(())
    }

    /// Get the text as a string
    pub fn text(&self) -> String {
        self.text.to_string()
    }

    /// Get the underlying Tree-sitter tree
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Get the language
    pub fn language(&self) -> LanguageId {
        self.language
    }
}

// The Tree type handles its own cleanup, so we don't need a custom Drop implementation
