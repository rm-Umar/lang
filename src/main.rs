mod scanner;
use crate::scanner::*;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: cringe_lang [file]");
        exit(69);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens().expect("Failed to scan tokens");
    for token in tokens {
        println!("{:?}", token);
    }
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        let mut buffer = String::new();
        let input = io::stdin();
        io::stdout().flush().expect("failed to flush");

        let mut handle = input.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Could not read line".to_string()),
        }
        run(&buffer)?;
    }
}
