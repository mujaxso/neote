use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use zaroxi_domain_editor::document_cache::{BufferManager, CachedDocument};
use zaroxi_domain_editor::Document;
use zaroxi_ops_file::FileLoader;

/// App‑specific editor service that orchestrates domain editor logic
#[allow(dead_code)]
pub struct EditorService {
    buffer_manager: Arc<BufferManager>,
}

#[allow(dead_code)]
impl EditorService {
    pub fn new() -> Self {
        Self {
            buffer_manager: Arc::new(BufferManager::new()),
        }
    }

    /// Get a reference to the global buffer manager.
    pub fn buffer_manager(&self) -> &Arc<BufferManager> {
        &self.buffer_manager
    }

    /// Create a new document from file content (used for testing or when no cache is needed).
    pub fn create_document_from_file(&self, path: PathBuf, content: String) -> Result<Document, anyhow::Error> {
        let mut document = Document::new();

        document
            .insert(0, &content)
            .map_err(|e| anyhow::anyhow!("Failed to insert content into document: {}", e))?;

        document.set_path(Some(path.to_string_lossy().to_string()));

        Ok(document)
    }

    /// Get document content as string
    pub fn get_document_content(&self, document: &Document) -> String {
        document.text()
    }

    /// Open a document using the buffer manager (cached).
    pub async fn open_document(&self, path: &PathBuf) -> Result<Arc<Mutex<CachedDocument>>, String> {
        self.buffer_manager
            .open_document(path, &FileLoader)
            .await
    }

    /// Get a cached document without disk I/O.
    pub async fn get_cached_document(&self, path: &PathBuf) -> Option<Arc<Mutex<CachedDocument>>> {
        self.buffer_manager.get_cached(path).await
    }
}
