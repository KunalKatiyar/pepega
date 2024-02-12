mod lexer;

use std::env::args;
use std::fs;
use std::process;

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

pub fn report(line: i32, where_error: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, where_error, message);
    panic!("Error");
}

fn run(contents: String) -> Result<(), String>{
    let mut lexer = lexer::lexer::Lexer::new(contents);
    lexer.scan_tokens();
    for token in lexer.tokens {
        println!("{:?}", token);
    }
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
