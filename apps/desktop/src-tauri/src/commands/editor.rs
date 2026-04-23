use tauri::command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use zaroxi_domain_editor::Document;
use zaroxi_domain_editor::LargeFileMode;
use zaroxi_ops_file::FileLoader;

/// In-memory store for open documents.
static DOCUMENTS: once_cell::sync::Lazy<Mutex<HashMap<String, Document>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

/// Response for opening a document.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocumentResponse {
    pub document_id: String,
    pub path: String,
    pub line_count: usize,
    pub char_count: usize,
    pub large_file_mode: String,
    pub is_read_only: bool,
    pub content: String,
}

/// Request for visible lines.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleLinesRequest {
    pub document_id: String,
    pub start_line: usize,
    pub count: usize,
}

/// Response for visible lines.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleLinesResponse {
    pub lines: Vec<LineDto>,
    pub total_lines: usize,
}

/// A single line of text.
#[derive(Debug, Serialize)]
pub struct LineDto {
    pub index: usize,
    pub text: String,
}

/// Request for an edit operation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRequest {
    pub document_id: String,
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_text: String,
}

/// Open a document and return its metadata.
/// The frontend will request visible lines separately.
#[command]
pub async fn open_document(path: String) -> Result<OpenDocumentResponse, String> {
    // Load the file using the appropriate strategy
    let (file_source, _document) = FileLoader::load_file(&path)
        .map_err(|e| format!("Failed to load file: {}", e))?;

    let size = file_source.len() as u64;
    let large_file_mode = LargeFileMode::from_size(size);
    let is_read_only = large_file_mode.is_read_only();

    // Extract the full file content before consuming file_source
    let content = file_source.as_str().to_string();

    // Create the editor document
    let document = match file_source {
        zaroxi_ops_file::file_loader::FileSource::Memory(text) => {
            Document::from_text_with_path(&text, path.clone())
        }
        zaroxi_ops_file::file_loader::FileSource::Mmap(mmap) => {
            Document::from_mmap(mmap, path.clone(), size)
        }
    };

    let line_count = document.len_lines();
    let char_count = document.len_chars();
    let document_id = uuid::Uuid::new_v4().to_string();

    // Store the document
    let mut docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    docs.insert(document_id.clone(), document);

    Ok(OpenDocumentResponse {
        document_id,
        path,
        line_count,
        char_count,
        large_file_mode: format!("{:?}", large_file_mode),
        is_read_only,
        content,
    })
}

/// Get visible lines for a document.
#[command]
pub async fn get_visible_lines(request: VisibleLinesRequest) -> Result<VisibleLinesResponse, String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&request.document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    let total_lines = document.len_lines();
    let mut lines = Vec::new();

    // Clamp start_line to valid range
    let start_line = request.start_line.min(total_lines);
    let end_line = (start_line + request.count).min(total_lines);
    for line_idx in start_line..end_line {
        if let Some(text) = document.line(line_idx) {
            lines.push(LineDto {
                index: line_idx,
                text: text.to_string(),
            });
        }
    }

    Ok(VisibleLinesResponse {
        lines,
        total_lines,
    })
}

/// Apply an edit to a document.
#[command]
pub async fn apply_edit(request: EditRequest) -> Result<(), String> {
    let mut docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get_mut(&request.document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    // Reject edits for read‑only documents (very large files)
    if document.large_file_mode().is_read_only() {
        return Err("Document is read‑only (very large file)".to_string());
    }

    // Convert byte positions to character positions
    let start_char = document.byte_to_char(request.start_byte);
    let old_end_char = document.byte_to_char(request.old_end_byte);

    // Ensure start <= end
    let (delete_start, delete_end) = if start_char <= old_end_char {
        (start_char, old_end_char)
    } else {
        (old_end_char, start_char)
    };

    // Delete old range
    if delete_start < delete_end {
        document.delete(delete_start, delete_end)?;
    }

    // Insert new text at the start position (after deletion, the insertion point is delete_start)
    if !request.new_text.is_empty() {
        document.insert(delete_start, &request.new_text)?;
    }

    Ok(())
}

/// Save a document to disk.
#[command]
pub async fn save_document(document_id: String) -> Result<(), String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    let path = document.path()
        .ok_or_else(|| "Document has no path".to_string())?;

    let text = document.text();
    std::fs::write(path, &text)
        .map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(())
}

/// Get the total line count for a document.
#[command]
pub async fn get_line_count(document_id: String) -> Result<usize, String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&document_id)
        .ok_or_else(|| "Document not found".to_string())?;
    Ok(document.len_lines())
}
