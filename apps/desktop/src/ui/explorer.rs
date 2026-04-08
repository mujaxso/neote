use std::collections::HashSet;
use core_types::workspace::DirectoryEntry;

/// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &str) -> String {
    use std::path::Path;
    let path = Path::new(path);
    let mut normalized = path.to_string_lossy().to_string();
    while normalized.ends_with(std::path::MAIN_SEPARATOR) {
        normalized.pop();
    }
    normalized
}

/// Represents a node in the file tree
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub entry: DirectoryEntry,
    pub children: Vec<TreeNode>,
}

/// Build a tree structure from flat directory entries
pub fn build_tree(
    entries: &[DirectoryEntry],
    workspace_path: &str,
) -> Vec<TreeNode> {
    // Sort entries by path for consistent ordering
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| a.path.cmp(&b.path));
    
    let workspace_root = if workspace_path.is_empty() {
        "".to_string()
    } else {
        normalize_path(workspace_path)
    };
    
    // Build a map from parent path to children
    let mut children_by_parent: std::collections::HashMap<String, Vec<DirectoryEntry>> = 
        std::collections::HashMap::new();
    
    for entry in &sorted_entries {
        let entry_path = std::path::Path::new(&entry.path);
        let parent = entry_path.parent();
        
        let parent_key = if let Some(parent) = parent {
            let parent_str = parent.to_string_lossy().to_string();
            normalize_path(&parent_str)
        } else {
            // Entry has no parent (root)
            // If it's directly in the workspace root, use workspace_root
            // Otherwise, use empty string
            "".to_string()
        };
        
        // Also check if the entry is directly in the workspace root
        // by comparing its parent with workspace_root
        let entry_parent_normalized = if let Some(parent) = parent {
            let parent_str = parent.to_string_lossy().to_string();
            normalize_path(&parent_str)
        } else {
            "".to_string()
        };
        
        let actual_key = if entry_parent_normalized == workspace_root {
            workspace_root.clone()
        } else {
            parent_key
        };
        
        children_by_parent.entry(actual_key).or_insert_with(Vec::new).push(entry.clone());
    }
    
    // Recursive function to build tree
    fn build_subtree(
        parent_path: &str,
        children_by_parent: &std::collections::HashMap<String, Vec<DirectoryEntry>>,
    ) -> Vec<TreeNode> {
        let mut nodes = Vec::new();
        
        if let Some(children) = children_by_parent.get(parent_path) {
            for child in children {
                let child_path = &child.path;
                let normalized_child_path = normalize_path(child_path);
                let child_nodes = build_subtree(&normalized_child_path, children_by_parent);
                nodes.push(TreeNode {
                    entry: child.clone(),
                    children: child_nodes,
                });
            }
        }
        
        // Sort nodes: directories first, then by name
        nodes.sort_by(|a, b| {
            if a.entry.is_dir != b.entry.is_dir {
                b.entry.is_dir.cmp(&a.entry.is_dir) // Directories first
            } else {
                a.entry.name.cmp(&b.entry.name)
            }
        });
        
        nodes
    }
    
    // Start building from workspace root
    build_subtree(&workspace_root, &children_by_parent)
}

/// Get visible tree nodes based on expanded directories
pub fn get_visible_nodes<'a>(
    tree: &'a [TreeNode],
    expanded_directories: &'a HashSet<String>,
    depth: usize,
) -> Vec<(usize, &'a TreeNode)> {
    let mut visible = Vec::new();
    
    for node in tree {
        visible.push((depth, node));
        
        // If this is a directory and it's expanded, add its children
        // Normalize the path for comparison (update.rs normalizes paths)
        if node.entry.is_dir {
            let normalized_path = normalize_path(&node.entry.path);
            if expanded_directories.contains(&normalized_path) {
                let child_visible = get_visible_nodes(&node.children, expanded_directories, depth + 1);
                visible.extend(child_visible);
            }
        }
    }
    
    visible
}
