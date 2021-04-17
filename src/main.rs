mod interpreter;
mod lexer;
mod parser;

use interpreter::*;
use lexer::*;
use parser::*;

use std::{
    io::{self, Write},
    process,
};

use colour::yellow;

fn main() {
    println!("cli-calc version 1.1\ntype :help for commands");

    let mut debug: bool = false;

    loop {
        yellow!(">> "); // print!() yellow
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            ":debug" => {
                debug = !debug;
                println!("debug = {}", debug);
            },
            ":exit" => process::exit(0),
            ":help" => {
                println!("\
                    :help\n\
                    :funcs\n\
                    :debug\n\
                    :exit\
                ")
            }
            ":funcs" =>{
                println!("\
                    abs(x)\n\
                    acos(x)\n\
                    acosh(x)\n\
                    asin(x)\n\
                    asinh(x)\n\
                    atan(x)\n\
                    atan2(x, y)\n\
                    atanh(x)\n\
                    ceil(x)\n\
                    ncbrt(x)\n\
                    cos(x)\n\
                    cosh(x)\n\
                    deg(x)\n\
                    exp(x)\n\
                    fact(x)\n\
                    floor(x)\n\
                    fract(x)\n\
                    hypot(x, y)\n\
                    ln(x)\n\
                    log10(x)\n\
                    log2(x)\n\
                    log(base, x)\n\
                    max(x, y)\n\
                    min(x, y)\n\
                    pow(x, exponent)\n\
                    rad(x)\n\
                    round(x)\n\
                    sign(x)\n\
                    sin(x)\n\
                    sinh(x)\n\
                    sqrt(x)\n\
                    sum(...)\n\
                    rad(x)\n\
                    root(root, x)\n\
                    tan(x)\n\
                    tanh(x)\n\
                    trunc(x)\
                ")
            }
            _ => {
                let lexer: Lexer = Lexer::new(input);
                let mut parser: Parser = Parser::new(lexer);
                match parser.parse_from_top() {
                    Ok(n) => {
                        if debug { println!("{:?}", n); }
                        let result = evaluate_ast(n);
                        match result {
                            Ok(r) => {
                                match r {
                                    InterpreterResult::Number(value) => println!("{}", value),
                                    InterpreterResult::Bool(value) => println!("{}", value)
                                }
                            }
                            Err(m) => println!("{}", m)
                        }
                    },
                    Err(m) => println!("{}", m)
                }
            }
        }
    }
}
