fn main() {
    // Tell Cargo to link the tree-sitter-rust C library
    // This is needed for the rust feature
    #[cfg(feature = "rust")]
    {
        // The tree-sitter-rust crate should have already built the library
        // We just need to ensure it's linked
        println!("cargo:rustc-link-lib=static=tree-sitter-rust");
    }
}
