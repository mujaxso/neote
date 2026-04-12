fn main() {
    // Check if font files exist from various possible locations
    let possible_paths = [
        // Relative to build.rs location
        "assets/fonts/JetBrainsMono-Regular.ttf",
        "assets/fonts/JetBrainsMonoNerdFont-Regular.ttf",
        "assets/fonts/FiraCode-Regular.ttf",
        "assets/fonts/FiraCodeNerdFont-Regular.ttf",
        "assets/fonts/NotoColorEmoji.ttf",
        "assets/fonts/SymbolsNerdFont-Regular.ttf",
        "assets/fonts/CascadiaCode-Regular.ttf",
        "assets/fonts/CascadiaCodeNerdFont-Regular.ttf",
        "assets/fonts/Iosevka-Regular.ttf",
        "assets/fonts/IosevkaNerdFont-Regular.ttf",
        // Relative to project root
        "apps/desktop/assets/fonts/JetBrainsMono-Regular.ttf",
        "apps/desktop/assets/fonts/JetBrainsMonoNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/FiraCode-Regular.ttf",
        "apps/desktop/assets/fonts/FiraCodeNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/NotoColorEmoji.ttf",
        "apps/desktop/assets/fonts/SymbolsNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/CascadiaCode-Regular.ttf",
        "apps/desktop/assets/fonts/CascadiaCodeNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/Iosevka-Regular.ttf",
        "apps/desktop/assets/fonts/IosevkaNerdFont-Regular.ttf",
    ];
    
    let mut jetbrains_found = false;
    let mut firacode_found = false;
    let mut _emoji_found = false;
    let mut symbols_found = false;
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            if path.contains("JetBrainsMono") {
                jetbrains_found = true;
            }
            if path.contains("FiraCode") {
                firacode_found = true;
            }
            if path.contains("NotoColorEmoji") {
                _emoji_found = true;
            }
            if path.contains("SymbolsNerdFont") {
                symbols_found = true;
            }
        }
    }
    
    // Only print warnings if essential fonts are missing
    // For icons, we need Symbols Nerd Font and at least one coding font
    if !symbols_found || (!jetbrains_found && !firacode_found) {
        println!("cargo:warning=Some font files not found. Icons may not display correctly.");
        println!("cargo:warning=To download fonts, run from apps/desktop directory:");
        println!("cargo:warning=  chmod +x scripts/download-fonts.sh");
        println!("cargo:warning=  ./scripts/download-fonts.sh");
    }

    // Determine if dynamic grammars feature is enabled
    let dynamic_grammars = std::env::var("CARGO_FEATURE_DYNAMIC_GRAMMARS").is_ok();

    if dynamic_grammars {
        // We'll attempt to download grammars for dynamic linking.
        // The script will place libraries in the appropriate target-specific directory.
        let runtime_root = std::path::Path::new("runtime/treesitter");
        let target = std::env::var("TARGET").unwrap_or_default();
        let os = if target.contains("linux") {
            "linux"
        } else if target.contains("darwin") {
            "macos"
        } else if target.contains("windows") {
            "windows"
        } else {
            // unknown, default to linux
            "linux"
        };
        let arch = if target.contains("x86_64") {
            "x86_64"
        } else if target.contains("aarch64") || target.contains("arm64") {
            "aarch64"
        } else {
            "x86_64"
        };
        let grammar_dir = runtime_root.join("grammars").join(format!("{}-{}", os, arch));
        // Check for at least Rust and TOML grammars (most essential)
        let rust_grammar = grammar_dir.join("libtree-sitter-rust.so");
        let toml_grammar = grammar_dir.join("libtree-sitter-toml.so");

        // Determine library extension based on OS
        let lib_ext = match os {
            "macos" => "dylib",
            "windows" => "dll",
            _ => "so",
        };
        let rust_grammar_with_ext = grammar_dir.join(format!("libtree-sitter-rust.{}", lib_ext));
        let toml_grammar_with_ext = grammar_dir.join(format!("libtree-sitter-toml.{}", lib_ext));

        let grammars_exist = (rust_grammar.exists() || rust_grammar_with_ext.exists())
            && (toml_grammar.exists() || toml_grammar_with_ext.exists());

        if !grammars_exist {
            println!("cargo:warning=Tree‑sitter grammar libraries missing for rust, toml. Attempting to download…");
            // Try to locate the fetch script in multiple locations
            let possible_script_paths = [
                runtime_root.join("fetch-grammars.sh"),
                std::path::Path::new("runtime").join("fetch-grammars.sh"),
                std::path::Path::new("runtime/treesitter").join("fetch-grammars.sh"),
            ];
            let mut script_found = None;
            for path in &possible_script_paths {
                if path.exists() {
                    script_found = Some(path);
                    break;
                }
            }
            if let Some(script_path) = script_found {
                use std::process::Command;
                // Pass TARGET environment variable to the script
                let status = Command::new("bash")
                    .arg(script_path)
                    .env("TARGET", &target)
                    .status();
                match status {
                    Ok(exit_status) if exit_status.success() => {
                        println!("cargo:warning=Successfully downloaded grammar libraries.");
                    }
                    Ok(exit_status) => {
                        println!("cargo:warning=fetch-grammars.sh exited with status {}", exit_status);
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to run fetch-grammars.sh: {}", e);
                    }
                }
            } else {
                println!("cargo:warning=fetch-grammars.sh not found. Grammars will be missing.");
                println!("cargo:warning=Please run the script manually from runtime/treesitter/");
            }
        } else {
            println!("cargo:warning=Tree‑sitter grammars are present.");
        }
    } else {
        println!("cargo:warning=Dynamic grammars disabled, using static linking.");
    }
}
