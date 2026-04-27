//! In‑memory document cache for the editor.
//!
//! This module provides a cache that stores loaded documents keyed by
//! their canonical file path.  The cache is designed to be shared across
//! multiple editor views so that switching tabs does not require re‑reading
//! the file from disk or rebuilding the rope.
//!
//! **Large‑file policy**: documents of class `Large` still hold a rope,
//! but the frontend will receive a truncated preview and treat the file
//! as read‑only.  The rope is never cloned after construction – we share
//! an `Arc<Mutex<CachedDocument>>`.

use crate::document::Document;
use crate::thresholds::FileClass;
use zaroxi_ops_file::file_loader::FileLoader;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use parking_lot::Mutex;

/// Metadata stored alongside each cached document.
#[derive(Debug, Clone)]
pub struct CachedDocumentMeta {
    /// The canonical file path (used as the cache key).
    pub path: PathBuf,
    /// File size in bytes at the time of loading.
    pub file_size: u64,
    /// File modification time (seconds since epoch) at the time of loading.
    pub mtime_secs: u64,
    /// When this entry was last accessed (for LRU eviction).
    pub last_access: Instant,
    /// Whether the document has unsaved changes.
    pub is_dirty: bool,
    /// Monotonically increasing version counter (incremented on each edit).
    pub version: u64,
    /// File classification (Normal / Medium / Large).
    pub file_class: FileClass,
}

/// A cached document together with its metadata.
#[derive(Debug)]
pub struct CachedDocument {
    pub meta: CachedDocumentMeta,
    /// The actual document (rope + path + dirty flag).
    pub document: Document,
}

impl CachedDocument {
    /// Create a new cached entry from a loaded `Document`.
    pub fn new(document: Document, file_size: u64, mtime_secs: u64) -> Self {
        let file_class = document.file_class();
        let version = document.version();
        let is_dirty = document.is_dirty();
        let path = document
            .path()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::new());

        Self {
            meta: CachedDocumentMeta {
                path,
                file_size,
                mtime_secs,
                last_access: Instant::now(),
                is_dirty,
                version,
                file_class,
            },
            document,
        }
    }

    /// Touch the last‑access timestamp (called when the document is activated).
    pub fn touch(&mut self) {
        self.meta.last_access = Instant::now();
    }

    /// Return the document's text content (may be expensive for large files).
    pub fn text(&self) -> String {
        self.document.text()
    }

    /// Return the number of lines.
    pub fn len_lines(&self) -> usize {
        self.document.len_lines()
    }

    /// Return the number of characters.
    pub fn len_chars(&self) -> usize {
        self.document.len_chars()
    }
}

// ---------------------------------------------------------------------------
// DocumentCache – central cache store (now stores Arc<Mutex<CachedDocument>>)
// ---------------------------------------------------------------------------

/// Thread‑safe cache of open documents, keyed by canonical file path.
#[derive(Debug)]
pub struct DocumentCache {
    /// The actual cache entries.  Each entry is guarded by a mutex so that
    /// the document can be mutated (edit/mark dirty) while multiple
    /// callers share a cheap `Arc` clone.
    entries: HashMap<PathBuf, Arc<Mutex<CachedDocument>>>,
    /// Maximum number of entries before eviction kicks in (0 = unlimited).
    max_entries: usize,
}

impl DocumentCache {
    /// Create a new empty cache.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
        }
    }

    /// Insert or replace a cached document.
    pub fn insert(&mut self, path: PathBuf, doc: CachedDocument) -> Arc<Mutex<CachedDocument>> {
        // If we are at capacity, evict the least recently used entry (unless it's dirty).
        if self.max_entries > 0 && self.entries.len() >= self.max_entries {
            self.evict_lru();
        }
        let entry = Arc::new(Mutex::new(doc));
        self.entries.insert(path, entry.clone());
        entry
    }

    /// Retrieve a cached document by path, touching its last‑access time.
    /// Returns a clone of the `Arc` so the caller can lock it and read/mutate.
    pub fn get(&mut self, path: &Path) -> Option<Arc<Mutex<CachedDocument>>> {
        if let Some(entry) = self.entries.get(path) {
            let mut guard = entry.lock();
            guard.touch();
            // Give the caller a clone of the Arc (cheap)
            Some(Arc::clone(entry))
        } else {
            None
        }
    }

    /// Check whether a path is already cached.
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.contains_key(path)
    }

    /// Remove a document from the cache (e.g., when the tab is closed).
    pub fn remove(&mut self, path: &Path) -> Option<Arc<Mutex<CachedDocument>>> {
        self.entries.remove(path)
    }

    /// Number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Evict the least recently used non‑dirty entry.
    fn evict_lru(&mut self) {
        let mut oldest: Option<PathBuf> = None;
        let mut oldest_time = std::time::Instant::now();

        for (path, entry) in &self.entries {
            let guard = entry.lock();
            if guard.meta.is_dirty {
                // Never evict dirty documents.
                continue;
            }
            if guard.meta.last_access < oldest_time {
                oldest_time = guard.meta.last_access;
                oldest = Some(path.clone());
            }
        }

        if let Some(path) = oldest {
            self.entries.remove(&path);
        }
    }
}

