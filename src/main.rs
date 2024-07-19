//! Entry point for the Lox interpreter.
//!
//! This module ties together all components of the Lox interpreter and provides
//! the command-line interface for running Lox programs or starting an interactive REPL.

mod ast;
mod error_reporter;
//mod interpreter;
mod parser;
mod pretty_printer;
mod scanner;
mod token;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

use ast::Program;
use error_reporter::ErrorReporter;
//use interpreter::Interpreter;
use parser::Parser;
use pretty_printer::PrettyPrinter;
use scanner::Scanner;

/// The main entry point for the Lox interpreter.
///
/// Handles command-line arguments to either run a Lox file or start an interactive REPL.
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: lox [script]");
            process::exit(64);
        }
    }
}

/// Starts an interactive REPL (Read-Eval-Print Loop) for Lox.
///
/// This function repeatedly prompts the user for input, executes the input,
/// and displays the result until an empty line is entered.
fn run_prompt() {
    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout, Critical I/O error");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line: Critical I/O error");

        if input.trim().is_empty() {
            break;
        } else {
            run(input);
        }
    }
}

/// Runs a Lox program from a file.
///
/// # Arguments
///
/// * `filename` - The path to the Lox source file to execute.
///
/// # Exits
///
/// * Exit code 66: If the file is not found.
/// * Exit code 74: For any other file reading errors.
fn run_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => run(contents),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("Error: File '{}' not found", filename);
                process::exit(66);
            } else {
                eprintln!("Error reading file '{}' : {}", filename, e);
                process::exit(74);
            }
        }
    }
}

/// Executes a string of Lox source code.
///
/// This function orchestrates the entire interpretation process:
/// 1. Scanning (lexical analysis)
/// 2. Parsing (syntax analysis)
/// 3. Pretty printing (for debugging)
/// 4. Interpretation (execution)
///
/// # Arguments
///
/// * `contents` - A string slice containing Lox source code to execute.
fn run(contents: String) {
    // Scanning
    let mut scanner = Scanner::new(&contents);
    let tokens = scanner.scan_tokens();
    check(scanner.error_reporter);

    // Parsing
    let mut parser = Parser::new(&tokens);
    let program: Program = parser.parse_program();
    check(parser.error_reporter);

    // Pretty printing (for debugging)
    let pretty_printer = PrettyPrinter::new();
    println!("{}", pretty_printer.print_program(&program));

    // Interpretation
    //let mut interpreter = Interpreter::new();
    //interpreter.evaluate_program(&program);
    //check(interpreter.error_reporter);
}

/// Checks if any errors were reported during execution.
///
/// If errors were found, exits the program with code 65.
///
/// # Arguments
///
/// * `error_reporter` - The ErrorReporter instance to check for errors.
///
/// # Exits
///
/// * Exit code 65: If any errors were reported.
fn check(error_reporter: ErrorReporter) {
    if error_reporter.had_error() {
        process::exit(65);
    }
}
