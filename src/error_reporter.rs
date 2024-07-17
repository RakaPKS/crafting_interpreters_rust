pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { had_error: false }
    }

    pub fn interpreter_error(&mut self, message: &str) {
        eprintln!("Error: {}", message);
        self.had_error = true;
    }

    pub fn error(&mut self, line: usize, column: usize, message: &str) {
        self.report(line, column, "", message);
    }

    fn report(&mut self, line: usize, column: usize, loc: &str, message: &str) {
        eprintln!(
            "[Line {}, Column {}] Error{}: {}",
            line, column, loc, message
        );
        self.had_error = true;
    }
    pub fn had_error(&self) -> bool {
        self.had_error
    }
}
