mod error_reporter;
mod scanner;
mod token;

use std::{
    env, fs,
    io::{self, Write},
    process,
};

use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Error too many arguments");
            process::exit(64);
        }
    }
}

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
        let input = input.trim();
        if input.is_empty() {
            run(input.to_string());
        } else {
            println!("{:?}", input);
        }
    }
}

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

fn run(contents: String) {
    let scanner = Scanner::new(contents);
}
