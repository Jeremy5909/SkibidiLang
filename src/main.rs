use std::{env, fs, io::{stdin, stdout, Write}};

use expr::{BinaryOperator, Expr, Literal, UnaryOperator};
use scanner::Scanner;

mod token;
mod scanner;
mod expr;
#[macro_use]
mod macros;

fn main() {
    let args: Vec<_> = env::args().collect();

    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: UnaryOperator::Minus,
            right: Box::new(Expr::Literal {
                value: Literal::Number(123.0),
            }),
        }),
        operator: BinaryOperator::Star,
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Literal::Number(45.67),
            }),
        }),
    };
    println!("{expr}");
    
    match args.len() {
        1 => run_prompt(), 
        2 => run_file(&args[1]),
        _ => panic!("Usage: skib [script]")
    }
}

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
