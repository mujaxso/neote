#[allow(dead_code)]
pub fn init() {
    // Initialize logging
    // TODO: Set up proper logging
    
    // Initialize dynamic grammar system
    init_dynamic_grammars();
}

fn init_dynamic_grammars() {
    use syntax_core::dynamic_loader;
    use syntax_core::query_cache;
    use syntax_core::grammar_registry;
    use syntax_core::grammar_builder;
    use syntax_core::runtime::Runtime;
    
    println!("Initializing dynamic grammar system...");
    
    // Initialize runtime
    let runtime = Runtime::new();
    println!("Runtime directory: {:?}", runtime.root());
    
    // Create runtime directory if it doesn't exist
    if !runtime.exists() {
        println!("Creating runtime directory...");
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
        println!("Auto-installing missing grammars: {:?}", missing);
        for language_id in &missing {
            println!("Installing {} grammar...", language_id);
            match grammar_builder::build_and_install_grammar(language_id) {
                Ok(_) => println!("Successfully installed {} grammar", language_id),
                Err(e) => eprintln!("Failed to install {} grammar: {}", language_id, e),
            }
        }
    }
    
    // Preload available grammars
    dynamic_loader::preload_available_grammars();
    
    // Preload queries
    query_cache::preload_queries();
    
    println!("Dynamic grammar system initialized");
}
