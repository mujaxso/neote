mod app;
mod bootstrap;
mod commands;
mod ui;

use std::env;
use std::io::{self, Write};

use app::App;
use bootstrap::init;
use workspace_daemon::files;
use workspace_model::state::WorkspaceState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <workspace-path>", args[0]);
        return Ok(());
    }
    
    let workspace_path = &args[1];
    println!("Opening workspace at: {}", workspace_path);
    
    // List directory contents
    let entries = files::list_directory(workspace_path)?;
    println!("Found {} entries:", entries.len());
    for (i, entry) in entries.iter().enumerate() {
        let type_str = if entry.is_dir { "dir" } else { "file" };
        println!("  {}. {} ({})", i + 1, entry.name, type_str);
    }
    
    // Create workspace state
    let mut workspace_state = WorkspaceState::new(workspace_path);
    workspace_state.set_file_tree(entries);
    
    // Simple interactive loop
    loop {
        println!("\nOptions:");
        println!("  1. Open a file");
        println!("  2. Edit current file");
        println!("  3. Save current file");
        println!("  4. List files again");
        println!("  5. Exit");
        print!("Choose an option: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();
        
        match choice {
            "1" => {
                // Open a file
                println!("Enter file number to open:");
                for (i, entry) in workspace_state.file_tree().iter().enumerate() {
                    if !entry.is_dir {
                        println!("  {}. {}", i + 1, entry.name);
                    }
                }
                print!("File number: ");
                io::stdout().flush()?;
                
                let mut file_input = String::new();
                io::stdin().read_line(&mut file_input)?;
                if let Ok(file_num) = file_input.trim().parse::<usize>() {
                    let mut file_index = 0;
                    for (i, entry) in workspace_state.file_tree().iter().enumerate() {
                        if !entry.is_dir {
                            file_index += 1;
                            if file_index == file_num {
                                // Read the file
                                match files::read_file(&entry.path) {
                                    Ok(content) => {
                                        let buffer_id = workspace_state.open_buffer(&entry.path, content.clone());
                                        println!("Opened '{}' (buffer ID: {:?})", entry.name, buffer_id);
                                        println!("Content preview: {}", 
                                            if content.len() > 50 { 
                                                format!("{}...", &content[..50]) 
                                            } else { 
                                                content 
                                            }
                                        );
                                    }
                                    Err(e) => println!("Error reading file: {}", e),
                                }
                                break;
                            }
                        }
                    }
                }
            }
            "2" => {
                // Edit current file
                if let Some(buffer) = workspace_state.active_buffer_mut() {
                    println!("Current content:");
                    println!("{}", buffer.text());
                    println!("Enter new content (end with a line containing only 'EOF'):");
                    
                    let mut new_content = String::new();
                    loop {
                        let mut line = String::new();
                        io::stdin().read_line(&mut line)?;
                        if line.trim() == "EOF" {
                            break;
                        }
                        new_content.push_str(&line);
                    }
                    
                    buffer.replace_all(new_content.trim_end());
                    println!("Content updated. Dirty: {}", buffer.is_dirty());
                } else {
                    println!("No file is currently open. Please open a file first.");
                }
            }
            "3" => {
                // Save current file
                if let Some((path, content)) = workspace_state.save_active_buffer() {
                    match files::write_file(&path.to_string_lossy(), &content) {
                        Ok(_) => println!("File saved successfully."),
                        Err(e) => println!("Error saving file: {}", e),
                    }
                } else {
                    println!("No active buffer to save.");
                }
            }
            "4" => {
                // List files again
                let entries = files::list_directory(workspace_path)?;
                workspace_state.set_file_tree(entries.clone());
                println!("Files in workspace:");
                for (i, entry) in entries.iter().enumerate() {
                    let type_str = if entry.is_dir { "dir" } else { "file" };
                    println!("  {}. {} ({})", i + 1, entry.name, type_str);
                }
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
    
    Ok(())
}
