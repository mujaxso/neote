//! Viewport state for visible editor area.

/// Viewport dimensions and state.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    width: f32,
    height: f32,
}

impl Viewport {
    /// Create a new viewport.
    pub fn new() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }

    /// Create a viewport with specific dimensions.
    pub fn with_dimensions(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Get the viewport width.
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Get the viewport height.
    pub fn height(&self) -> f32 {
        self.height
    }

    /// Set the viewport dimensions.
    pub fn set_dimensions(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
