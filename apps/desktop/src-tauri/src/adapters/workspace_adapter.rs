use crate::services::workspace_service::FileEntry;

/// Convert domain workspace to DTO
pub fn domain_workspace_to_dto(workspace: &zaroxi_domain_workspace::workspace::Workspace) -> crate::commands::workspace::OpenWorkspaceResponse {
    crate::commands::workspace::OpenWorkspaceResponse {
        workspace_id: workspace.id.to_string(),
        root_path: workspace.root_path.clone(),
        file_count: 0, // TODO: Implement actual file count
    }
}

/// Convert file entry to DTO
pub fn file_entry_to_dto(entry: &FileEntry) -> crate::commands::workspace::DirectoryEntryDto {
    crate::commands::workspace::DirectoryEntryDto {
        path: entry.path.clone(),
        name: entry.name.clone(),
        is_dir: entry.is_dir,
        file_type: entry.file_type.clone(),
    }
}