// ---------------------------------------------------------------------------
// BufferManager – high‑level API for the editor commands
// ---------------------------------------------------------------------------

/// The global buffer manager that owns the document cache.
///
/// This is the single point of contact for the Tauri commands and the frontend
/// service.  It ensures that opening a file reuses an already‑cached document
/// when possible, and that dirty documents are never silently replaced.
#[derive(Debug)]
pub struct BufferManager {
    cache: Mutex<DocumentCache>,
}

impl BufferManager {
    /// Create a new buffer manager with a cache that holds at most 8 entries.
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(DocumentCache::new(8)),
        }
    }

    /// Open a document by path.
    ///
    /// If the document is already cached and the file on disk has not changed,
    /// the cached version is returned immediately (no disk I/O).
    ///
    /// If the file has changed on disk, the cache is updated (unless the cached
    /// version is dirty, in which case the caller must decide how to reconcile).
    ///
    /// Returns an `Arc<Mutex<CachedDocument>>`; the caller must lock the
    /// mutex to access the inner data.
    pub async fn open_document(
        &self,
        path: &Path,
        _file_loader: &FileLoader,
    ) -> Result<Arc<Mutex<CachedDocument>>, String> {
        let canonical = path
            .canonicalize()
            .map_err(|e| format!("Cannot canonicalize path: {}", e))?;

        // Check cache first.
        {
            let mut cache = self.cache.lock();
            if let Some(entry) = cache.get(&canonical) {
                let guard = entry.lock();
                // If the cached document is dirty, we must NOT replace it with
                // stale disk content.  Return the dirty version.
                if guard.meta.is_dirty {
                    return Ok(Arc::clone(&entry));
                }

                // Check if the file on disk has changed.
                let current_meta = std::fs::metadata(&canonical)
                    .map_err(|e| format!("Cannot stat file: {}", e))?;
                let current_mtime = mtime_secs(&current_meta);
                let current_size = current_meta.len();

                if guard.meta.mtime_secs == current_mtime && guard.meta.file_size == current_size {
                    // File unchanged – reuse cached version.
                    return Ok(Arc::clone(&entry));
                }
                // File changed on disk – fall through to reload.
            }
        }

        // Load from disk.
        let (file_source, size) = FileLoader::load_file(path.to_str().unwrap_or(""))
            .map_err(|e| format!("Failed to load file: {}", e))?;

        // For large files we delegate creation to from_mmap.
        let document = match &file_source {
            zaroxi_ops_file::file_loader::FileSource::Mmap(mmap) => {
                Document::from_mmap(mmap, canonical.to_string_lossy().to_string(), size)
            }
            zaroxi_ops_file::file_loader::FileSource::Memory(s) => {
                Document::from_text_with_path(s, canonical.to_string_lossy().to_string())
            }
        };

        let mtime = std::fs::metadata(&canonical)
            .ok()
            .map(|m| mtime_secs(&m))
            .unwrap_or(0);

        let cached = CachedDocument::new(document, size, mtime);

        // Store in cache and return a shared reference.
        let entry = {
            let mut cache = self.cache.lock();
            cache.insert(canonical.clone(), cached)
        };

        Ok(entry)
    }

    /// Retrieve a cached document without any disk I/O.
    /// Returns `None` if the document is not in the cache.
    pub async fn get_cached(&self, path: &Path) -> Option<Arc<Mutex<CachedDocument>>> {
        let canonical = path.canonicalize().ok()?;
        let mut cache = self.cache.lock();
        cache.get(&canonical)
    }

    /// Mark a document as dirty (unsaved changes).
    pub async fn mark_dirty(&self, path: &Path) {
        let canonical = path.canonicalize().ok();
        if let Some(canonical) = canonical {
            let mut cache = self.cache.lock();
            if let Some(entry) = cache.get(&canonical) {
                let mut guard = entry.lock();
                guard.meta.is_dirty = true;
                guard.meta.version += 1;
            }
        }
    }

    /// Mark a document as clean (after saving).
    pub async fn mark_clean(&self, path: &Path) {
        let canonical = path.canonicalize().ok();
        if let Some(canonical) = canonical {
            let mut cache = self.cache.lock();
            if let Some(entry) = cache.get(&canonical) {
                let mut guard = entry.lock();
                guard.meta.is_dirty = false;
                guard.meta.version += 1;
            }
        }
    }

    /// Remove a document from the cache (e.g., when the tab is closed).
    pub async fn close_document(&self, path: &Path) {
        let canonical = path.canonicalize().ok();
        if let Some(canonical) = canonical {
            let mut cache = self.cache.lock();
            cache.remove(&canonical);
        }
    }

    /// Return the number of cached documents.
    pub async fn cache_size(&self) -> usize {
        let cache = self.cache.lock();
        cache.len()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mtime_secs(metadata: &std::fs::Metadata) -> u64 {
    use std::time::UNIX_EPOCH;
    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
