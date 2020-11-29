mod lexer;
mod parser;
mod interpreter;

use lexer::*;
use parser::*;
use interpreter::*;

use std::io::{self, Write};
use std::process;

use colour::yellow;

fn main() {
    println!("cli-calc version 1.0.0\ntype :help for commands");

    let mut debug: bool = false;

    loop {
        yellow!(">> "); // print!() yellow
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            ":debug" => {
                debug = !debug;
                println!("debug = {}", debug);
            },
            ":exit" => process::exit(0),
            ":help" => println!(":help\n:debug\n:exit"),
            _ => {
                let lexer: Lexer = Lexer::new(input);
                let mut parser: Parser = Parser::new(lexer);
                match parser.parse_expression() {
                    Ok(n) => {
                        if debug { println!("{:?}", n); }
                        println!("{}", evaluate_ast(n));
                    },
                    Err(m) => println!("{}", m)
                }
            }
        }
    }
}