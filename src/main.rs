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
            ":help" => println!(":help\n:funcs\n:debug\n:exit"),
            ":funcs" => println!("abs(x)\nacos(x)\nacosh(x)\nasin(x)\nasinh(x)\natan(x)\natanh(x)\nceil(x)\ncbrt(x)\ncos(x)\ncosh(x)\ndeg(x)\nexp(x)\nfloor(x)\nfract(x)\nhypot(x, y)\nln(x)\nlog(x)\nlogn(base, x)\nmax(x, y)\nmin(x, y)\npow(x, exponent)\nrad(x)\nround(x)\nsign(x)\nsin(x)\nsinh(x)\nsqrt(x)\nsum(...)\nrad(x)\nroot(root, x)\ntan(x)\ntanh(x)\ntrunc(x)"),
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