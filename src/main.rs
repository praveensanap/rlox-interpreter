mod scanner;
mod token;

use scanner::*;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> std::io::Result<()> {
    let buf = fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    print!("> ");
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(m) => {
                    m.report("can't read file".to_string());
                }
            }
        } else {
            break;
        }
    }
}

#[derive(Debug)]
struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(&self, line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}

fn run(code: String) -> Result<(), LoxError> {
    println!("{}", code);
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
