use crate::message::Message;
use crate::state::{App, FileLoadingState, FileMetadata};
use crate::update::dialog;
use file_ops::{FileLoader, WorkspaceLoader};
use iced::Command;
use tokio::time;
use editor_core::Document;

// File size thresholds for tiered handling
const SYNTAX_HIGHLIGHT_THRESHOLD: u64 = 2 * 1024 * 1024; // 2MB - disable syntax highlighting above this
const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB - reduce features but keep editing
const VERY_LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100MB - read-only mode

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::WorkspacePathChanged(path) => {
            app.workspace_path = path;
            Command::none()
        }
        Message::OpenWorkspace => {
            dialog::open_workspace_dialog()
        }
        Message::WorkspaceLoaded(result) => {
            handle_workspace_loaded(app, result)
        }
        Message::FileSelected(index) => {
            handle_file_selected(app, index)
        }
        Message::FileSelectedByPath(path) => {
            handle_file_selected_by_path(app, path)
        }
        Message::FileMetadataLoaded(result) => {
            handle_file_metadata_loaded(app, result)
        }
        Message::ConfirmOpenLargeFile(path, size) => {
            handle_confirm_open_large_file(app, path, size)
        }
        Message::OpenLargeFileReadOnly(path) => {
            handle_open_large_file_read_only(app, path)
        }
        Message::FileLoaded(result) => {
            handle_file_loaded(app, result)
        }
        Message::SaveFile => {
            handle_save_file(app)
        }
        Message::FileSaved(result) => {
            handle_file_saved(app, result)
        }
        Message::RefreshWorkspace => {
            handle_refresh_workspace(app)
        }
        Message::SubmitManualWorkspacePath(path) => {
            handle_submit_manual_workspace_path(app, path)
        }
        Message::WorkspaceDialogCancelled => {
            app.status_message = "Workspace selection cancelled".to_string();
            Command::none()
        }
        _ => Command::none(),
    }
}

fn handle_workspace_loaded(app: &mut App, result: Result<(String, Vec<core_types::workspace::DirectoryEntry>), String>) -> Command<Message> {
    match result {
        Ok((path, entries)) => {
            app.workspace_path = path.clone();
            app.file_entries = entries.clone();
            app.status_message = format!("Workspace loaded: {} files", app.file_entries.len());
            app.error_message = None;
            
            // Update explorer state
            app.explorer_state.set_workspace_root(std::path::PathBuf::from(&app.workspace_path));
            app.explorer_state.set_file_tree(entries);
            
            let mut state = app.workspace_state.lock().unwrap();
            state.set_workspace_root(&app.workspace_path);
            state.set_file_tree(app.file_entries.clone());
        }
        Err(e) => {
            app.error_message = Some(e);
            app.status_message = "Failed to load workspace".to_string();
        }
    }
    Command::none()
}

fn handle_file_selected(app: &mut App, index: usize) -> Command<Message> {
    if index < app.file_entries.len() {
        let entry = &app.file_entries[index];
        // Only handle files, not directories
        if !entry.is_dir {
            let path = entry.path.clone();
            // Start by loading metadata first
            app.file_loading_state = FileLoadingState::LoadingMetadata { 
                path: path.clone() 
            };
            app.status_message = format!("Checking {}...", entry.name);
            app.error_message = None;
            app.is_file_read_only = false;
            app.is_file_too_large_for_editor = false;
            
            // Clear syntax highlight cache for previous file
            app.syntax_highlight_cache.clear();
            app.syntax_highlight_spans.clear();
            app.syntax_highlight_span_count = 0;
            app.syntax_cache_version += 1;
            // Clear text editor content
            app.text_editor = iced::widget::text_editor::Content::new();
            
            // Load metadata using the file-ops crate
            Command::perform(
                async move {
                    match FileLoader::load_metadata(&path) {
                        Ok(metadata) => Message::FileMetadataLoaded(Ok(FileMetadata {
                            path: metadata.path,
                            size: metadata.size,
                        })),
                        Err(e) => Message::FileMetadataLoaded(Err(format!("Failed to load metadata: {}", e))),
                    }
                },
                |result| result,
            )
        } else {
            // Directories are handled by Message::ToggleDirectory
            Command::none()
        }
    } else {
        Command::none()
    }
}

fn handle_file_selected_by_path(app: &mut App, path: String) -> Command<Message> {
    // Find the index of the file in file_entries
    let index = app.file_entries.iter().position(|entry| entry.path == path);
    if let Some(index) = index {
        handle_file_selected(app, index)
    } else {
        Command::none()
    }
}

