//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, Tree};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    Property,
    Namespace,
    Plain,
}

/// Highlight a syntax tree for a given language.
pub fn highlight(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    match language {
        LanguageId::Rust => highlight_with_query(language, source, tree),
        #[cfg(feature = "toml")]
        LanguageId::Toml => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "toml"))]
        LanguageId::Toml => Ok(Vec::new()),
        #[cfg(feature = "markdown")]
        LanguageId::Markdown => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "markdown"))]
        LanguageId::Markdown => Ok(Vec::new()),
        LanguageId::PlainText => Ok(Vec::new()),
        LanguageId::Dynamic(_) => highlight_with_query(language, source, tree),
    }
}

fn highlight_with_query(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let query_str = get_query_for_language(language)?;
    let ts_lang = language
        .tree_sitter_language()
        .ok_or_else(|| SyntaxError::LanguageNotSupported(language.as_str().to_string()))?;

    // If the query fails (e.g., because of unrecognized node types), we return empty highlights
    // rather than propagating an error. This allows the editor to keep working without syntax
    // highlighting for that particular language.
    let query = match Query::new(ts_lang, query_str) {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            return Ok(Vec::new());
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    for match_ in cursor.matches(&query, root_node, source.as_bytes()) {
        for capture in match_.captures {
            let node = capture.node;
            let start = node.start_byte();
            let end = node.end_byte();
            let capture_name = &query.capture_names()[capture.index as usize];
            let highlight = map_capture_name(capture_name);
            spans.push(HighlightSpan {
                start,
                end,
                highlight,
            });
        }
    }

    // Sort spans by start position
    spans.sort_by_key(|span| span.start);
    
    Ok(spans)
}

pub fn map_capture_name(name: &str) -> Highlight {
    match name {
        // Programming language captures
        "comment" => Highlight::Comment,
        "string" => Highlight::String,
        "string.escape" => Highlight::String,
        "escape" => Highlight::String,
        "string.special" => Highlight::String,
        "keyword" => Highlight::Keyword,
        "function" | "function.call" | "function.method" => Highlight::Function,
        "function.macro" | "macro" => Highlight::Function, // Macros use function color
        "variable" | "variable.parameter" => Highlight::Variable,
        "variable.builtin" => Highlight::Type, // Built-in variables like 'self' use type color (amber in dark theme)
        "type" | "type.builtin" => Highlight::Type,
        "constant" | "constant.builtin" => Highlight::Constant,
        "attribute" => Highlight::Attribute,
        "operator" => Highlight::Operator,
        "punctuation.bracket" => Highlight::Operator,
        "punctuation.delimiter" => Highlight::Operator,
        "number" => Highlight::Number,
        "boolean" => Highlight::Constant,
        "property" => Highlight::Property,
        "namespace" => Highlight::Namespace,
        "constructor" => Highlight::Type,
        "label" => Highlight::Variable,
        "mutable_specifier" => Highlight::Keyword,
        "lifetime" => Highlight::Type,  // Lifetimes use type color
        
        // Markdown-specific captures (based on tree-sitter-markdown grammar)
        // These may vary between versions
        "heading" => Highlight::Type,
        "heading.1" => Highlight::Type,
        "heading.2" => Highlight::Type,
        "heading.3" => Highlight::Type,
        "heading.4" => Highlight::Type,
        "heading.5" => Highlight::Type,
        "heading.6" => Highlight::Type,
        "atx_heading" => Highlight::Type,
        "setext_heading" => Highlight::Type,
        "emphasis" => Highlight::Comment,
        "strong_emphasis" => Highlight::Keyword,
        "strong" => Highlight::Keyword,
        "link" => Highlight::Variable,
        "link_text" => Highlight::Variable,
        "link_destination" => Highlight::String,
        "link_url" => Highlight::String,
        "link_title" => Highlight::String,
        "inline_code_span" => Highlight::Constant,
        "inline_code" => Highlight::Constant,
        "code_fence" => Highlight::Property,
        "block_quote" => Highlight::Comment,
        "blockquote" => Highlight::Comment,
        "list" => Highlight::Property,
        "list_item" => Highlight::Property,
        "thematic_break" => Highlight::Operator,
        "html_block" => Highlight::Attribute,
        "html_inline" => Highlight::Attribute,
        "table" => Highlight::Property,
        "table_header" => Highlight::Type,
        "table_row" => Highlight::Property,
        "table_cell" => Highlight::Plain,
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    match language {
        LanguageId::Rust => {
            if let Some(_query) = crate::query_cache::get_query("rust", "highlights") {
                // Store the query source in a static string
                static mut RUST_QUERY: Option<&'static str> = None;
                unsafe {
                    if RUST_QUERY.is_none() {
                        // Get the query text from the query object
                        // Note: tree-sitter Query doesn't have a source() method
                        // We'll need to load it from file directly
                        let runtime = crate::runtime::Runtime::new();
                        let query_path = runtime.language_dir("rust").join("queries/highlights.scm");
                        if let Ok(query_text) = std::fs::read_to_string(&query_path) {
                            RUST_QUERY = Some(Box::leak(query_text.into_boxed_str()));
                        }
                    }
                    if let Some(query_str) = RUST_QUERY {
                        Ok(query_str)
                    } else {
                        Err(SyntaxError::LanguageNotSupported(
                            "rust grammar not available".to_string(),
                        ))
                    }
                }
            } else {
                Err(SyntaxError::LanguageNotSupported(
                    "rust grammar not available".to_string(),
                ))
            }
        }
        LanguageId::Toml => {
            if let Some(_query) = crate::query_cache::get_query("toml", "highlights") {
                static mut TOML_QUERY: Option<&'static str> = None;
                unsafe {
                    if TOML_QUERY.is_none() {
                        let runtime = crate::runtime::Runtime::new();
                        let query_path = runtime.language_dir("toml").join("queries/highlights.scm");
                        if let Ok(query_text) = std::fs::read_to_string(&query_path) {
                            TOML_QUERY = Some(Box::leak(query_text.into_boxed_str()));
                        }
                    }
                    if let Some(query_str) = TOML_QUERY {
                        Ok(query_str)
                    } else {
                        Err(SyntaxError::LanguageNotSupported(
                            "toml grammar not available".to_string(),
                        ))
                    }
                }
            } else {
                Err(SyntaxError::LanguageNotSupported(
                    "toml grammar not available".to_string(),
                ))
            }
        }
        LanguageId::Markdown => {
            if let Some(_query) = crate::query_cache::get_query("markdown", "highlights") {
                static mut MARKDOWN_QUERY: Option<&'static str> = None;
                unsafe {
                    if MARKDOWN_QUERY.is_none() {
                        let runtime = crate::runtime::Runtime::new();
                        let query_path = runtime.language_dir("markdown").join("queries/highlights.scm");
                        if let Ok(query_text) = std::fs::read_to_string(&query_path) {
                            MARKDOWN_QUERY = Some(Box::leak(query_text.into_boxed_str()));
                        }
                    }
                    if let Some(query_str) = MARKDOWN_QUERY {
                        Ok(query_str)
                    } else {
                        Err(SyntaxError::LanguageNotSupported(
                            "markdown grammar not available".to_string(),
                        ))
                    }
                }
            } else {
                Err(SyntaxError::LanguageNotSupported(
                    "markdown grammar not available".to_string(),
                ))
            }
        }
        LanguageId::PlainText => Err(SyntaxError::LanguageNotSupported(
            "plaintext has no syntax queries".to_string(),
        )),
        LanguageId::Dynamic(id) => {
            // Use OnceLock to lazily initialize the HashMap
            use std::sync::OnceLock;
            use std::collections::HashMap;
            
            static DYNAMIC_QUERIES: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
            
            // Get or initialize the HashMap
            let queries = DYNAMIC_QUERIES.get_or_init(|| HashMap::new());
            
            // Check if we already have the query cached
            if let Some(query_str) = queries.get(id) {
                return Ok(query_str);
            }
            
            // Try to load the query from file
            let runtime = crate::runtime::Runtime::new();
            let query_path = runtime.language_dir(id).join("queries/highlights.scm");
            
            match std::fs::read_to_string(&query_path) {
                Ok(query_text) => {
                    // Leak the string to make it static
                    let query_str = Box::leak(query_text.into_boxed_str());
                    
                    // Insert into the HashMap (we need to get a mutable reference)
                    // Since OnceLock only gives us &, we need to handle this differently
                    // We'll use a Mutex for interior mutability
                    use std::sync::Mutex;
                    static DYNAMIC_QUERIES_MUTEX: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
                    
                    let mutex = DYNAMIC_QUERIES_MUTEX.get_or_init(|| Mutex::new(HashMap::new()));
                    let mut queries_guard = mutex.lock().unwrap();
                    queries_guard.insert(id.to_string(), query_str);
                    
                    Ok(query_str)
                }
                Err(_) => {
                    Err(SyntaxError::LanguageNotSupported(
                        format!("{} grammar not available", id),
                    ))
                }
            }
        }
    }
}
