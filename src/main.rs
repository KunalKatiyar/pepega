mod lexer;
mod parser;

use std::env::args;
use std::fs;
use std::process;
use crate::lexer::token::Token;
use crate::parser::parser::Parser;

fn run_prompt() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        run(input).expect("Cannot run input.");
    }
}

pub fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

pub fn error_token(token: Token, message: &str) {
    if token.kind == lexer::token::TokenType::EOF {
        report(token.line as i32, " at end".to_string(), message.to_string());
    } else {
        report(token.line as i32, format!(" at '{}'", token.lexeme), message.to_string());
    }
}

pub fn report(line: i32, where_error: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, where_error, message);
    panic!("Error");
}

fn run(contents: String) -> Result<(), String>{
    let mut lexer = lexer::lexer::Lexer::new(contents);
    lexer.scan_tokens();
    let mut parser = Parser::new(lexer.tokens);
    let expr = parser.parse();
    println!("{}", expr.to_string());
    Ok(())
}

fn run_file(path: String) -> Result<(),String> {
    let content = fs::read_to_string(path).unwrap();
    run(content)
}


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: pepega [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].clone()).expect("Cannot run file.");
    } else {
        println!("Pepega 0.1.0: >>>> Interactive Mode <<<<");
        run_prompt();
    }
}
