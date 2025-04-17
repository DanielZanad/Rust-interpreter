use scanner::Scanner;
use std::{
    env, fs,
    io::{self},
    process,
};

mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;

static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if env::args().len() == 2 {
        match args.get(1) {
            Some(file_name) => run_file(file_name),
            None => println!("invalid argument"),
        }
    } else {
        run_prompt();
    }

    fn run_file(path: &String) {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        run(contents);

        unsafe {
            // Indicate an error in the exit code;
            if HAD_ERROR {
                process::exit(65)
            }
        }
    }

    fn run_prompt() {
        loop {
            print!("> ");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");

            run(line);
            unsafe {
                HAD_ERROR = false;
            }
        }
    }

    fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens.iter() {
            println!("{}", token.to_string());
        }
    }
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, _where: &str, message: &str) {
    println!("[line  ${line}] Error ${_where} : ${message}",);
    unsafe { HAD_ERROR = true };
}
