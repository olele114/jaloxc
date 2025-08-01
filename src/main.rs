/// Main module for Lox interpreter implementation in Rust.
/// 
/// Handles command-line interface, file execution, and REPL functionality.
/// Coordinates scanning and token generation from source input.
mod expr;
mod token;
mod scanner;

use std::{
    env, io,
    path::Path,
    io::Write
};
use crate::scanner::Scanner;

/// Entry point for the Lox interpreter.
/// 
/// Parses command line arguments and dispatches to appropriate execution modes.
fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}

/// Executes Lox source code from a file.
///
/// # Arguments
/// * `path` - Path to the Lox script file
fn run_file(path: impl AsRef<Path>) {
    let source = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(74);
    });

    run(&source);
}

/// Starts the interactive Read-Eval-Print Loop (REPL).
///
/// Continuously reads user input, executes it, and prints results.
/// Exits on Ctrl+D or when an error occurs.
fn run_prompt() {
    println!("jaloxc interpreter (exit with Ctrl+D)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => run(&line),
        }
    }
}

/// Executes Lox source code.
///
/// Coordinates the scanning process and outputs tokens.
///
/// # Arguments
/// * `source` - Lox source code to Execute
fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().clone();

    for token in tokens {
        println!("{}", token);
    }
}
