use std::env::args;
use lox::{run_file, run_prompt};

mod lox;
mod token;
mod scanner;

fn main() {
    match args().len() {
        1 => run_prompt(), 
        2 => run_file(&args().nth(1).unwrap()),
        _ => panic!("Usage: skib [script]")
    }
}

