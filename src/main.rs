use std::{env, fs, process::exit};

use ast_printer::print_ast;
use scanner::Scanner;
use token::Token;

static mut HAD_ERROR: bool = false;

mod ast_printer;
mod expr;
mod literal_object;
mod scanner;
mod token;
mod token_type;

fn main() {
    print_ast();
    // let args = env::args();
    // if args.len() > 2 {
    //     println!("Usage: rlox [script]");
    //     exit(64);
    // } else if args.len() == 2 {
    //     let file_name = args.last();
    //     match file_name {
    //         Some(file) => run_file(&file),
    //         None => {}
    //     }
    //     // run_file(args.)
    // } else {
    //     run_prompt();
    // }
}

fn run_file(path: &str) {
    let file = fs::read_to_string(path);

    match file {
        Ok(file) => run(&file),
        Err(_) => unsafe {
            HAD_ERROR = true;
            exit(65)
        },
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::default(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.to_string())
    }
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!("> ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.is_empty() {
            break;
        }
        run(&input);
        unsafe {
            HAD_ERROR = false;
        }
    }
}

pub fn error(line: u64, message: &str) {
    report(line, "", message)
}

fn report(line: u64, where_: &str, message: &str) {
    println!("[line   {}  ] Error  {}  : {}", line, where_, message);
}
