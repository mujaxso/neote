//! Download and compile Tree-sitter grammars.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::runtime::Runtime;
use super::grammar_registry::GrammarInfo;

/// Build a Tree-sitter grammar and install it to the runtime directory
pub fn build_and_install_grammar(language_id: &str) -> Result<(), String> {
    let grammar_info = GrammarInfo::for_language(language_id)
        .ok_or_else(|| format!("No grammar info available for {}", language_id))?;
    
    let runtime = Runtime::new();
    
    // Create temporary directory
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Download source as zip instead of using git clone
    println!("Downloading {} grammar source...", language_id);
    let repo_dir = temp_dir.path().join("repo");
    
    // Create repo directory
    fs::create_dir_all(&repo_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", repo_dir.display(), e))?;
    
    // Use git clone with timeout and no credential helper
    println!("Cloning {}...", grammar_info.repo_url);
    
    // Set up git command with timeout and no credential helper
    let mut cmd = Command::new("timeout");
    cmd.args(["30", "git", "clone", "--depth", "1"]);
    
    // Disable credential helper to avoid prompts
    cmd.args(["--config", "credential.helper="]);
    
    cmd.args([&grammar_info.repo_url, repo_dir.to_str().unwrap()]);
    
    match cmd.status() {
        Ok(status) if status.success() => {
            println!("Successfully cloned repository");
        }
        Ok(_) => {
            // Try without timeout command (for systems without timeout)
            let mut cmd2 = Command::new("git");
            cmd2.args(["clone", "--depth", "1", "--config", "credential.helper=", &grammar_info.repo_url, repo_dir.to_str().unwrap()]);
            
            match cmd2.status() {
                Ok(status2) if status2.success() => {
                    println!("Successfully cloned repository");
                }
                Ok(_) => {
                    return Err("Failed to clone repository (git clone failed)".to_string());
                }
                Err(e) => {
                    return Err(format!("Failed to run git clone: {}", e));
                }
            }
        }
        Err(e) => {
            // timeout command not available, try git directly
            let mut cmd2 = Command::new("git");
            cmd2.args(["clone", "--depth", "1", "--config", "credential.helper=", &grammar_info.repo_url, repo_dir.to_str().unwrap()]);
            
            match cmd2.status() {
                Ok(status2) if status2.success() => {
                    println!("Successfully cloned repository");
                }
                Ok(_) => {
                    return Err("Failed to clone repository (git clone failed)".to_string());
                }
                Err(e2) => {
                    return Err(format!("Failed to run git clone: {}", e2));
                }
            }
        }
    }
    
    // No zip extraction needed - we cloned directly into repo_dir
    
    // We cloned directly into repo_dir, so source_dir is repo_dir
    // Navigate to subdirectory if needed
    let source_dir = if let Some(subdir) = &grammar_info.subdirectory {
        repo_dir.join(subdir)
    } else {
        repo_dir.clone()
    };
    
    // Verify source directory exists
    if !source_dir.exists() {
        return Err(format!("Source directory does not exist: {:?}", source_dir));
    }
    
    // Check if tree-sitter CLI is available
    let has_tree_sitter_cli = Command::new("tree-sitter")
        .arg("--version")
        .output()
        .is_ok();
    
    let lib_path = if has_tree_sitter_cli {
        // Use tree-sitter CLI
        println!("Using tree-sitter CLI to build {}...", language_id);
        
        let status = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("generate")
            .status()
            .map_err(|e| format!("Failed to run tree-sitter generate: {}", e))?;
        
        if !status.success() {
            return Err("tree-sitter generate failed".to_string());
        }
        
        let status = Command::new("tree-sitter")
            .current_dir(&source_dir)
            .arg("build")
            .status()
            .map_err(|e| format!("Failed to run tree-sitter build: {}", e))?;
        
        if !status.success() {
            return Err("tree-sitter build failed".to_string());
        }
        
        // Find the built library
        let target_dir = source_dir.join("target").join("release");
        let lib_name = get_library_name(language_id);
        
        if target_dir.join(&lib_name).exists() {
            target_dir.join(lib_name)
        } else if source_dir.join(&lib_name).exists() {
            source_dir.join(lib_name)
        } else {
            return Err(format!("Could not find built library {}", lib_name));
        }
    } else {
        // Manual compilation with cc
        println!("Using cc to build {}...", language_id);
        
        // Generate parser.c if needed
        if !source_dir.join("src/parser.c").exists() {
            if source_dir.join("grammar.js").exists() {
                // Try to use node with tree-sitter CLI package
                let status = Command::new("npx")
                    .current_dir(&source_dir)
                    .args(["tree-sitter", "generate"])
                    .status();
                
                if status.is_err() || !status.unwrap().success() {
                    return Err("Failed to generate parser.c. Install tree-sitter CLI or node.js".to_string());
                }
            } else {
                return Err("No grammar.js found and parser.c doesn't exist".to_string());
            }
        }
        
        // Compile all source files
        let mut object_files = Vec::new();
        for source_file in &grammar_info.source_files {
            let source_path = source_dir.join(source_file);
            if !source_path.exists() {
                println!("Warning: Source file {} does not exist, skipping", source_file);
                continue; // Skip missing files (some grammars don't have scanner.c)
            }
            
            let object_file = temp_dir.path().join(format!("{}.o", source_file.replace('/', "_")));
            
            println!("Compiling {}...", source_file);
            let output = Command::new("cc")
                .args(["-c", "-fPIC", "-I./src", "-o"])
                .arg(&object_file)
                .arg(&source_path)
                .current_dir(&source_dir)
                .output()
                .map_err(|e| format!("Failed to compile {}: {}", source_file, e))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to compile {}: {}", source_file, stderr));
            }
            
            object_files.push(object_file);
        }
        
        if object_files.is_empty() {
            return Err("No source files compiled".to_string());
        }
        
        // Link shared library
        let lib_name = get_library_name(language_id);
        let lib_path = temp_dir.path().join(&lib_name);
        
        let mut cmd = Command::new("cc");
        cmd.args(["-shared", "-fPIC", "-o"])
            .arg(&lib_path);
        
        for obj in &object_files {
            cmd.arg(obj);
        }
        
        cmd.arg("-lstdc++");
        
        let status = cmd.status()
            .map_err(|e| format!("Failed to link library: {}", e))?;
        
        if !status.success() {
            return Err("Failed to link shared library".to_string());
        }
        
        lib_path
    };
    
    // Install library to runtime directory
    let target_dir = runtime.grammar_dir();
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;
    
    let target_lib_path = target_dir.join(get_library_name(language_id));
    
    fs::copy(&lib_path, &target_lib_path)
        .map_err(|e| format!("Failed to copy library: {}", e))?;
    
    println!("Installed library to: {}", target_lib_path.display());
    
    // Install query files
    let query_source_dir = source_dir.join("queries");
    if query_source_dir.exists() {
        let query_target_dir = runtime.language_dir(language_id).join("queries");
        fs::create_dir_all(&query_target_dir)
            .map_err(|e| format!("Failed to create query directory: {}", e))?;
        
        for query_file in &grammar_info.query_files {
            let source_path = query_source_dir.join(query_file);
            if source_path.exists() {
                let target_path = query_target_dir.join(query_file);
                fs::copy(&source_path, &target_path)
                    .map_err(|e| format!("Failed to copy query file {}: {}", query_file, e))?;
                println!("Installed query file: {}", query_file);
            }
        }
    } else {
        println!("Warning: No query directory found for {}", language_id);
    }
    
    println!("Successfully installed {} grammar!", language_id);
    Ok(())
}

