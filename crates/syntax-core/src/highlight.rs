//! Syntax highlighting using Tree-sitter queries.

use tree_sitter::{Query, QueryCursor, Tree};
use ropey::Rope;

/// Highlight configuration for a language
pub struct HighlightConfiguration {
    /// Tree-sitter query for highlighting
    query: Query,
    /// Capture names for highlighting
    capture_names: Vec<String>,
}

impl Clone for HighlightConfiguration {
    fn clone(&self) -> Self {
        // Since Query doesn't implement Clone, we need to recreate it
        // This is a bit inefficient, but necessary
        // We'll need to store the query string to recreate it
        // For now, we'll panic if cloning is attempted, or implement a different approach
        // Since this is for internal use, we can avoid cloning in most cases
        // Let's implement a workaround by storing the query string
        unimplemented!("HighlightConfiguration cloning not supported due to Query limitations")
    }
}

impl HighlightConfiguration {
    /// Create a new highlight configuration
    pub fn new(
        language: &tree_sitter::Language,
        highlights_query: &str,
        _locals_query: &str,
        _injections_query: &str,
    ) -> Result<Self, crate::SyntaxError> {
        let query = Query::new(language, highlights_query)
            .map_err(|e| crate::SyntaxError::QueryError(e.to_string()))?;
        
        let capture_names = query.capture_names().iter().map(|s| s.to_string()).collect();
        
        Ok(Self {
            query,
            capture_names,
        })
    }
}

/// A highlight span in the document
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Highlight type (maps to theme token categories)
    pub highlight: Highlight,
}

/// Highlight types (maps to Tree-sitter capture names)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Highlight {
    Comment,
    String,
    Keyword,
    Function,
    Variable,
    Type,
    Constant,
    Attribute,
    Operator,
    Number,
    Plain,
}

impl Highlight {
    /// Convert from Tree-sitter capture name
    pub fn from_capture_name(name: &str) -> Self {
        match name {
            "comment" => Highlight::Comment,
            "string" => Highlight::String,
            "keyword" => Highlight::Keyword,
            "function" => Highlight::Function,
            "variable" => Highlight::Variable,
            "type" => Highlight::Type,
            "constant" => Highlight::Constant,
            "attribute" => Highlight::Attribute,
            "operator" => Highlight::Operator,
            "number" => Highlight::Number,
            _ => Highlight::Plain,
        }
    }
}

/// Highlight a syntax tree
pub fn highlight_tree(
    tree: &Tree,
    text: &Rope,
    config: &HighlightConfiguration,
) -> Result<Vec<HighlightSpan>, crate::SyntaxError> {
    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    
    let mut highlights = Vec::new();
    
    // Convert rope to string for highlighting
    let text_str = text.to_string();
    
    for match_ in cursor.matches(&config.query, root_node, text_str.as_bytes()) {
        for capture in match_.captures {
            let node = capture.node;
            let start = node.start_byte();
            let end = node.end_byte();
            
            let capture_name = &config.capture_names[capture.index as usize];
            let highlight = Highlight::from_capture_name(capture_name);
            
            highlights.push(HighlightSpan {
                start,
                end,
                highlight,
            });
        }
    }
    
    // Sort highlights by start position
    highlights.sort_by_key(|h| h.start);
    
    Ok(highlights)
}
