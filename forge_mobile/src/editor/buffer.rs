// buffer.rs

use ropey::Rope;

/// Represents a mobile-optimized text buffer.
pub struct Buffer {
    content: Rope,
}

impl Buffer {
    /// Creates a new empty Buffer.
    pub fn new() -> Self {
        Buffer { content: Rope::new() }
    }

    /// Inserts text at a specified position.
    pub fn insert(&mut self, position: usize, text: &str) {
        self.content.insert(position, text);
    }

    /// Removes text from a specified position.
    pub fn remove(&mut self, position: usize, length: usize) {
        self.content.remove(position..position + length);
    }

    /// Returns the current content as a string.
    pub fn content(&self) -> String {
        self.content.to_string()
    }
}