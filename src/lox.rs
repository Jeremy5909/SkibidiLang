use std::{fs, io::{stdin, stdout, Write}};

use crate::scanner::Scanner;

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<_> = scanner.scan_tokens().to_vec();

    for token in tokens {
        println!("{:?}", token);
    }
}

pub fn run_file(file_path: &str) {
    run(&fs::read_to_string(file_path).unwrap());   
}

pub fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        run(input.trim());
    }
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, pos: &str, message: &str) {
    eprintln!("[line {}] Error{}: {} ", line, pos, message);
}