fn handle_file_metadata_loaded(app: &mut App, result: Result<FileMetadata, String>) -> Command<Message> {
    match result {
        Ok(metadata) => {
            
            // Always proceed to load the file, but track which mode we're in
            if metadata.size > VERY_LARGE_FILE_THRESHOLD {
                app.file_loading_state = FileLoadingState::VeryLargeFileWarning {
                    path: metadata.path.clone(),
                    size: metadata.size,
                };
                app.status_message = format!("Very large file detected ({} MB) - opening in read-only mode", 
                                           metadata.size / (1024 * 1024));
                // For very large files, open in read-only mode
                return Command::perform(
                    async move {
                        // Small delay to show the warning
                        time::sleep(std::time::Duration::from_millis(100)).await;
                        Message::OpenLargeFileReadOnly(metadata.path)
                    },
                    |msg| msg,
                );
            } else if metadata.size > LARGE_FILE_THRESHOLD {
                app.file_loading_state = FileLoadingState::LargeFileWarning {
                    path: metadata.path.clone(),
                    size: metadata.size,
                };
                app.status_message = format!("Large file detected ({} MB) - editing enabled, some features reduced", 
                                           metadata.size / (1024 * 1024));
                // For large files, still load normally but we'll disable expensive features
                return Command::perform(
                    async move {
                        Message::ConfirmOpenLargeFile(metadata.path, metadata.size)
                    },
                    |msg| msg,
                );
            } else {
                // Normal file, proceed with full features
                app.file_loading_state = FileLoadingState::LoadingContent {
                    path: metadata.path.clone(),
                    size: metadata.size,
                };
                app.status_message = format!("Loading file...");
                
                return Command::perform(
                    async move {
                        let path = metadata.path;
                        match FileLoader::load_file(&path) {
                            Ok((content, _)) => {
                                // Create a Document from the content
                                let document = Document::from_text_with_path(&content, path.clone());
                                Message::FileLoaded(Ok((path, content, document)))
                            }
                            Err(e) => Message::FileLoaded(Err(format!("Failed to load file: {}", e))),
                        }
                    },
                    |result| result,
                );
            }
        }
        Err(e) => {
            app.file_loading_state = FileLoadingState::Idle;
            app.error_message = Some(e);
            app.status_message = "Failed to load file metadata".to_string();
        }
    }
    Command::none()
}

fn handle_confirm_open_large_file(app: &mut App, path: String, size: u64) -> Command<Message> {
    app.file_loading_state = FileLoadingState::LoadingContent {
        path: path.clone(),
        size,
    };
    app.status_message = format!("Loading large file...");
    
    Command::perform(
        async move {
            match FileLoader::load_file(&path) {
                Ok((content, _)) => {
                    let document = Document::from_text_with_path(&content, path.clone());
                    Message::FileLoaded(Ok((path, content, document)))
                }
                Err(e) => Message::FileLoaded(Err(format!("Failed to load file: {}", e))),
            }
        },
        |result| result,
    )
}

fn handle_open_large_file_read_only(app: &mut App, path: String) -> Command<Message> {
    app.file_loading_state = FileLoadingState::ReadOnlyPreview {
        path: path.clone(),
        size: 0, // We'll get this from metadata
    };
    app.status_message = format!("Opening in read-only mode...");
    app.active_file_path = Some(path.clone());
    app.is_file_too_large_for_editor = true;
    app.is_file_read_only = true;
    
    // For very large files, only load a preview
    Command::perform(
        async move {
            match FileLoader::load_file_preview(&path, 100 * 1024) {
                Ok((content, _)) => {
                    let document = Document::from_text_with_path(&content, path.clone());
                    Message::FileLoaded(Ok((path, content, document)))
                }
                Err(e) => Message::FileLoaded(Err(format!("Failed to load file preview: {}", e))),
            }
        },
        |result| result,
    )
}

