use std::io::{self, Write};
use std::process;
use std::collections::VecDeque;

use colour::yellow;

#[derive(Debug, Clone)]
enum Token { NUM(f64), OPE(Operator), IDE(String), LPA, RPA, EOF }
#[derive(Debug, Clone)]
enum Operator { ADD, SUB, MUL, DIV, MOD, EXP }

struct Lexer {
    tokens: VecDeque<Token>
}

impl Lexer {
    fn next_token(&mut self) -> Token {
        self.tokens.pop_back().unwrap_or(Token::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens.back().unwrap_or(&Token::EOF).clone()
    }

    fn new(text: String) -> Lexer {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        let mut num_buffer: String = String::new();
        let mut ide_buffer: String = String::new();

        for character in text.chars() {
            if character.is_digit(10) || character == '.' {
                num_buffer += &character.to_string();
                continue;
            } else if !num_buffer.is_empty() {
                tokens.push_front(Token::NUM(num_buffer.parse::<f64>().expect("Failed to parse String to f64")));
                num_buffer.clear();
            }

            if character.is_alphabetic() {
                ide_buffer += &character.to_string();
                continue;
            } else if !ide_buffer.is_empty() {
                tokens.push_front(Token::IDE(ide_buffer.clone()));
                ide_buffer.clear();
            }

            match character {
                '+' => tokens.push_front(Token::OPE(Operator::ADD)),
                '-' => tokens.push_front(Token::OPE(Operator::SUB)),
                '*' => tokens.push_front(Token::OPE(Operator::MUL)),
                '/' => tokens.push_front(Token::OPE(Operator::DIV)),
                '%' => tokens.push_front(Token::OPE(Operator::MOD)),
                '^' => tokens.push_front(Token::OPE(Operator::EXP)),
                '(' => tokens.push_front(Token::LPA),
                ')' => tokens.push_front(Token::RPA),
                _ => ()
            } 
        }

        return Lexer {tokens};
    }
}

#[derive(Debug)]
enum ASTNode { NUM(f64), VAR(String), FUN(String, Box<ASTNode>), UNA(Operator, Box<ASTNode>), BIN(Operator, Box<ASTNode>, Box<ASTNode>) }

struct Parser {
    lexer: Lexer
}

impl Parser {
    fn parse_item(&mut self) -> Result<ASTNode, String> {
        let token: Token = self.lexer.next_token();

        match token {
            Token::NUM(x) => Ok(ASTNode::NUM(x)),
            Token::IDE(x) => {
                if let Token::LPA = self.lexer.peek() {
                    self.lexer.next_token();
                    let expr: ASTNode = self.parse_expression()?;
                    self.lexer.next_token(); // Consume RPA
                    Ok(ASTNode::FUN(x, Box::new(expr)))
                } else {
                    Ok(ASTNode::VAR(x))
                }
            },
            Token::LPA => {
                let expr: ASTNode = self.parse_expression()?;
                self.lexer.next_token(); // Consume RPA
                Ok(expr)
            },
            Token::OPE(x) => {
                match x {
                    Operator::ADD | Operator::SUB => Ok(ASTNode::UNA(x, Box::new(self.parse_item()?))),
                    _ => Err(String::from("Parse error"))
                }
            },
            Token::EOF => {
                println!("Encountered Token::EOF");
                Err(String::from("Parse error"))
            },
            _ => Err(String::from("Parse error"))
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let mut item_node: ASTNode = self.parse_item()?;
        while let Token::OPE(op) = self.lexer.peek() {
            if let Operator::EXP = op {
                self.lexer.next_token();
                item_node = ASTNode::BIN(op, Box::new(item_node), Box::new(self.parse_factor()?));
            } else {
                break;
            }
        }
        Ok(item_node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut factor_node: ASTNode = self.parse_factor()?;

        while let Token::OPE(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::MUL | Operator::DIV | Operator::MOD => {
                    if let Token::OPE(op) = self.lexer.next_token() {
                        factor_node = ASTNode::BIN(op, Box::new(factor_node), Box::new(self.parse_factor()?));
                    }
                },
                _ => break
            }
        }

        Ok(factor_node)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut term_node: ASTNode = self.parse_term()?;

        while let Token::OPE(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::ADD | Operator::SUB => {
                    if let Token::OPE(op) = self.lexer.next_token() {
                        term_node = ASTNode::BIN(op, Box::new(term_node), Box::new(self.parse_term()?));
                    }
                },
                _ => break
            }
        }

        Ok(term_node)
    }
}

fn evaluate_ast(node: ASTNode) -> f64 {
    match node {
        ASTNode::NUM(x) => x,
        ASTNode::BIN(x, y, z) => {
            let left_node: f64 = evaluate_ast(*y);
            let right_node: f64 = evaluate_ast(*z);
            match x {
                Operator::ADD => left_node + right_node,
                Operator::SUB => left_node - right_node,
                Operator::MUL => left_node * right_node,
                Operator::DIV => left_node / right_node,
                Operator::MOD => left_node % right_node,
                Operator::EXP => left_node.powf(right_node)
            }
        },
        ASTNode::UNA(x, y) => {
            let child: f64 = evaluate_ast(*y);
            match x {
                Operator::SUB => -child,
                _ => child // ASTNode::UNA can only be Operator::ADD or Operator::SUB
            }
        },
        ASTNode::FUN(x, y) => {
            let child: f64 = evaluate_ast(*y);
            match x.as_str() {
                "abs" => child.abs(),
                "sin" => child.sin(),
                "cos" => child.sin(),
                "tan" => child.tan(),
                "asin" => child.asin(),
                "acos" => child.acos(),
                "atan" => child.atan(),
                "ln" => child.ln(),
                "sqrt" => child.sqrt(),
                "cbrt" => child.cbrt(),
                "exp" => child.exp(),
                "floor" => child.floor(),
                "ceil" => child.ceil(),
                "round" => child.round(),
                "trunc" => child.trunc(),
                "fract" => child.fract(),
                "signum" => child.signum(),
                "sinh" => child.sinh(),
                "cosh" => child.cosh(),
                "tanh" => child.tanh(),
                "asinh" => child.asinh(),
                "acosh" => child.acosh(),
                "atanh" => child.atanh(),
                _ => child
            }
        },
        ASTNode::VAR(x) => {
            match x.as_str() {
                "pi" => std::f64::consts::PI,
                "e" => std::f64::consts::E,
                "tau" => std::f64::consts::TAU,
                "phi" => 1.6180339887498948482045868343656381f64,
                _ => 0f64
            }
        },
        _ => 0f64
    }
}

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
                let mut parser: Parser = Parser {lexer};
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