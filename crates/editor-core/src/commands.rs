//! Editor commands for undo/redo and editing operations.

/// Editor commands that can be executed on the editor state.
#[derive(Debug, Clone)]
pub enum EditorCommand {
    InsertText {
        position: usize,
        text: String,
    },
    DeleteRange {
        start: usize,
        end: usize,
    },
    MoveCursor(crate::cursor::CursorMovement),
    SetSelection(Option<crate::selection::Selection>),
}
