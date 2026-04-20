//! Workspace service implementation.

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// Workspace service for handling workspace operations.
pub struct WorkspaceService {
    /// Internal state.
    state: Arc<Mutex<WorkspaceServiceState>>,
}

struct WorkspaceServiceState {
    /// Whether the service is running.
    running: bool,
}

impl WorkspaceService {
    /// Create a new workspace service.
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(WorkspaceServiceState { running: false })),
        }
    }

    /// Start the workspace service.
    pub async fn start(&self) -> Result<(), anyhow::Error> {
        let mut state = self.state.lock().await;
        if state.running {
            return Err(anyhow::anyhow!("Workspace service is already running"));
        }
        state.running = true;
        info!("Workspace service started");
        Ok(())
    }

    /// Stop the workspace service.
    pub async fn stop(&self) -> Result<(), anyhow::Error> {
        let mut state = self.state.lock().await;
        if !state.running {
            return Err(anyhow::anyhow!("Workspace service is not running"));
        }
        state.running = false;
        info!("Workspace service stopped");
        Ok(())
    }

    /// Check if the service is running.
    pub async fn is_running(&self) -> bool {
        let state = self.state.lock().await;
        state.running
    }
}
