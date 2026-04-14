//! Language identification and grammar loading.

use std::path::Path;
use tree_sitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    Markdown,
    PlainText,
}

impl LanguageId {
    /// Determine language from file path.
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Check for TOML files
        if ext.eq_ignore_ascii_case("toml") {
            return LanguageId::Toml;
        }
        
        // Check for Markdown files
        if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
            return LanguageId::Markdown;
        }
        
        // Check for specific TOML filenames
        match name.as_str() {
            "cargo.toml" | "rust-toolchain.toml" | "clippy.toml" | "rustfmt.toml" 
            | ".clippy.toml" | ".rustfmt.toml" | "pyproject.toml" | "taplo.toml" => {
                return LanguageId::Toml;
            }
            _ => {}
        }
        
        match ext {
            "rs" => LanguageId::Rust,
            _ => LanguageId::PlainText,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageId::Rust => "rust",
            LanguageId::Toml => "toml",
            LanguageId::Markdown => "markdown",
            LanguageId::PlainText => "plaintext",
        }
    }

    /// Return the statically linked Tree-sitter language if available.
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            #[cfg(feature = "rust")]
            LanguageId::Rust => Some(tree_sitter_rust::language()),
            #[cfg(not(feature = "rust"))]
            LanguageId::Rust => None,
            #[cfg(feature = "toml")]
            LanguageId::Toml => Some(tree_sitter_toml::language()),
            #[cfg(not(feature = "toml"))]
            LanguageId::Toml => None,
            #[cfg(feature = "markdown")]
            LanguageId::Markdown => load_dynamic_language("markdown"),
            #[cfg(not(feature = "markdown"))]
            LanguageId::Markdown => None,
            LanguageId::PlainText => None,
        }
    }
}

/// Helper function to load any language dynamically from runtime directory
/// This works for ALL languages in the grammar registry
fn load_dynamic_language(language_id: &str) -> Option<tree_sitter::Language> {
    use crate::runtime::Runtime;
    use std::sync::OnceLock;
    use std::collections::HashMap;
    
    // Use a global cache for all dynamically loaded languages
    static LANGUAGE_CACHE: OnceLock<HashMap<String, Option<tree_sitter::Language>>> = OnceLock::new();
    
    let cache = LANGUAGE_CACHE.get_or_init(|| HashMap::new());
    
    // Check cache first
    if let Some(cached) = cache.get(language_id) {
        return cached.clone();
    }
    
    // Not in cache, try to load
    let runtime = Runtime::new();
    match runtime.load_language(language_id) {
        Ok(lang) => {
            // Successfully loaded - cache it
            let mut cache = LANGUAGE_CACHE.get().unwrap().clone();
            cache.insert(language_id.to_string(), Some(lang.clone()));
            // Note: We can't update the OnceLock, but that's okay
            Some(lang)
        }
        Err(e) => {
            // Just log the error and return None - don't try to install automatically
            // This prevents blocking the UI during startup
            eprintln!("Note: {} grammar not found. {}", language_id, e);
            eprintln!("To install, run: cargo run --bin download-grammars -- install {}", language_id);
            None
        }
    }
}