fn handle_file_loaded(app: &mut App, result: Result<(String, String, Document), String>) -> Command<Message> {
    match result {
        Ok((path, content, document)) => {
            app.active_file_path = Some(path.clone());
            // Create editor state from document
            let editor_state = editor_core::EditorState::from_document(document);
            app.editor_state = Some(editor_state);
            app.file_loading_state = FileLoadingState::Idle;
            
            let file_size_bytes = content.len();
            
            // Determine file mode based on size
            let is_very_large = file_size_bytes > VERY_LARGE_FILE_THRESHOLD as usize;
            let is_large = file_size_bytes > LARGE_FILE_THRESHOLD as usize;
            let needs_syntax_highlight = file_size_bytes <= SYNTAX_HIGHLIGHT_THRESHOLD as usize;
            
            // Update syntax highlighting flag and clear cache if disabled
            app.syntax_highlighting_enabled = needs_syntax_highlight;
            if !needs_syntax_highlight {
                app.syntax_highlight_cache.clear();
                app.syntax_highlight_spans.clear();
                app.syntax_highlight_span_count = 0;
                app.syntax_cache_version += 1;
            }
            
            // Handle based on size tier
            if is_very_large {
                // Very large files: read-only mode with limited preview
                app.is_file_read_only = true;
                app.is_file_too_large_for_editor = true;
                app.status_message = format!(
                    "Very large file opened in read-only preview ({} MB total, showing first 100KB)",
                    file_size_bytes / (1024 * 1024)
                );
                
                // Limit preview content
                let preview_content = if content.len() > 100_000 {
                    &content[..100_000]
                } else {
                    &content
                };
                app.text_editor = iced::widget::text_editor::Content::with_text(&format!(
                    "// Read-only preview (first {} bytes)\n// File is very large ({} MB total)\n\n{}",
                    preview_content.len(),
                    file_size_bytes / (1024 * 1024),
                    preview_content
                ));
            } else {
                // Large or normal files: editing enabled
                app.is_file_read_only = false;
                app.is_file_too_large_for_editor = false;
                
                if is_large {
                    app.status_message = format!(
                        "Large file opened ({} MB) - editing enabled, syntax highlighting disabled",
                        file_size_bytes / (1024 * 1024)
                    );
                } else if !needs_syntax_highlight {
                    app.status_message = format!(
                        "File opened ({} MB) - syntax highlighting disabled for performance",
                        file_size_bytes / (1024 * 1024)
                    );
                } else {
                    app.status_message = format!("File loaded: {} ({} bytes)", path, file_size_bytes);
                }
                
                // Initialize editor content with full text for editable files
                if let Some(ref editor_state) = app.editor_state {
                    let text = editor_state.document().text();
                    app.text_editor = iced::widget::text_editor::Content::with_text(&text);
                }
            }
            
            app.error_message = None;
            app.is_dirty = false;
            
            // Update workspace state
            {
                let content_clone = content.clone();
                let mut state = app.workspace_state.lock().unwrap();
                state.open_buffer(&path, content_clone);
            }
            
            // Send EditorSetDocument to trigger syntax highlighting only for appropriate files
            if needs_syntax_highlight {
                Command::perform(
                    async move {
                        Message::EditorSetDocument(editor_core::Document::from_text_with_path(&content, path))
                    },
                    |msg| msg,
                )
            } else {
                // Don't trigger syntax highlighting for large files
                Command::none()
            }
        }
        Err(e) => {
            app.file_loading_state = FileLoadingState::Idle;
            app.is_file_read_only = false;
            app.is_file_too_large_for_editor = false;
            app.error_message = Some(e);
            app.status_message = "Failed to load file".to_string();
            Command::none()
        }
    }
}

fn handle_save_file(app: &mut App) -> Command<Message> {
    if let Some(path) = &app.active_file_path {
        if let Some(ref editor_state) = app.editor_state {
            let content = editor_state.document().text();
            let path_clone = path.clone();
            let content_clone = content.clone();
            
            Command::perform(
                async move {
                    // Use the file-ops crate to save the file
                    match WorkspaceLoader::save_file(&path_clone, &content_clone) {
                        Ok(_) => Message::FileSaved(Ok(())),
                        Err(e) => Message::FileSaved(Err(format!("Failed to save file: {}", e))),
                    }
                },
                |result| result,
            )
        } else {
            app.status_message = "No buffer to save".to_string();
            Command::none()
        }
    } else {
        app.status_message = "No file selected to save".to_string();
        Command::none()
    }
}

fn handle_file_saved(app: &mut App, result: Result<(), String>) -> Command<Message> {
    match result {
        Ok(_) => {
            if let Some(editor_state) = &mut app.editor_state {
                editor_state.document_mut().mark_saved();
                app.is_dirty = editor_state.document().is_dirty();
            }
            app.status_message = "File saved successfully".to_string();
            app.error_message = None;
        }
        Err(e) => {
            let error_msg = e.clone();
            app.error_message = Some(e);
            app.status_message = format!("Failed to save file: {}", error_msg);
        }
    }
    Command::none()
}

fn handle_refresh_workspace(app: &mut App) -> Command<Message> {
    if !app.workspace_path.is_empty() {
        let path = app.workspace_path.clone();
        Command::perform(
            async move {
                match WorkspaceLoader::list_directory(&path) {
                    Ok(entries) => Message::WorkspaceLoaded(Ok((path, entries))),
                    Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                }
            },
            |result| result,
        )
    } else {
        Command::none()
    }
}

fn handle_submit_manual_workspace_path(app: &mut App, path: String) -> Command<Message> {
    if path.is_empty() {
        app.status_message = "Please enter a workspace path".to_string();
        Command::none()
    } else {
        // Check if the path exists
        let path_buf = std::path::PathBuf::from(&path);
        if !path_buf.exists() {
            app.error_message = Some(format!("Path does not exist: {}", path));
            app.status_message = "Invalid path".to_string();
            return Command::none();
        }
        if !path_buf.is_dir() {
            app.error_message = Some(format!("Path is not a directory: {}", path));
            app.status_message = "Path must be a directory".to_string();
            return Command::none();
        }
        
        // Load the workspace from the manually entered path
        let path_clone = path.clone();
        Command::perform(
            async move {
                match WorkspaceLoader::list_directory(&path_clone) {
                    Ok(entries) => {
                        Message::WorkspaceLoaded(Ok((path_clone, entries)))
                    },
                    Err(e) => {
                        Message::WorkspaceLoaded(Err(format!("Manual workspace load failed: {}", e)))
                    },
                }
            },
            |result| result,
        )
    }
}
