use std::path::Path;
use std::fs;
use thiserror::Error;
use core_types::workspace::DirectoryEntry;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path does not exist: {0}")]
    NotFound(String),
    #[error("Path is not a directory: {0}")]
    NotDirectory(String),
}

pub struct WorkspaceLoader;

impl WorkspaceLoader {
    pub fn list_directory(path: &str) -> Result<Vec<DirectoryEntry>, WorkspaceError> {
        let dir_path = Path::new(path);
        if !dir_path.exists() {
            return Err(WorkspaceError::NotFound(path.to_string()));
        }
        if !dir_path.is_dir() {
            return Err(WorkspaceError::NotDirectory(path.to_string()));
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            
            let is_dir = path.is_dir();
            
            entries.push(DirectoryEntry {
                path: path.to_string_lossy().to_string(),
                name,
                is_dir,
            });
        }
        
        Ok(entries)
    }
}
