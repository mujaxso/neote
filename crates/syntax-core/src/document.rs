//! Syntax-aware document wrapper that combines text with syntax trees.

use ropey::Rope;
use std::sync::Arc;
use parking_lot::Mutex;

use crate::language::LanguageId;
use crate::parser::SyntaxTree;
use crate::highlight::{HighlightConfiguration, HighlightSpan};
use crate::SyntaxError;

/// A document with syntax tree support
pub struct SyntaxDocument {
    /// The text content
    text: Rope,
    /// The syntax tree (if language is supported)
    syntax_tree: Option<SyntaxTree>,
    /// Language of this document
    language: LanguageId,
    /// Highlight configuration for this language
    highlight_config: Option<Arc<HighlightConfiguration>>,
    /// Whether the syntax tree needs reparsing
    needs_reparse: bool,
}

impl SyntaxDocument {
    /// Create a new syntax document
    pub fn new(
        text: &str,
        language: LanguageId,
        highlight_config: Option<Arc<HighlightConfiguration>>,
        parser: Option<Arc<Mutex<tree_sitter::Parser>>>,
    ) -> Result<Self, SyntaxError> {
        let syntax_tree = if let Some(parser) = parser {
            // Try to create a syntax tree if we have a parser
            match SyntaxTree::new(parser.clone(), text, language) {
                Ok(tree) => Some(tree),
                Err(e) => {
                    // If we can't create a syntax tree, log it but continue without one
                    eprintln!("Failed to create syntax tree for {:?}: {}", language, e);
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            text: Rope::from_str(text),
            syntax_tree,
            language,
            highlight_config,
            needs_reparse: false,
        })
    }

    /// Apply a text edit to the document
    pub fn edit(&mut self, start_byte: usize, old_end_byte: usize, new_text: &str) -> Result<(), SyntaxError> {
        // Validate edit range
        if start_byte > old_end_byte {
            return Err(SyntaxError::InvalidEditRange);
        }
        
        // Calculate positions before any mutable borrows
        let start_position = self.byte_to_point(start_byte);
        let old_end_position = self.byte_to_point(old_end_byte);
        let new_end_byte = start_byte + new_text.len();
        let new_end_position = self.byte_to_point(new_end_byte);
        
        // Update the rope text
        let start_char = self.text.byte_to_char(start_byte);
        let old_end_char = self.text.byte_to_char(old_end_byte);
        
        self.text.remove(start_char..old_end_char);
        self.text.insert(start_char, new_text);

        // Update syntax tree if it exists
        if let Some(tree) = &mut self.syntax_tree {
            // Apply edit to syntax tree
            tree.edit(start_byte, old_end_byte, new_end_byte, 
                     start_position, old_end_position, new_end_position);
            
            self.needs_reparse = true;
        }

        Ok(())
    }

    /// Reparse the document if needed
    pub fn reparse_if_needed(&mut self) -> Result<(), SyntaxError> {
        if self.needs_reparse {
            if let Some(tree) = &mut self.syntax_tree {
                match tree.reparse() {
                    Ok(()) => {
                        self.needs_reparse = false;
                    }
                    Err(e) => {
                        // If reparse fails, we'll keep the tree but mark it as potentially stale
                        eprintln!("Failed to reparse syntax tree: {}", e);
                        // Don't clear needs_reparse so we can try again later
                    }
                }
            }
        }
        Ok(())
    }

    /// Get the document text
    pub fn text(&self) -> String {
        self.text.to_string()
    }

    /// Get the language
    pub fn language(&self) -> LanguageId {
        self.language
    }

    /// Get syntax tree (if available)
    pub fn syntax_tree(&self) -> Option<&SyntaxTree> {
        self.syntax_tree.as_ref()
    }

    /// Check if this document has syntax support
    pub fn has_syntax_support(&self) -> bool {
        self.syntax_tree.is_some() && self.highlight_config.is_some()
    }

    /// Get highlight spans for the document
    pub fn highlight_spans(&self) -> Result<Vec<HighlightSpan>, SyntaxError> {
        if let (Some(tree), Some(config)) = (&self.syntax_tree, &self.highlight_config) {
            crate::highlight::highlight_tree(tree.tree(), &self.text, config)
        } else {
            // Return empty highlights for unsupported languages
            Ok(Vec::new())
        }
    }

    /// Convert byte offset to tree-sitter point
    fn byte_to_point(&self, byte: usize) -> tree_sitter::Point {
        let line = self.text.byte_to_line(byte);
        let line_start = self.text.line_to_byte(line);
        let column = byte - line_start;
        
        tree_sitter::Point { row: line, column }
    }
}
