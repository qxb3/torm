use ratatui::layout::Position;

use crate::TormResult;

/// Represents a cursor in a buffer.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    /// The current column of the cursor.
    /// NOTE: This represents a coordinate in screen
    /// not the visual representation of it.
    col: u16,

    /// The current line of the cursor.
    /// NOTE: This represents a coordinate in screen
    /// not the visual representation of it.
    line: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the column of the cursor.
    pub fn set_col(&mut self, col: u16) {
        self.col = col;
    }

    /// Gets the current column of the cursor.
    pub fn col(&self) -> u16 {
        self.col
    }

    /// Sets the line of the cursor.
    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    /// Gets the current line of the cursor.
    pub fn line(&self) -> u16 {
        self.line
    }

    /// Gets the current column and line of the cursor in ratatui::layout::Position.
    pub fn position(&self) -> Position {
        let col = self.col();
        let line = self.line();

        Position::new(col, line)
    }

    /// Moves the line up with n count.
    /// This function does nothing if the current line is already at 0.
    pub fn move_up(&mut self, count: u16) {
        if self.line != 0 {
            self.line -= count;
        }
    }

    /// Moves the line down with n count.
    pub fn move_down(&mut self, count: u16) {
        self.line += count;
    }

    /// Moves the column to the left with n count.
    /// This function does nothing if the current column is already at 0.
    pub fn move_left(&mut self, count: u16) {
        if self.col != 0 {
            self.col -= count;
        }
    }

    /// Moves the column to the right with n count.
    pub fn move_right(&mut self, count: u16) {
        self.col += count;
    }
}
