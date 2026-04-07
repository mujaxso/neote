mod app;
mod bootstrap;
mod commands;
mod ui;

use std::env;
use std::sync::{Arc, Mutex};

use eframe::egui;
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;

struct NeoteApp {
    workspace_path: String,
    workspace_state: Arc<Mutex<WorkspaceState>>,
    file_entries: Vec<core_types::workspace::DirectoryEntry>,
    selected_file_index: Option<usize>,
    editor_text: String,
    dirty: bool,
    sidebar: crate::ui::sidebar::Sidebar,
    pending_sidebar_events: Vec<crate::events::SidebarEvent>,
}

impl NeoteApp {
    fn new(workspace_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let entries = files::list_directory(&workspace_path)?;
        let workspace_state = Arc::new(Mutex::new(WorkspaceState::new(&workspace_path)));
        workspace_state.lock().unwrap().set_file_tree(entries.clone());
        
        Ok(Self {
            workspace_path,
            workspace_state,
            file_entries: entries,
            selected_file_index: None,
            editor_text: String::new(),
            dirty: false,
            sidebar: crate::ui::sidebar::Sidebar::default(),
            pending_sidebar_events: Vec::new(),
        })
    }

    fn empty() -> Self {
        Self {
            workspace_path: String::new(),
            workspace_state: Arc::new(Mutex::new(WorkspaceState::new(""))),
            file_entries: Vec::new(),
            selected_file_index: None,
            editor_text: String::new(),
            dirty: false,
            sidebar: crate::ui::sidebar::Sidebar::default(),
            pending_sidebar_events: Vec::new(),
        }
    }

    fn open_workspace(&mut self, path: String) -> Result<(), String> {
        match files::list_directory(&path) {
            Ok(entries) => {
                self.workspace_path = path.clone();
                let mut state = self.workspace_state.lock().unwrap();
                state.set_workspace_root(&path);
                state.set_file_tree(entries.clone());
                self.file_entries = entries;
                Ok(())
            }
            Err(e) => Err(format!("Failed to open workspace: {}", e)),
        }
    }