/// Download a file from a URL to a local path
fn download_file(url: &str, path: &std::path::Path) -> Result<(), String> {
    use std::io::{Read, Write};
    
    println!("Downloading {} to {}...", url, path.display());
    
    // First try using system commands (curl/wget) which might be more reliable
    if let Ok(()) = fallback_download(url, path) {
        println!("Download successful using system command");
        return Ok(());
    }
    
    println!("System command failed, trying ureq...");
    
    // Try using ureq as fallback
    let mut request = ureq::get(url)
        .timeout(std::time::Duration::from_secs(60));
    
    // Add User-Agent for GitHub API
    if url.contains("api.github.com") {
        request = request.set("User-Agent", "qyzer-studio/0.1.0");
    }
    
    let response = request.call();
    
    match response {
        Ok(resp) => {
            let mut file = std::fs::File::create(path)
                .map_err(|e| format!("Failed to create file {}: {}", path.display(), e))?;
            
            // Get content length for progress
            let content_length = resp.header("Content-Length")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(0);
            
            println!("Downloading {} bytes...", content_length);
            
            // Read response body into a Vec<u8> first
            let mut bytes: Vec<u8> = Vec::new();
            resp.into_reader().read_to_end(&mut bytes)
                .map_err(|e| format!("Failed to read response: {}", e))?;
            
            println!("Read {} bytes", bytes.len());
            
            // Write bytes to file
            std::io::Write::write_all(&mut file, &bytes)
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            
            println!("Download completed successfully");
            Ok(())
        }
        Err(ureq::Error::Status(code, resp)) => {
            let err = format!("HTTP error {}: {}", code, resp.status_text());
            println!("{}", err);
            Err(err)
        }
        Err(e) => {
            let err = format!("ureq download failed: {}", e);
            println!("{}", err);
            Err(err)
        }
    }
}

