#[allow(dead_code)]
pub fn init() {
    // Initialize logging
    init_logging();
    
    // Initialize dynamic grammar system
    init_dynamic_grammars();
}

fn init_logging() {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    INIT.call_once(|| {
        // Simple logging to stderr for now
        // Check if we should enable debug logging
        let debug_env = std::env::var("ZAROXI_STUDIO_DEBUG").is_ok() || std::env::var("QYZER_STUDIO_DEBUG").is_ok();
        
        let mut builder = env_logger::Builder::from_default_env();
        
        if debug_env {
            builder.filter_level(log::LevelFilter::Debug);
        } else {
            // Default to warn level to avoid noise, but allow info for important messages
            builder.filter_level(log::LevelFilter::Info);
        }
        
        // Add some default formatting
        builder.format_timestamp(None);
        builder.format_module_path(false);
        builder.format_target(false);
        
        if let Err(e) = builder.try_init() {
            eprintln!("Failed to initialize logger: {}", e);
        }
        
        if debug_env {
            log::debug!("Debug logging enabled");
        }
        
        // Log environment info for debugging file picker issues
        log::info!("Initializing Zaroxi Studio");
        log::debug!("Environment: WAYLAND_DISPLAY={:?}, XDG_SESSION_TYPE={:?}, XDG_CURRENT_DESKTOP={:?}, HYPRLAND_INSTANCE_SIGNATURE={:?}",
            std::env::var("WAYLAND_DISPLAY"),
            std::env::var("XDG_SESSION_TYPE"),
            std::env::var("XDG_CURRENT_DESKTOP"),
            std::env::var("HYPRLAND_INSTANCE_SIGNATURE"));
    });
}

fn init_dynamic_grammars() {
    use syntax_core::dynamic_loader;
    use syntax_core::query_cache;
    use syntax_core::grammar_registry;
    use syntax_core::grammar_builder;
    use syntax_core::runtime::Runtime;
    
    // Initialize runtime
    let runtime = Runtime::new();
    
    // Create runtime directory if it doesn't exist
    if !runtime.exists() {
        let _ = std::fs::create_dir_all(runtime.root());
    }
    
    // Check which grammars are available
    let registry = grammar_registry::GrammarRegistry::global();
    let mut missing = Vec::new();
    
    for language_id in registry.language_ids() {
        if !dynamic_loader::is_grammar_available(language_id) {
            missing.push(language_id);
        }
    }
    
    // Auto-install missing grammars
    if !missing.is_empty() {
        for language_id in &missing {
            match grammar_builder::build_and_install_grammar(language_id) {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to install {} grammar: {}", language_id, e),
            }
        }
    }
    
    // Preload available grammars
    dynamic_loader::preload_available_grammars();
    
    // Preload queries
    query_cache::preload_queries();
}
