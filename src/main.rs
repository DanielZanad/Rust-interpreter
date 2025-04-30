use std::{env, fs, process::exit};

use ast_printer::AstPrinter;
use parser::Parser;
use scanner::Scanner;
use token::Token;
use token_type::TokenType;

static mut HAD_ERROR: bool = false;

mod ast_printer;
mod expr;
mod interpreter;
mod literal_object;
mod parser;
mod scanner;
mod token;
mod token_type;

fn main() {
    let args = env::args();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        let file_name = args.last();
        match file_name {
            Some(file) => run_file(&file),
            None => {}
        }
        // run_file(args.)
    } else {
        run_prompt();
    }
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
    // Todo: add lifetimes to avoid clone
    let mut parser = Parser::new(tokens.clone());
    let expression = parser.parse().unwrap();

    unsafe {
        if HAD_ERROR {
            return;
        }
    }

    let printer = AstPrinter;
    println!("{}", AstPrinter::print(&printer, &expression))
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.is_empty() {
            break;
        }

        println!("Input: {}", input);
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

fn token_error(token: &Token, message: &'static str) {
    if token.type_ == TokenType::EOF {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", &token.lexeme), message);
    }
}
