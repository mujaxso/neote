//! Syntax manager for coordinating multiple documents and languages.

use crate::error::SyntaxError;
use crate::highlight::{highlight, HighlightSpan};
use crate::language::LanguageId;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Parser, Tree};

pub struct SyntaxManager {
    documents: HashMap<String, SyntaxDocument>,
    // Cache parsers per language to avoid recreating them
    parsers: HashMap<LanguageId, Parser>,
}

struct SyntaxDocument {
    text: String,
    language: LanguageId,
    tree: Option<Tree>,
}

impl SyntaxManager {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            parsers: HashMap::new(),
        }
    }

    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        eprintln!("DEBUG: update_document for path: {}", path.display());
        let language = LanguageId::from_path(path);
        eprintln!("DEBUG: Language determined: {:?}", language);
        
        // Get or create parser for this language
        let parser = self.parsers.entry(language).or_insert_with(|| {
            eprintln!("DEBUG: Creating new parser for language");
            let mut parser = Parser::new();
            if let Some(ts_lang) = language.tree_sitter_language() {
                eprintln!("DEBUG: Setting tree-sitter language");
                let _ = parser.set_language(ts_lang);
            } else {
                eprintln!("DEBUG: No tree-sitter language available");
            }
            parser
        });

        let tree = if language.tree_sitter_language().is_some() {
            eprintln!("DEBUG: Parsing document");
            parser.parse(text, None)
        } else {
            eprintln!("DEBUG: No tree-sitter language, skipping parse");
            None
        };

        let doc = SyntaxDocument {
            text: text.to_string(),
            language,
            tree,
        };
        self.documents.insert(doc_id.to_string(), doc);
        eprintln!("DEBUG: Document updated successfully");
        Ok(())
    }

    pub fn edit_document(
        &mut self,
        doc_id: &str,
        start_byte: usize,
        old_end_byte: usize,
        new_text: &str,
    ) -> Result<(), SyntaxError> {
        // For simplicity, we reparse the whole document after each edit.
        // A real implementation would use incremental parsing.
        // Currently we do nothing, but we could call update_document again.
        // However we don't have the original text here.
        // We'll just ignore for now.
        let _ = (doc_id, start_byte, old_end_byte, new_text);
        Ok(())
    }

    pub fn contains_document(&self, doc_id: &str) -> bool {
        self.documents.contains_key(doc_id)
    }

    pub fn highlight_spans(&self, doc_id: &str) -> Result<Vec<HighlightSpan>, SyntaxError> {
        let doc = self
            .documents
            .get(doc_id)
            .ok_or(SyntaxError::DocumentNotFound)?;
        match &doc.tree {
            Some(tree) => {
                // Use the global query cache
                // For now, just use the standard highlight function which uses the query cache
                highlight(doc.language, &doc.text, tree)
            }
            None => Ok(Vec::new()),
        }
    }
    
    /// Initialize dynamic grammars and preload queries
    pub fn initialize_dynamic_grammars(&mut self) {
        use crate::dynamic_loader::preload_available_grammars;
        use crate::query_cache::preload_queries;
        
        // Preload available grammars
        preload_available_grammars();
        
        // Preload queries
        preload_queries();
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