/// Fallback download using system commands
fn fallback_download(url: &str, path: &std::path::Path) -> Result<(), String> {
    println!("Trying to download using system command...");
    
    if cfg!(windows) {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Invoke-WebRequest -Uri '{}' -OutFile '{}'", url, path.display())])
            .output()
            .map_err(|e| format!("Failed to run powershell: {}", e))?;
        
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Powershell failed: {}", stderr))
        }
    } else {
        // Try curl first
        let mut curl_args = vec!["-L", "-f", "-o", path.to_str().unwrap(), url];
        // Add headers for GitHub API
        if url.contains("api.github.com") {
            curl_args = vec!["-L", "-f", "-H", "User-Agent: qyzer-studio/0.1.0", "-o", path.to_str().unwrap(), url];
        }
        
        let curl_result = Command::new("curl")
            .args(&curl_args)
            .output();
        
        match curl_result {
            Ok(output) if output.status.success() => {
                println!("curl download successful");
                Ok(())
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("curl failed: {}", stderr);
                
                // Try wget as fallback
                let wget_result = Command::new("wget")
                    .args(["-O", path.to_str().unwrap(), url])
                    .output();
                
                match wget_result {
                    Ok(output) if output.status.success() => {
                        println!("wget download successful");
                        Ok(())
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        Err(format!("wget failed: {}", stderr))
                    }
                    Err(e) => Err(format!("Failed to run wget: {}", e)),
                }
            }
            Err(e) => {
                println!("curl not available: {}", e);
                // Try wget as fallback
                let wget_result = Command::new("wget")
                    .args(["-O", path.to_str().unwrap(), url])
                    .output();
                
                match wget_result {
                    Ok(output) if output.status.success() => {
                        println!("wget download successful");
                        Ok(())
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        Err(format!("wget failed: {}", stderr))
                    }
                    Err(e) => Err(format!("Failed to run wget: {}", e)),
                }
            }
        }
    }
}

/// Get the platform-specific library name for a language
fn get_library_name(language_id: &str) -> String {
    let prefix = if cfg!(windows) { "" } else { "lib" };
    let extension = if cfg!(windows) {
        ".dll"
    } else if cfg!(target_os = "macos") {
        ".dylib"
    } else {
        ".so"
    };
    
    format!("{}tree-sitter-{}{}", prefix, language_id, extension)
}

/// Check if a grammar is installed
pub fn is_grammar_installed(language_id: &str) -> bool {
    let runtime = Runtime::new();
    let lib_path = runtime.grammar_library_path(language_id);
    lib_path.exists()
}

/// Install missing grammars for a list of languages
pub fn install_missing_grammars(language_ids: &[&str]) -> Vec<String> {
    let mut installed = Vec::new();
    
    for &language_id in language_ids {
        if !is_grammar_installed(language_id) {
            println!("Grammar for {} is not installed. Installing...", language_id);
            match build_and_install_grammar(language_id) {
                Ok(()) => {
                    installed.push(language_id.to_string());
                    println!("Successfully installed {} grammar", language_id);
                }
                Err(e) => {
                    eprintln!("Failed to install {} grammar: {}", language_id, e);
                }
            }
        } else {
            println!("Grammar for {} is already installed", language_id);
        }
    }
    
    installed
}
