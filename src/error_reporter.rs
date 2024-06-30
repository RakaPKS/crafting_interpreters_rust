pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[Line {}] Error{}: {}", line, loc, message);
        self.had_error = true;
    }
}
