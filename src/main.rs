use std::io;
use std::io::Write;
use std::process;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
enum Operator { ADD, SUB, MUL, DIV, EXP }

#[derive(Clone, Debug)]
enum Token { NUM(f64), OPE(Operator), FUN(String), LPR, RPR }

fn precedence(operator: Operator) -> u8 {
    match operator {
        Operator::ADD => 1,
        Operator::SUB => 1,
        Operator::MUL => 2,
        Operator::DIV => 2,
        Operator::EXP => 3,
    }
}

fn analyze(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut num_buffer: String = String::new();

    for character in text.chars() {
        if character.is_digit(10) || character == '.' {
            num_buffer += &character.to_string();
            continue;
        }

        if !num_buffer.is_empty() {
            tokens.push(Token::NUM(num_buffer.parse::<f64>().expect("Failed to parse String to f64")));
            num_buffer.clear();
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
            Token::OPE(x) => {
                loop {
                    if let Some(v) = stack.last() {
                        if let Token::OPE(y) = v {
                            if precedence(y.clone()) > precedence(x.clone()) || (precedence(y.clone()) == precedence(x.clone()) && x.clone() != Operator::EXP) {
                                queue.push_back(stack.pop().unwrap());
                            } else {
                                break;
                            }
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
                let a: f64 = num_stack.pop().unwrap();
                let b: f64 = num_stack.pop().unwrap();
                match x {
                    Operator::ADD => num_stack.push(a + b),
                    Operator::SUB => num_stack.push(b - a),
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

        let mut show_rpn: bool = false;

        if input.contains(":help") {
            println!(":exit to close");
            println!(":help for commands");
            println!(":rpn <expr> to show rpn queue");
            continue;
        } else if input.contains(":exit") {
            process::exit(0);
        } else if input.contains(":rpn") {
            show_rpn = true;
        }

        let tokens: VecDeque<Token> = shunting_yard(analyze(input));
        if show_rpn {
            let mut id: i32 = 1;
            for token in tokens.iter() {
                println!("{}: {:?}", id, token);
                id += 1;
            }
            print!("\n");
        }
        println!("{}", evaluate_rpn(tokens));
    }
}