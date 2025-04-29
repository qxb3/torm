use std::fs;
use std::path::PathBuf;

use crate::{TormResult, cursor::Cursor};

/// Represents a text buffer.
pub struct Buffer {
    /// Buffer cursor.
    cursor: Cursor,

    /// Buffer text content.
    content: String,

    /// The line count of the content.
    /// NOTE: starts at 0.
    line_count: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            cursor: Cursor::new(),
            content: String::new(),
            line_count: 0,
        }
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new Buffer from a file.
    pub fn from_file<P: Into<PathBuf>>(file_path: P) -> TormResult<Self> {
        let content = fs::read_to_string(file_path.into())?;
        let line_count = content.split('\n').count();

        Ok(Self {
            cursor: Cursor::new(),
            content,
            line_count: line_count - 1,
        })
    }

    /// Gets a referece of this buffer cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Gets a mutable referece of this buffer cursor.
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Gets a referece of this buffer content.
    pub fn content(&self) -> &String {
        &self.content
    }

    /// Gets a mutable referece of this buffer content.
    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }

    /// Gets the line count of the content of this buffer..
    pub fn line_count(&self) -> usize {
        self.line_count
    }

    /// Gets the mutable reference line count of the content of this buffer..
    pub fn line_count_mut(&mut self) -> &mut usize {
        &mut self.line_count
    }
}
