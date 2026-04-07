//! Text buffer implementation.
//!
//! Manages the underlying text storage, line tracking, and basic text
//! manipulation operations.

use ropey::Rope;

#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    version: i32,
    dirty: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = TextBuffer::new("Hello, world!");
        assert_eq!(buffer.len_chars(), 13);
        assert_eq!(buffer.len_lines(), 1);
        assert_eq!(buffer.text(), "Hello, world!");
        assert_eq!(buffer.to_string(), "Hello, world!");
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.version(), 0);
    }

    #[test]
    fn test_from_str() {
        let buffer = TextBuffer::from_str("Test");
        assert_eq!(buffer.text(), "Test");
    }

    #[test]
    fn test_replace_all() {
        let mut buffer = TextBuffer::new("Initial");
        buffer.replace_all("Replaced");
        assert_eq!(buffer.text(), "Replaced");
        assert!(buffer.is_dirty());
        assert_eq!(buffer.version(), 1);
    }

    #[test]
    fn test_insert() {
        let mut buffer = TextBuffer::new("Hello world!");
        buffer.insert_char_idx(6, "beautiful ").unwrap();
        assert_eq!(buffer.text(), "Hello beautiful world!");
        assert!(buffer.is_dirty());
        assert_eq!(buffer.version(), 1);
    }

    #[test]
    fn test_insert_out_of_bounds() {
        let mut buffer = TextBuffer::new("Hello");
        let result = buffer.insert_char_idx(10, " world");
        assert!(result.is_err());
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.version(), 0);
    }

    #[test]
    fn test_delete() {
        let mut buffer = TextBuffer::new("Hello beautiful world!");
        buffer.delete_char_range(6, 16).unwrap(); // Delete "beautiful "
        assert_eq!(buffer.text(), "Hello world!");
        assert!(buffer.is_dirty());
        assert_eq!(buffer.version(), 1);
    }

    #[test]
    fn test_delete_invalid_range() {
        let mut buffer = TextBuffer::new("Hello");
        let result = buffer.delete_char_range(3, 10);
        assert!(result.is_err());
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.version(), 0);
    }

    #[test]
    fn test_mark_saved() {
        let mut buffer = TextBuffer::new("Text");
        buffer.replace_all("New text");
        assert!(buffer.is_dirty());
        buffer.mark_saved();
        assert!(!buffer.is_dirty());
    }

    #[test]
    fn test_line_counting() {
        let buffer = TextBuffer::new("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.len_lines(), 3);
    }

    #[test]
    fn test_char_to_line() {
        let buffer = TextBuffer::new("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.char_to_line(0), 0); // 'L' in Line 1
        assert_eq!(buffer.char_to_line(7), 1); // 'L' in Line 2
        assert_eq!(buffer.char_to_line(14), 2); // 'L' in Line 3
    }

    #[test]
    fn test_line_col_to_char() {
        let buffer = TextBuffer::new("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line_col_to_char(0, 2).unwrap(), 2); // 'n' in Line 1
        assert_eq!(buffer.line_col_to_char(1, 3).unwrap(), 10); // 'e' in Line 2
    }

    #[test]
    fn test_unicode_safety() {
        let mut buffer = TextBuffer::new("Hello ");
        buffer.insert_char_idx(6, "🌍").unwrap();
        assert_eq!(buffer.text(), "Hello 🌍");
        assert_eq!(buffer.len_chars(), 8); // Note: emoji is 1 char, not 2
        assert_eq!(buffer.len_lines(), 1);
    }

    #[test]
    fn test_slice_char_range() {
        let buffer = TextBuffer::new("Hello, world!");
        let slice = buffer.slice_char_range(7, 12).unwrap();
        assert_eq!(slice, "world");
    }

    #[test]
    fn test_is_large() {
        let small_buffer = TextBuffer::new("Hello");
        assert!(!small_buffer.is_large());
        assert!(!small_buffer.is_very_large());
        
        // Note: We can't easily test large buffers without allocating a lot of memory
        // But we can test the thresholds are correct
        assert!(TextBuffer::new(&"a".repeat(1_000_001)).is_large());
        assert!(TextBuffer::new(&"a".repeat(10_000_001)).is_very_large());
    }

    #[test]
    fn test_apply_iced_action() {
        use iced::widget::text_editor::{Action, EditAction};
        
        let mut buffer = TextBuffer::new("Hello world!");
        
        // Test insert action
        let insert_action = Action::Edit(EditAction::InsertText {
            char_idx: 6,
            text: "beautiful ".to_string(),
        });
        
        buffer.apply_iced_action(&insert_action).unwrap();
        assert_eq!(buffer.text(), "Hello beautiful world!");
        
        // Test delete action
        let delete_action = Action::Edit(EditAction::DeleteRange {
            char_idx: 6,
            len: 10,
        });
        
        buffer.apply_iced_action(&delete_action).unwrap();
        assert_eq!(buffer.text(), "Hello world!");
    }
}

impl TextBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            rope: Rope::from_str(&text.into()),
            version: 0,
            dirty: false,
        }
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            version: 0,
            dirty: false,
        }
    }

    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn replace_all(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
        self.version += 1;
        self.dirty = true;
    }

    /// Apply an iced text editor action to the buffer
    /// This is a simplified implementation that handles common actions
    pub fn apply_iced_action(&mut self, action: &iced::widget::text_editor::Action) -> Result<(), String> {
        use iced::widget::text_editor::Action;
        
        match action {
            Action::Edit(edit_action) => {
                match edit_action {
                    iced::widget::text_editor::EditAction::InsertText { char_idx, text } => {
                        self.insert_char_idx(*char_idx, text)?;
                    }
                    iced::widget::text_editor::EditAction::DeleteRange { char_idx, len } => {
                        let start = *char_idx;
                        let end = start + *len;
                        self.delete_char_range(start, end)?;
                    }
                    // For other actions, we can implement them as needed
                    _ => {
                        // Fall back to full replacement for unsupported actions
                        // This is not ideal but works for now
                        return Err("Unsupported edit action".to_string());
                    }
                }
            }
            // For other actions, we can handle them as needed
            _ => {
                return Err("Unsupported action type".to_string());
            }
        }
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Check if the buffer is considered large
    pub fn is_large(&self) -> bool {
        // Threshold: 1MB in characters (approximate)
        const LARGE_THRESHOLD: usize = 1_000_000;
        self.len_chars() > LARGE_THRESHOLD
    }

    /// Check if the buffer is considered very large (read-only recommended)
    pub fn is_very_large(&self) -> bool {
        // Threshold: 10MB in characters
        const VERY_LARGE_THRESHOLD: usize = 10_000_000;
        self.len_chars() > VERY_LARGE_THRESHOLD
    }

    /// Get a snapshot of the buffer as a string
    /// Use sparingly for large buffers
    pub fn snapshot_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn insert_char_idx(&mut self, char_idx: usize, text: &str) -> Result<(), String> {
        if char_idx > self.rope.len_chars() {
            return Err(format!("Char index {} out of bounds (length {})", 
                char_idx, self.rope.len_chars()));
        }
        self.rope.insert(char_idx, text);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn delete_char_range(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.rope.len_chars() {
            return Err(format!("End {} out of bounds (length {})", 
                end, self.rope.len_chars()));
        }
        self.rope.remove(start..end);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    // Helper methods for position conversion
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    pub fn line_col_to_char(&self, line: usize, col: usize) -> Result<usize, String> {
        if line >= self.rope.len_lines() {
            return Err(format!("Line {} out of bounds (total lines {})", 
                line, self.rope.len_lines()));
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        // Column is in characters, not bytes
        if col > line_len {
            return Err(format!("Column {} out of bounds for line {} (line length {})", 
                col, line, line_len));
        }
        Ok(line_start + col)
    }

    pub fn slice_char_range(&self, start: usize, end: usize) -> Result<String, String> {
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.rope.len_chars() {
            return Err(format!("End {} out of bounds (length {})", 
                end, self.rope.len_chars()));
        }
        Ok(self.rope.slice(start..end).to_string())
    }
}
