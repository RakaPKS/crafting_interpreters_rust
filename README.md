# Lox Interpreter in Rust

This project is an implementation of a Lox programming language Tree-Walk interpreter in Rust, inspired by the book "Crafting Interpreters" by Robert Nystrom. It provides a fully functional interpreter for the Lox language with some Rust-specific adaptations. The goal of this project was to have a better understanding of Rust, so certain implementation have been significantly modified to be more rust-like and less Java-like. 

## Table of Contents

- [Features](#features)
- [Documentation](#documentation)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [License](#license)

## Features

- Lexical analysis (scanning)
- Syntax analysis (parsing)
- Runtime interpretation
- Support for Lox language constructs including:
  - Arithmetic and logical operations
  - Variable declarations and assignments
  - Control flow statements (if, while, for)
  - Function declarations and calls
  - Object-oriented programming with classes

## Documentation

The documentation for this project is available at [https://RakaPKS.github.io/crafting_interpreters_rust/](https://RakaPKS.github.io/crafting_interpreters_rust/)

## Getting Started

To get started with this Lox interpreter, you'll need Rust installed on your system. If you don't have Rust installed, you can get it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Clone the repository:

```
git clone https://github.com/RakaPKS/crafting_interpreters_rust.git
cd crafting-interpreter-rust
```

Build the project:

```
cargo build --release
```

## Usage

You can use the Lox interpreter in two modes:

1. REPL (Read-Eval-Print Loop) mode:
   ```
   cargo run
   ```
   This will start an interactive session where you can type Lox expressions and statements.

2. File execution mode:
   ```
   cargo run -- path/to/your/lox/script.lox
   ```
   This will execute the Lox script file specified.

## Project Structure

The project, as of now, is organized into several modules:

- `main.rs`: Entry point of the interpreter
- `scanner.rs`: Lexical analyzer
- `parser.rs`: Syntax analyzer
- `interpreter.rs`: Runtime interpreter
- `expression.rs`: Expression data structures
- `token.rs`: Token definitions
- `error_reporter.rs`: Error handling utilities
- `pretty_printer.rs`: AST visualization tool


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

