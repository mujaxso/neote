//! Download and compile Tree-sitter grammars.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::runtime::Runtime;
use crate::grammar_registry;

/// Try to locate tree-sitter include directory containing parser.h
fn find_tree_sitter_include_path() -> Result<String, String> {
    // Try system include paths
    let system_paths = [
        "/usr/include",
        "/usr/local/include",
        "/opt/homebrew/include",
        "/usr/local/opt/tree-sitter/include",
    ];
    for path in system_paths {
        let header_path = std::path::Path::new(path).join("tree_sitter/parser.h");
        if header_path.exists() {
            return Ok(path.to_string());
        }
    }
    
    // Try to find via cargo metadata
    if let Ok(output) = std::process::Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
    {
        if output.status.success() {
            let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
                .map_err(|e| format!("Failed to parse cargo metadata: {}", e))?;
            if let Some(packages) = metadata.get("packages").and_then(|p: &serde_json::Value| p.as_array()) {
                for package in packages {
                    if let Some(name) = package.get("name").and_then(|n: &serde_json::Value| n.as_str()) {
                        if name == "tree-sitter" {
                            if let Some(manifest_path) = package.get("manifest_path").and_then(|m: &serde_json::Value| m.as_str()) {
                                let manifest = std::path::Path::new(manifest_path);
                                if let Some(root) = manifest.parent() {
                                    let include_path = root.join("lib").join("include");
                                    if include_path.exists() {
                                        return Ok(include_path.to_string_lossy().to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Try to find in target directory
    if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = std::path::Path::new(&cargo_manifest_dir);
        // Look in target/build directory for tree-sitter-*/out/build
        let target_dir = manifest_path.join("../../target");
        if target_dir.exists() {
            // Use find command to locate parser.h
            if let Ok(output) = std::process::Command::new("find")
                .arg(&target_dir)
                .arg("-name")
                .arg("parser.h")
                .arg("-type")
                .arg("f")
                .output()
            {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        if line.contains("tree_sitter") {
                            let path: &std::path::Path = std::path::Path::new(line);
                            if let Some(parent) = path.parent() {
                                if let Some(grandparent) = parent.parent() {
                                    return Ok(grandparent.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Last resort: try to use pkg-config
    if let Ok(output) = std::process::Command::new("pkg-config")
        .args(["--cflags", "tree-sitter"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for part in stdout.split_whitespace() {
                if part.starts_with("-I") {
                    let path = &part[2..];
                    return Ok(path.to_string());
                }
            }
        }
    }
    
    Err("Could not find tree-sitter include path".to_string())
}

/// Build a Tree-sitter grammar and install it to the runtime directory
pub fn build_and_install_grammar(language_id: &str) -> Result<(), String> {
    let grammar_info = grammar_registry::for_language(language_id)
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
    
    // Special handling for markdown to ensure correct URL
    let repo_url = if language_id == "markdown" {
        println!("Using corrected markdown repository URL...");
        "https://github.com/tree-sitter-grammars/tree-sitter-markdown".to_string()
    } else {
        grammar_info.repo_url.clone()
    };
    
    println!("Cloning {}...", repo_url);
    
    // Clone repository with git
    println!("Cloning {}...", repo_url);
    
    let mut cmd = Command::new("git");
    cmd.args(["clone", "--depth", "1"]);
    cmd.env("GIT_TERMINAL_PROMPT", "0");
    cmd.args([&repo_url, repo_dir.to_str().unwrap()]);
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run git clone: {}", e))?;
    
    if !status.success() {
        return Err(format!("Failed to clone repository. Exit code: {:?}", status.code()));
    }
    
    println!("Successfully cloned repository");
    
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
    
    // For languages with subdirectories, we need to check if the source files exist
    // relative to the source_dir, not the repo root
    // The source_files in grammar_info are relative to the subdirectory
    // So we don't need to adjust them
    
    // Check if tree-sitter CLI is available and at a compatible version
    let has_tree_sitter_cli = Command::new("tree-sitter")
        .arg("--version")
        .output()
        .is_ok_and(|output| {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                // Check if version is at least 0.20.0
                version_str.contains("0.20") || version_str.contains("0.21") || 
                version_str.contains("0.22") || version_str.contains("0.23") ||
                version_str.contains("0.24") || version_str.contains("0.25") ||
                version_str.contains("0.26")
            } else {
                false
            }
        });
    
    let lib_path;
    
    if has_tree_sitter_cli {
        // Use tree-sitter CLI
        println!("Using tree-sitter CLI to build {}...", language_id);
        
        // Check if package.json exists and install dependencies if needed
        if source_dir.join("package.json").exists() {
            println!("Installing npm dependencies...");
            let install_output = Command::new("npm")
                .current_dir(&source_dir)
                .arg("install")
                .output()
                .map_err(|e| format!("Failed to run npm install: {}", e))?;
            
            if !install_output.status.success() {
                let stderr = String::from_utf8_lossy(&install_output.stderr);
                let stdout = String::from_utf8_lossy(&install_output.stdout);
                eprintln!("npm install output:\nstdout: {}\nstderr: {}", stdout, stderr);
                // Continue anyway, maybe dependencies are already installed
            } else {
                println!("npm dependencies installed successfully");
            }
        }
        
        // Check if we need to run tree-sitter generate
        // Look for grammar.js in source_dir or parent directory (for subdirectory cases)
        let grammar_js_exists = source_dir.join("grammar.js").exists() || 
                               repo_dir.join("grammar.js").exists();
        let parser_c_exists = source_dir.join("parser.c").exists() || 
                             source_dir.join("src/parser.c").exists() ||
                             repo_dir.join("parser.c").exists() ||
                             repo_dir.join("src/parser.c").exists();
        
        if grammar_js_exists && !parser_c_exists {
            // Run tree-sitter generate in the directory containing grammar.js
            let generate_dir = if source_dir.join("grammar.js").exists() {
                &source_dir
            } else {
                &repo_dir
            };
            
            println!("Running tree-sitter generate in {}...", generate_dir.display());
            let generate_output = Command::new("tree-sitter")
                .current_dir(generate_dir)
                .arg("generate")
                .output()
                .map_err(|e| format!("Failed to run tree-sitter generate: {}", e))?;
            
            if !generate_output.status.success() {
                let stderr = String::from_utf8_lossy(&generate_output.stderr);
                let stdout = String::from_utf8_lossy(&generate_output.stdout);
                return Err(format!("tree-sitter generate failed:\nstdout: {}\nstderr: {}", stdout, stderr));
            }
            println!("tree-sitter generate succeeded");
        } else if !parser_c_exists {
            // Check if source files exist in the grammar_info.source_files list
            let mut any_source_exists = false;
            for source_file in &grammar_info.source_files {
                if source_dir.join(source_file).exists() {
                    any_source_exists = true;
                    break;
                }
            }
            if !any_source_exists {
                return Err(format!("Cannot build {}: no source files found and grammar.js not found", language_id));
            }
            println!("Source files exist, skipping tree-sitter generate");
        } else {
            println!("parser.c already exists, skipping tree-sitter generate");
        }
        
        // Check if we should use tree-sitter build or manual compilation
        // If grammar.js exists in source_dir or repo_dir, use tree-sitter build
        let grammar_js_path = if source_dir.join("grammar.js").exists() {
            source_dir.join("grammar.js")
        } else if repo_dir.join("grammar.js").exists() {
            repo_dir.join("grammar.js")
        } else {
            PathBuf::new()
        };
        
        if grammar_js_path.exists() {
            // For markdown, always run tree-sitter build to ensure the library is built
            let should_run_build = if language_id == "markdown" {
                true
            } else {
                // For other languages, check if the library already exists
                let lib_name = get_library_name(language_id);
                let parser_lib_name = if cfg!(windows) {
                    "parser.dll"
                } else if cfg!(target_os = "macos") {
                    "parser.dylib"
                } else {
                    "parser.so"
                };
                // Check in source_dir first, then in the directory containing grammar.js
                let grammar_js_dir = grammar_js_path.parent().unwrap();
                !source_dir.join(parser_lib_name).exists() && 
                !source_dir.join(&lib_name).exists() &&
                !grammar_js_dir.join(parser_lib_name).exists() &&
                !grammar_js_dir.join(&lib_name).exists()
            };
            
            if should_run_build {
                println!("Running tree-sitter build for {}...", language_id);
                // Run tree-sitter build in the directory containing grammar.js
                let build_dir = grammar_js_path.parent().unwrap();
                let build_output = Command::new("tree-sitter")
                    .current_dir(build_dir)
                    .arg("build")
                    .output()
                    .map_err(|e| format!("Failed to run tree-sitter build: {}", e))?;
                
                if !build_output.status.success() {
                    let stderr = String::from_utf8_lossy(&build_output.stderr);
                    let stdout = String::from_utf8_lossy(&build_output.stdout);
                    return Err(format!("tree-sitter build failed:\nstdout: {}\nstderr: {}", stdout, stderr));
                }
                
                // Print build output for debugging
                let build_stdout = String::from_utf8_lossy(&build_output.stdout);
                let build_stderr = String::from_utf8_lossy(&build_output.stderr);
                if !build_stdout.trim().is_empty() {
                    println!("tree-sitter build stdout: {}", build_stdout);
                }
                if !build_stderr.trim().is_empty() {
                    println!("tree-sitter build stderr: {}", build_stderr);
                }
            } else {
                println!("Library already exists, skipping tree-sitter build");
            }
            
            // Find the built library
            let lib_name = get_library_name(language_id);
            let parser_lib_name = if cfg!(windows) {
                "parser.dll"
            } else if cfg!(target_os = "macos") {
                "parser.dylib"
            } else {
                "parser.so"
            };
            
            // Check common locations
            let grammar_js_dir = grammar_js_path.parent().unwrap();
            let possible_paths = vec![
                source_dir.join(parser_lib_name),
                source_dir.join(&lib_name),
                source_dir.join("target").join("release").join(parser_lib_name),
                source_dir.join("target").join("release").join(&lib_name),
                grammar_js_dir.join(parser_lib_name),
                grammar_js_dir.join(&lib_name),
                grammar_js_dir.join("target").join("release").join(parser_lib_name),
                grammar_js_dir.join("target").join("release").join(&lib_name),
            ];
            
            // For markdown, also check markdown-inline.so
            let markdown_paths = if language_id == "markdown" {
                let markdown_lib_name = if cfg!(windows) {
                    "markdown-inline.dll"
                } else if cfg!(target_os = "macos") {
                    "markdown-inline.dylib"
                } else {
                    "markdown-inline.so"
                };
                vec![
                    source_dir.join(markdown_lib_name),
                    source_dir.join("target").join("release").join(markdown_lib_name),
                ]
            } else {
                vec![]
            };
            
            let all_paths: Vec<_> = possible_paths.into_iter().chain(markdown_paths.into_iter()).collect();
            
            let mut found = None;
            for path in &all_paths {
                if path.exists() {
                    found = Some(path.clone());
                    break;
                }
            }
            
            lib_path = found.ok_or_else(|| {
                format!("Could not find built library after tree-sitter build")
            })?;
        } else {
            // No grammar.js, compile manually with cc
            println!("No grammar.js found, compiling manually with cc...");
            
            // Check if source files exist
            let mut source_files_exist = true;
            for source_file in &grammar_info.source_files {
                if !source_dir.join(source_file).exists() {
                    println!("Warning: Source file {} does not exist", source_file);
                    source_files_exist = false;
                }
            }
            
            if !source_files_exist {
                return Err(format!("Some source files are missing for {}", language_id));
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
                // Try to find tree-sitter include path
                let mut include_args = vec!["-c", "-fPIC", "-I./src"];
                
                // Add include path for tree-sitter headers if available
                if let Ok(tree_sitter_include) = find_tree_sitter_include_path() {
                    include_args.push("-I");
                    include_args.push(&tree_sitter_include);
                }
                
                // Add include path for the repo root (for common/ directory)
                if let Some(repo_root) = source_dir.parent() {
                    if repo_root.join("common").exists() {
                        include_args.push("-I");
                        include_args.push(repo_root.to_str().unwrap());
                    }
                }
                
                include_args.extend_from_slice(&["-o", object_file.to_str().unwrap()]);
                
                let output = Command::new("cc")
                    .args(&include_args)
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
            lib_path = temp_dir.path().join(&lib_name);
            
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
        lib_path = temp_dir.path().join(&lib_name);
        
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
    }
    
    // Install library to runtime directory
    let target_dir = runtime.grammar_dir();
    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create target directory: {}", e))?;
    
    let target_lib_path = target_dir.join(get_library_name(language_id));
    
    // For markdown, if the built library has a different name, rename it
    let source_lib_path = if language_id == "markdown" {
        let has_inline_name = lib_path.file_name()
            .and_then(|n: &std::ffi::OsStr| n.to_str())
            .map(|n: &str| n.contains("markdown-inline"))
            .unwrap_or(false);
        if has_inline_name {
        // Rename markdown-inline.so to libtree-sitter-markdown.so
        let renamed_path = lib_path.parent().unwrap().join(get_library_name(language_id));
        fs::copy(&lib_path, &renamed_path)
            .map_err(|e| format!("Failed to rename library: {}", e))?;
        renamed_path
    } else {
        lib_path
    };
    
    fs::copy(&source_lib_path, &target_lib_path)
        .map_err(|e| format!("Failed to copy library: {}", e))?;
    
    println!("Installed library to: {}", target_lib_path.display());
    
    // Install query files
    // For languages with subdirectories, queries might be in the parent directory
    let query_source_dir = if let Some(_subdir) = &grammar_info.subdirectory {
        // Try to find queries in the parent directory (repo root)
        let parent_dir = repo_dir.clone();
        let queries_in_parent = parent_dir.join("queries");
        if queries_in_parent.exists() {
            queries_in_parent
        } else {
            // Fall back to source_dir/queries
            source_dir.join("queries")
        }
    } else {
        source_dir.join("queries")
    };
    
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
            } else {
                // Try to find the query file in the repo root's queries directory
                let repo_queries_dir = repo_dir.join("queries");
                if repo_queries_dir.exists() {
                    let repo_source_path = repo_queries_dir.join(query_file);
                    if repo_source_path.exists() {
                        let target_path = query_target_dir.join(query_file);
                        fs::copy(&repo_source_path, &target_path)
                            .map_err(|e| format!("Failed to copy query file {}: {}", query_file, e))?;
                        println!("Installed query file from repo root: {}", query_file);
                    } else {
                        println!("Warning: Query file {} not found in {}", query_file, query_source_dir.display());
                    }
                } else {
                    println!("Warning: Query file {} not found in {}", query_file, query_source_dir.display());
                }
            }
        }
    } else {
        println!("Warning: No query directory found for {} at {}", language_id, query_source_dir.display());
        // Try to find queries in the repo root
        let repo_queries_dir = repo_dir.join("queries");
        if repo_queries_dir.exists() {
            let query_target_dir = runtime.language_dir(language_id).join("queries");
            fs::create_dir_all(&query_target_dir)
                .map_err(|e| format!("Failed to create query directory: {}", e))?;
            
            for query_file in &grammar_info.query_files {
                let source_path = repo_queries_dir.join(query_file);
                if source_path.exists() {
                    let target_path = query_target_dir.join(query_file);
                    fs::copy(&source_path, &target_path)
                        .map_err(|e| format!("Failed to copy query file {}: {}", query_file, e))?;
                    println!("Installed query file from repo root: {}", query_file);
                }
            }
        }
    }
    
    println!("Successfully installed {} grammar!", language_id);
    Ok(())
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
    
    // Some language IDs use underscores but the library uses hyphens
    // For example: "c_sharp" -> "c-sharp" in library name
    let lib_name = match language_id {
        "c_sharp" => "c-sharp",
        _ => language_id,
    };
    
    format!("{}tree-sitter-{}{}", prefix, lib_name, extension)
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
                    eprintln!("Will continue with other grammars...");
                }
            }
        } else {
            println!("Grammar for {} is already installed", language_id);
        }
    }
    
    installed
}
