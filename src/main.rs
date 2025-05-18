use std::{collections::HashMap, env, fs, process::exit};

use ast_printer::AstPrinter;
use interpreter::{Interpreter, RuntimeError};
use parser::Parser;
use scanner::Scanner;
use token::Token;
use token_type::TokenType;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod literal_object;
mod parser;
mod scanner;
mod stmt;
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
    let file = fs::read_to_string(path).expect("Failed to read file");
    run(&file);

    unsafe {
        if HAD_ERROR {
            exit(65);
        }
        if HAD_RUNTIME_ERROR {
            exit(70);
        }
    }
}
fn run(source: &str) {
    let mut scanner = Scanner::default(source);
    let tokens = scanner.scan_tokens();
    let mut interpreter: Interpreter = Interpreter::new();
    // Todo: add lifetimes to avoid clone

    let mut parser = Parser::new(tokens.clone());
    let statements = parser.parse();

    match statements {
        Ok(stmts) => {
            let result = interpreter.interpret(stmts);
            match result {
                Ok(_) => {}
                Err(error) => panic!("{}", error.message),
            }
        }
        Err(error) => {
            unsafe {
                if HAD_ERROR {
                    return;
                }
            }
            panic!("{}", error.message);
        }
    }

    // match expression {
    //     Ok(expr) => {
    //         let interpreter: Interpreter = Interpreter::new();
    //         unsafe {
    //             if HAD_ERROR {
    //                 return;
    //             }
    //         }
    //         interpreter.interpret(expr);
    //     }
    //     Err(err) => println!("{:?}", err),
    // }
}

fn run_prompt() {
    let mut interpreter: Interpreter = Interpreter::new();
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
        let mut scanner = Scanner::default(&input);
        let tokens = scanner.scan_tokens();

        // Todo: add lifetimes to avoid clone
        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse();

        match statements {
            Ok(stmts) => {
                let result = interpreter.interpret(stmts);
                match result {
                    Ok(result) => {}
                    Err(error) => panic!("{}", error.message),
                }
            }
            Err(error) => {
                unsafe {
                    if HAD_ERROR {
                        return;
                    }
                }
                panic!("{}", error.message);
            }
        }
    }
}

pub fn error(line: u64, message: &str) {
    report(line, "", message)
}

pub fn run_time_error(error: RuntimeError) {
    println!("{} \n[line {}]", error.message, error.token.line);
    unsafe { HAD_RUNTIME_ERROR = true }
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
