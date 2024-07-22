//! Provides error reporting functionality for the Lox interpreter.
//!
//! This module contains the `ErrorReporter` struct which is responsible for
//! tracking and reporting errors during the interpretation process
//! without crashing or panicing.

/// Represents errors that can occur during parsing.
pub enum ParseError {
    UnexpectedToken(),
    MissingToken(),
    UnexpectedEOF(),
}

pub enum RuntimeError {
    UndefinedVariable(),
    UnInitializedVariable(),
}

/// A struct for reporting and tracking errors in the Lox interpreter.
pub struct ErrorReporter {
    /// Indicates whether an error has been encountered.
    had_error: bool,
}

impl ErrorReporter {
    /// Creates a new `ErrorReporter` instance.
    ///
    /// Initializes with no errors reported.
    pub fn new() -> Self {
        ErrorReporter { had_error: false }
    }

    /// Reports an error at a specific line and column.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number where the error occurred.
    /// * `column` - The column number where the error occurred.
    /// * `message` - The error message to report.
    pub fn error(&mut self, line: usize, column: usize, message: &str) {
        self.report(line, column, "", message);
    }

    /// Internal method to format and print the error message.
    /// Also sets the `had_error` flag to true.
    fn report(&mut self, line: usize, column: usize, loc: &str, message: &str) {
        eprintln!(
            "[Line {}, Column {}] Error{}: {}",
            line, column, loc, message
        );
        self.had_error = true;
    }

    /// Returns whether an error has been reported.
    pub fn had_error(&self) -> bool {
        self.had_error
    }
}
