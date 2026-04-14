#!/usr/bin/env bash
set -e

echo "Installing Tree-sitter Markdown grammar..."

# Create runtime directory
RUNTIME_DIR="$HOME/Work/qyzer/runtime/treesitter"
mkdir -p "$RUNTIME_DIR/grammars/linux-x86_64"
mkdir -p "$RUNTIME_DIR/languages/markdown/queries"

# Download and compile
cd /tmp
rm -rf tree-sitter-markdown
git clone --depth 1 https://github.com/tree-sitter/tree-sitter-markdown
cd tree-sitter-markdown

# Build with tree-sitter CLI if available
if command -v tree-sitter &> /dev/null; then
    tree-sitter generate
    tree-sitter build --release
    cp target/release/libtree-sitter-markdown.so "$RUNTIME_DIR/grammars/linux-x86_64/"
else
    # Manual build
    cc -shared -fPIC -o libtree-sitter-markdown.so src/parser.c src/scanner.c
    cp libtree-sitter-markdown.so "$RUNTIME_DIR/grammars/linux-x86_64/"
fi

# Copy query files
if [ -d queries ]; then
    cp queries/* "$RUNTIME_DIR/languages/markdown/queries/"
fi

echo "Markdown grammar installed successfully!"
echo "Library: $RUNTIME_DIR/grammars/linux-x86_64/libtree-sitter-markdown.so"
