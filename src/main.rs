mod scanner;
mod token;

use scanner::*;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{self, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    for a in args {
        println!("called with args {}", a);
    }

    let res = run_file(&"test.lox".to_string()).is_err();
    println!("{}", res)
    // if args.len() > 2 {
    //     println!("Usage: rlox [script]");
    //     for a in args {
    //         println!(" got {}", a);
    //     }
    //     std::process::exit(64);
    // } else if args.len() == 2 {
    //     run_file(&args[1]).expect("Could not run file");
    // } else {
    // }
}

fn run_file(path: &String) -> std::io::Result<()> {
    let buf = fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    print!("> ");
    _ = stdout().flush();
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
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
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

    match tokens {
        Ok(t) => {
            for i in t {
                println!("{:?}", i);
            }
        }
        Err(_) => {}
    }

    Ok(())
}
