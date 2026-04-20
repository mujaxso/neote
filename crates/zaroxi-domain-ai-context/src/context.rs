//! AI context collection.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A piece of context that can be used to inform AI decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextItem {
    /// Unique identifier for this context item.
    pub id: Uuid,
    /// The content of the context.
    pub content: String,
    /// The source of the context (e.g., "file", "buffer", "clipboard").
    pub source: String,
    /// A relevance score (higher means more relevant).
    pub relevance: f32,
}

/// A collection of context items.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ContextCollection {
    /// The items in the collection.
    pub items: Vec<ContextItem>,
}

impl ContextCollection {
    /// Create a new empty context collection.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a context item to the collection.
    pub fn add(&mut self, item: ContextItem) {
        self.items.push(item);
    }

    /// Get the number of items in the collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
