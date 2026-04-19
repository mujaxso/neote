use std::sync::Arc;
use parking_lot::RwLock;
use crate::services::{WorkspaceService, EditorService};

#[derive(Default)]
pub struct AppState {
    workspace_service: RwLock<Option<Arc<WorkspaceService>>>,
    editor_service: RwLock<Option<Arc<EditorService>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn initialize(&self) -> Result<(), String> {
        // Initialize services
        let workspace_service = Arc::new(WorkspaceService::new());
        let editor_service = Arc::new(EditorService::new());
        
        *self.workspace_service.write() = Some(workspace_service);
        *self.editor_service.write() = Some(editor_service);
        
        Ok(())
    }
    
    pub fn workspace_service(&self) -> Result<Arc<WorkspaceService>, String> {
        self.workspace_service
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| "Workspace service not initialized".to_string())
    }
    
    pub fn editor_service(&self) -> Result<Arc<EditorService>, String> {
        self.editor_service
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| "Editor service not initialized".to_string())
    }
}
