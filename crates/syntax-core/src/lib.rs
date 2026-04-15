//! Syntax layer for Neote IDE.
//!
//! This crate provides Tree-sitter-based syntax parsing, highlighting,
//! and language support for the editor. It's designed to be:
//! - Incremental: updates syntax trees efficiently after edits
//! - Modular: clean separation between parsing, highlighting, and UI
//! - Extensible: easy to add new languages and features
//! - Performant: minimal overhead for large files and frequent edits

pub mod error;
pub mod highlight;
pub mod language;
pub mod manager;
pub mod runtime;
pub mod grammar_registry;
pub mod grammar_builder;
pub mod dynamic_loader;
pub mod query_cache;

// Re-export main types for convenience
pub use error::SyntaxError;
pub use highlight::{Highlight, HighlightSpan};
pub use language::LanguageId;
pub use manager::SyntaxManager;
pub use grammar_registry::GrammarInfo;
pub use grammar_builder::{build_and_install_grammar, is_grammar_installed, install_missing_grammars};
pub use dynamic_loader::DynamicGrammarLoader;
// Note: QueryCache::get returns Option<&'static Query>
pub use query_cache::QueryCache;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_toml_language_detection() {
        use std::path::Path;
        
        assert_eq!(
            LanguageId::from_path(Path::new("Cargo.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("test.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new(".clippy.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("pyproject.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("rustfmt.toml")),
            LanguageId::Toml
        );
        assert_eq!(
            LanguageId::from_path(Path::new("config.toml")),
            LanguageId::Toml
        );
    }
    
    #[test]
    fn test_markdown_language_detection() {
        use std::path::Path;
        
        assert_eq!(
            LanguageId::from_path(Path::new("README.md")),
            LanguageId::Markdown
        );
        assert_eq!(
            LanguageId::from_path(Path::new("document.markdown")),
            LanguageId::Markdown
        );
        assert_eq!(
            LanguageId::from_path(Path::new("notes.MD")),
            LanguageId::Markdown
        );
        assert_eq!(
            LanguageId::from_path(Path::new("test.mdx")),
            LanguageId::PlainText  // .mdx is not supported by default
        );
    }
    
    #[cfg(feature = "markdown")]
    #[test]
    fn test_markdown_highlight_captures() {
        use crate::highlight::map_capture_name;
        
        // Test that Markdown-specific captures map to appropriate Highlight variants
        assert_eq!(map_capture_name("heading"), Highlight::Type);
        assert_eq!(map_capture_name("emphasis"), Highlight::Comment);
        assert_eq!(map_capture_name("strong"), Highlight::Keyword);
        assert_eq!(map_capture_name("link"), Highlight::Variable);
        assert_eq!(map_capture_name("inline_code"), Highlight::Constant);
        assert_eq!(map_capture_name("code_fence"), Highlight::Property);
        assert_eq!(map_capture_name("blockquote"), Highlight::Comment);
        assert_eq!(map_capture_name("list"), Highlight::Property);
        assert_eq!(map_capture_name("thematic_break"), Highlight::Operator);
        
        // Test that programming language captures still work
        assert_eq!(map_capture_name("comment"), Highlight::Comment);
        assert_eq!(map_capture_name("string"), Highlight::String);
        assert_eq!(map_capture_name("keyword"), Highlight::Keyword);
    }
    
    #[cfg(feature = "markdown")]
    #[test]
    fn test_markdown_language_parsing() {
        use crate::language::LanguageId;
        use std::path::Path;
        
        // Test that the language can be obtained
        let lang = LanguageId::Markdown;
        assert_eq!(lang.as_str(), "markdown");
        
        // Test that tree_sitter_language returns something when feature is enabled
        // This may panic if tree-sitter-markdown is not properly linked, but that's okay for a test
        let _ = lang.tree_sitter_language();
    }
    
    #[test]
    fn test_dynamic_language_registry() {
        use crate::grammar_registry::GrammarRegistry;
        
        let registry = GrammarRegistry::global();
        assert!(registry.contains_language("rust"));
        assert!(registry.contains_language("toml"));
        assert!(registry.contains_language("markdown"));
        
        let rust_info = registry.get("rust").unwrap();
        assert_eq!(rust_info.name, "Rust");
        assert!(rust_info.extensions.contains(&"rs".to_string()));
    }
}
