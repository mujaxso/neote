fn main() {
    // Check if font files exist from various possible locations
    let possible_paths = [
        // Relative to build.rs location
        "assets/fonts/JetBrainsMono-Regular.ttf",
        "assets/fonts/FiraCode-Regular.ttf",
        "assets/fonts/NotoColorEmoji.ttf",
        // Relative to project root
        "apps/desktop/assets/fonts/JetBrainsMono-Regular.ttf",
        "apps/desktop/assets/fonts/FiraCode-Regular.ttf",
        "apps/desktop/assets/fonts/NotoColorEmoji.ttf",
    ];
    
    let mut jetbrains_found = false;
    let mut firacode_found = false;
    let mut emoji_found = false;
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            if path.contains("JetBrainsMono") {
                jetbrains_found = true;
            }
            if path.contains("FiraCode") {
                firacode_found = true;
            }
            if path.contains("NotoColorEmoji") {
                emoji_found = true;
            }
        }
    }
    
    // Only print warnings if fonts are missing
    if !jetbrains_found || !firacode_found || !emoji_found {
        println!("cargo:warning=Some font files not found. Icons may not display correctly.");
        println!("cargo:warning=To download fonts, run from apps/desktop directory:");
        println!("cargo:warning=  chmod +x scripts/download-fonts.sh");
        println!("cargo:warning=  ./scripts/download-fonts.sh");
    }
}
