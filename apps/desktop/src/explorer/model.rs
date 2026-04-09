use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    pub fn new(entry: &DirectoryEntry) -> Self {
        Self {
            path: PathBuf::from(&entry.path),
            name: entry.name.clone(),
            is_dir: entry.is_dir,
            children: Vec::new(),
        }
    }
}

pub fn build_explorer_tree(entries: &[DirectoryEntry]) -> Vec<ExplorerNode> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // Create nodes from all entries
    let mut nodes: Vec<ExplorerNode> = entries.iter()
        .map(|entry| ExplorerNode::new(entry))
        .collect();
    
    // Build a map from path string to index for quick lookup
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        path_to_index.insert(node.path.to_string_lossy().to_string(), i);
    }
    
    // Group children by parent path
    let mut children_by_parent: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (i, node) in nodes.iter().enumerate() {
        if let Some(parent_path) = node.path.parent() {
            // Normalize the parent path
            let mut parent_str = parent_path.to_string_lossy().to_string();
            // Remove trailing separator if present
            while parent_str.ends_with('/') || parent_str.ends_with('\\') {
                parent_str.pop();
            }
            
            // Check if parent exists in our entries
            if path_to_index.contains_key(&parent_str) {
                children_by_parent.entry(parent_str)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    
    // Build the tree structure
    let mut root_indices = Vec::new();
    
    for (i, node) in nodes.iter().enumerate() {
        // Check if this node has a parent in the tree
        let has_parent = if let Some(parent_path) = node.path.parent() {
            // Normalize the parent path
            let mut parent_str = parent_path.to_string_lossy().to_string();
            while parent_str.ends_with('/') || parent_str.ends_with('\\') {
                parent_str.pop();
            }
            path_to_index.contains_key(&parent_str)
        } else {
            false
        };
        
        if !has_parent {
            root_indices.push(i);
        }
    }
    
    // Build subtrees for each root
    for &root_idx in &root_indices {
        build_subtree(root_idx, &mut nodes, &children_by_parent);
    }
    
    // Collect root nodes
    let mut root_nodes: Vec<ExplorerNode> = root_indices
        .into_iter()
        .map(|idx| nodes[idx].clone())
        .collect();
    
    // Sort root nodes: directories first, then files, alphabetically
    root_nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    root_nodes
}

fn build_subtree(
    index: usize,
    nodes: &mut [ExplorerNode],
    children_by_parent: &HashMap<String, Vec<usize>>,
) {
    // Normalize the path string
    let mut path_str = nodes[index].path.to_string_lossy().to_string();
    while path_str.ends_with('/') || path_str.ends_with('\\') {
        path_str.pop();
    }
    
    if let Some(child_indices) = children_by_parent.get(&path_str) {
        let mut children = Vec::new();
        
        // First, collect all children
        for &child_idx in child_indices {
            // Recursively build the child's subtree
            build_subtree(child_idx, nodes, children_by_parent);
            children.push(nodes[child_idx].clone());
        }
        
        // Sort children: directories first, then files, alphabetically
        children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        nodes[index].children = children;
    }
}
