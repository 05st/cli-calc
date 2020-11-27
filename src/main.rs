use std::io;
use std::io::Write;
use std::process;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
enum Operator { ADD, SUB, MUL, DIV, EXP }

#[derive(Clone, Debug)]
enum Token { NUM(f64), OPE(Operator), IDN(String), LPR, RPR }

impl Operator {
    fn precedence(self) -> u8 {
        match self {
            Operator::ADD => 1,
            Operator::SUB => 1,
            Operator::MUL => 2,
            Operator::DIV => 2,
            Operator::EXP => 3
        }
    }

    fn is_left_associative(self) -> bool {
        match self {
            Operator::ADD => true,
            Operator::SUB => true,
            Operator::MUL => true,
            Operator::DIV => true,
            Operator::EXP => false
        }
    }

    fn display(self) -> String {
        match self {
            Operator::ADD => "+",
            Operator::SUB => "-",
            Operator::MUL => "*",
            Operator::DIV => "/",
            Operator::EXP => "^"
        }.to_string()
    }
}

impl Token {
    fn display(self) -> String {
        match self {
            Token::NUM(x) => x.to_string(),
            Token::OPE(x) => return x.display(),
            Token::IDN(x) => return x,
            Token::LPR => "(".to_string(),
            Token::RPR => ")".to_string()
        }
    }
}

fn analyze(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_buffer: String = String::new();
    let mut idn_buffer: String = String::new();

    for character in text.chars() {
        if character.is_digit(10) || character == '.' {
            num_buffer += &character.to_string();
            continue;
        }

        if !num_buffer.is_empty() {
            tokens.push(Token::NUM(num_buffer.parse::<f64>().expect("Failed to parse String to f64")));
            num_buffer.clear();
        }

        if character.is_alphabetic() {
            idn_buffer += &character.to_string();
            continue;
        }

        if !idn_buffer.is_empty() {
            tokens.push(Token::IDN(idn_buffer.clone()));
            idn_buffer.clear();
        }

        match character {
            '+' => tokens.push(Token::OPE(Operator::ADD)),
            '-' => tokens.push(Token::OPE(Operator::SUB)),
            '*' => tokens.push(Token::OPE(Operator::MUL)),
            '/' => tokens.push(Token::OPE(Operator::DIV)),
            '^' => tokens.push(Token::OPE(Operator::EXP)),
            '(' => tokens.push(Token::LPR),
            ')' => tokens.push(Token::RPR),
            _ => ()
        }
    }

    return tokens;
}

fn shunting_yard(tokens: Vec<Token>) -> VecDeque<Token> {
    let mut queue: VecDeque<Token> = VecDeque::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::NUM(_) => queue.push_back(token.clone()),
            Token::OPE(op1) => {
                while let Some(token) = stack.last() {
                    if let Token::OPE(op2) = token {
                        let op2_prec: u8 = op2.clone().precedence();
                        let op1_prec: u8 = op1.clone().precedence();
                        if op2_prec > op1_prec || (op2_prec == op1_prec && op1.clone().is_left_associative()) {
                            queue.push_back(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                stack.push(token.clone());
            },
            Token::LPR => stack.push(token.clone()),
            Token::RPR => {
                loop {
                    if let Token::LPR = stack.last().unwrap() {
                        stack.pop().unwrap();
                        break;
                    } else {
                        queue.push_back(stack.pop().unwrap());
                    }
                }
            },
            _ => (),
        }
    }

    while stack.len() > 0 {
        queue.push_back(stack.pop().unwrap());
    }

    return queue;
}

fn evaluate_rpn(tokens: VecDeque<Token>) -> f64 {
    let mut num_stack: Vec<f64> = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::NUM(x) => num_stack.push(x.clone()),
            Token::OPE(x) => {
                let b: f64 = num_stack.pop().unwrap();
                let a: f64 = num_stack.pop().unwrap();
                match x {
                    Operator::ADD => num_stack.push(a + b),
                    Operator::SUB => num_stack.push(a - b),
                    Operator::MUL => num_stack.push(a * b),
                    Operator::DIV => num_stack.push(a / b),
                    Operator::EXP => num_stack.push(a.powf(b)),
                }
            }
            _ => ()
        }
    }

    return num_stack.pop().unwrap_or(0.0);
}

fn main() {
    println!("cli-calc version 1.0");
    println!(":help for commands");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut show_tokens: bool = false;

        if input.contains(":help") {
            println!(":exit");
            println!(":help");
            println!(":tokens <expr>");
            continue;
        } else if input.contains(":exit") {
            process::exit(0);
        } else if input.contains(":tokens") {
            show_tokens = true;
        }

        let tokens: Vec<Token> = analyze(input);
        if show_tokens {
            let mut id: i32 = 1;
            for token in tokens.iter() {
                println!("{}: {:?} [{}]", id, token, token.clone().display());
                id += 1;
            }
            print!("\n");
        }
        let rpn: VecDeque<Token> = shunting_yard(tokens);
        println!("{}", evaluate_rpn(rpn));
    }
}