    fn create_file(&mut self, path: String) -> Result<(), String> {
        // Ensure the path is within the workspace
        if !self.workspace_path.is_empty() && path.starts_with(&self.workspace_path) {
            // Create parent directories if they don't exist
            if let Some(parent) = std::path::Path::new(&path).parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create directories: {}", e))?;
                }
            }
            match files::write_file(&path, "") {
                Ok(_) => {
                    // Refresh the file list
                    if !self.workspace_path.is_empty() {
                        match files::list_directory(&self.workspace_path) {
                            Ok(entries) => {
                                self.file_entries = entries;
                                let mut state = self.workspace_state.lock().unwrap();
                                state.set_file_tree(self.file_entries.clone());
                            }
                            Err(e) => return Err(format!("Failed to refresh after creating file: {}", e)),
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Failed to create file: {}", e)),
            }
        } else {
            Err("File must be within the workspace".to_string())
        }
    }

    fn delete_file(&mut self, path: String) -> Result<(), String> {
        use std::fs;
        
        if !self.workspace_path.is_empty() && path.starts_with(&self.workspace_path) {
            match fs::remove_file(&path) {
                Ok(_) => {
                    // Refresh the file list
                    if !self.workspace_path.is_empty() {
                        match files::list_directory(&self.workspace_path) {
                            Ok(entries) => {
                                self.file_entries = entries;
                                let mut state = self.workspace_state.lock().unwrap();
                                state.set_file_tree(self.file_entries.clone());
                            }
                            Err(e) => return Err(format!("Failed to refresh after deleting file: {}", e)),
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Failed to delete file: {}", e)),
            }
        } else {
            Err("File must be within the workspace".to_string())
        }
    }

    fn open_file(&mut self, index: usize) {
        if index < self.file_entries.len() {
            let entry = &self.file_entries[index];
            if !entry.is_dir {
                match files::read_file(&entry.path) {
                    Ok(content) => {
                        let mut state = self.workspace_state.lock().unwrap();
                        state.open_buffer(&entry.path, content.clone());
                        self.editor_text = content;
                        self.selected_file_index = Some(index);
                        self.dirty = false;
                    }
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                    }
                }
            }
        }
    }

    fn process_sidebar_events(&mut self) {
        use crate::events::SidebarEvent;
        
        // Take all pending events to avoid borrowing issues
        let events = std::mem::take(&mut self.pending_sidebar_events);
        
        for event in events {
            match event {
                SidebarEvent::OpenWorkspace => {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        let path_str = path.to_string_lossy().to_string();
                        if let Err(e) = self.open_workspace(path_str) {
                            eprintln!("Failed to open workspace: {}", e);
                        }
                    }
                }
                SidebarEvent::CreateFile => {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        let path_str = path.to_string_lossy().to_string();
                        if let Err(e) = self.create_file(path_str) {
                            eprintln!("Failed to create file: {}", e);
                        }
                    }
                }
                SidebarEvent::DeleteFile => {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path_str = path.to_string_lossy().to_string();
                        if let Err(e) = self.delete_file(path_str) {
                            eprintln!("Failed to delete file: {}", e);
                        }
                    }
                }
                SidebarEvent::OpenFile => {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let path_str = path.to_string_lossy().to_string();
                        // Find the index of this file in file_entries
                        if let Some(index) = self.file_entries.iter().position(|entry| entry.path == path_str) {
                            self.open_file(index);
                        } else {
                            // If not in current list, open it directly
                            match files::read_file(&path_str) {
                                Ok(content) => {
                                    let mut state = self.workspace_state.lock().unwrap();
                                    state.open_buffer(&path_str, content.clone());
                                    self.editor_text = content;
                                    self.dirty = false;
                                    // If we have a workspace, refresh the file list to include the new file
                                    if !self.workspace_path.is_empty() {
                                        // Check if the file is within the workspace
                                        if path_str.starts_with(&self.workspace_path) {
                                            if let Ok(entries) = files::list_directory(&self.workspace_path) {
                                                self.file_entries = entries;
                                                state.set_file_tree(self.file_entries.clone());
                                            }
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Failed to read file: {}", e),
                            }
                        }
                    }
                }
            }
        }
    }

    fn save_current_file(&mut self) {
        let mut state = self.workspace_state.lock().unwrap();
        if let Some((path, _)) = state.save_active_buffer() {
            match files::write_file(&path.to_string_lossy(), &self.editor_text) {
                Ok(_) => {
                    self.dirty = false;
                }
                Err(e) => {
                    eprintln!("Failed to save file: {}", e);
                }
            }
        }
    }

    fn handle_sidebar_actions(&mut self) {
        // Handle open workspace dialog
        if self.sidebar.open_workspace_dialog {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                let path_str = path.to_string_lossy().to_string();
                if let Err(e) = self.open_workspace(path_str) {
                    eprintln!("Failed to open workspace: {}", e);
                }
                self.sidebar.open_workspace_dialog = false;
            } else {
                self.sidebar.open_workspace_dialog = false;
            }
        }

        // Handle create file dialog
        if self.sidebar.create_file_dialog {
            if let Some(path) = rfd::FileDialog::new().save_file() {
                let path_str = path.to_string_lossy().to_string();
                if let Err(e) = self.create_file(path_str) {
                    eprintln!("Failed to create file: {}", e);
                }
                self.sidebar.create_file_dialog = false;
                self.sidebar.create_file_path_input.clear();
            } else {
                self.sidebar.create_file_dialog = false;
                self.sidebar.create_file_path_input.clear();
            }
        }

        // Handle delete file dialog
        if self.sidebar.delete_file_dialog {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                let path_str = path.to_string_lossy().to_string();
                if let Err(e) = self.delete_file(path_str) {
                    eprintln!("Failed to delete file: {}", e);
                }
                self.sidebar.delete_file_dialog = false;
                self.sidebar.delete_file_path_input.clear();
            } else {
                self.sidebar.delete_file_dialog = false;
                self.sidebar.delete_file_path_input.clear();
            }
        }

        // Handle open file dialog
        if self.sidebar.open_file_dialog {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                let path_str = path.to_string_lossy().to_string();
                // Find the index of this file in file_entries
                if let Some(index) = self.file_entries.iter().position(|entry| entry.path == path_str) {
                    self.open_file(index);
                } else {
                    // If not in current list, open it directly
                    match files::read_file(&path_str) {
                        Ok(content) => {
                            let mut state = self.workspace_state.lock().unwrap();
                            state.open_buffer(&path_str, content.clone());
                            self.editor_text = content;
                            self.dirty = false;
                            // If we have a workspace, refresh the file list to include the new file
                            if !self.workspace_path.is_empty() {
                                // Check if the file is within the workspace
                                if path_str.starts_with(&self.workspace_path) {
                                    if let Ok(entries) = files::list_directory(&self.workspace_path) {
                                        self.file_entries = entries;
                                        state.set_file_tree(self.file_entries.clone());
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to read file: {}", e),
                    }
                }
                self.sidebar.open_file_dialog = false;
                self.sidebar.open_file_path_input.clear();
            } else {
                self.sidebar.open_file_dialog = false;
                self.sidebar.open_file_path_input.clear();
            }
        }
    }
}

impl eframe::App for NeoteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle sidebar actions (file dialogs, etc.)
        self.handle_sidebar_actions();
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Neote");
                ui.separator();
                ui.label(format!("Workspace: {}", if self.workspace_path.is_empty() { "None" } else { &self.workspace_path }));
                if ui.button("Refresh").clicked() && !self.workspace_path.is_empty() {
                    match files::list_directory(&self.workspace_path) {
                        Ok(entries) => {
                            self.file_entries = entries;
                            self.workspace_state.lock().unwrap().set_file_tree(self.file_entries.clone());
                        }
                        Err(e) => {
                            eprintln!("Failed to refresh directory: {}", e);
                        }
                    }
                }
            });
        });

        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            self.sidebar.ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.workspace_path.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.heading("Welcome to Neote");
                    ui.label("Please open a workspace to get started.");
                    if ui.button("Open Workspace").clicked() {
                        // This will be handled by the sidebar
                    }
                });
            } else {
                ui.columns(2, |columns| {
                    // Left column: file list
                    columns[0].vertical(|ui| {
                        ui.heading("Files");
                        let mut file_to_open: Option<usize> = None;
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, entry) in self.file_entries.iter().enumerate() {
                                let label = if entry.is_dir {
                                    format!("📁 {}", entry.name)
                                } else {
                                    format!("📄 {}", entry.name)
                                };
                                if ui.selectable_label(self.selected_file_index == Some(i), label).clicked() && !entry.is_dir {
                                    file_to_open = Some(i);
                                }
                            }
                        });
                        if let Some(index) = file_to_open {
                            self.open_file(index);
                        }
                    });

                    // Right column: editor
                    columns[1].vertical(|ui| {
                        ui.horizontal(|ui| {
                            if let Some(index) = self.selected_file_index {
                                let entry = &self.file_entries[index];
                                ui.heading(&entry.name);
                                if self.dirty {
                                    ui.label("(modified)");
                                }
                            } else {
                                ui.heading("No file selected");
                            }
                            
                            if ui.button("Save").clicked() {
                                self.save_current_file();
                            }
                        });
                        
                        ui.separator();
                        
                        let mut state = self.workspace_state.lock().unwrap();
                        if let Some(buffer) = state.active_buffer_mut() {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.editor_text)
                                    .desired_rows(20)
                                    .desired_width(f32::INFINITY)
                            );
                            
                            if response.changed() {
                                buffer.replace_all(self.editor_text.clone());
                                self.dirty = buffer.is_dirty();
                            }
                        } else {
                            ui.label("Select a file to edit");
                        }
                    });
                });
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status:");
                if self.workspace_path.is_empty() {
                    ui.label("No workspace open");
                } else if self.dirty {
                    ui.label("File has unsaved changes");
                } else {
                    ui.label("All changes saved");
                }
            });
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force X11 backend to avoid Wayland issues
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    
    let args: Vec<String> = env::args().collect();
    let initial_workspace_path = if args.len() >= 2 {
        Some(args[1].clone())
    } else {
        None
    };
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Neote"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Neote",
        options,
        Box::new(|_cc| {
            match initial_workspace_path {
                Some(path) => {
                    match NeoteApp::new(path) {
                        Ok(app) => Box::new(app),
                        Err(e) => {
                            eprintln!("Failed to initialize app with workspace: {}", e);
                            // Start with empty app
                            Box::new(NeoteApp::empty())
                        }
                    }
                }
                None => Box::new(NeoteApp::empty()),
            }
        }),
    ).map_err(|e| e.into())
}
