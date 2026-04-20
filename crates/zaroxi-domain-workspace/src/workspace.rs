//! Workspace domain model.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A workspace represents a directory on disk that contains source code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique identifier for this workspace.
    pub id: Uuid,
    /// The root path of the workspace on the filesystem.
    pub root_path: String,
    /// The name of the workspace (derived from the directory name).
    pub name: String,
    /// Whether the workspace is currently open.
    pub is_open: bool,
    /// Timestamp when the workspace was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the workspace was last accessed.
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
}

impl Workspace {
    /// Create a new workspace from a root path.
    pub fn new(root_path: String) -> Self {
        let name = std::path::Path::new(&root_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            root_path,
            name,
            is_open: true,
            created_at: now,
            last_accessed_at: now,
        }
    }

    /// Update the last accessed timestamp to now.
    pub fn touch(&mut self) {
        self.last_accessed_at = chrono::Utc::now();
    }
}
