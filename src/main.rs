use std::io;
use std::io::Write;
use std::process;

#[derive(Debug)]
enum Token { NUM(f64), ADD, SUB, MUL, DIV, EXP, FUN(String), LPR, RPR }

fn precedence(operator: Token) -> u8 {
    match operator {
        Token::ADD => 1,
        Token::SUB => 1,
        Token::MUL => 2,
        Token::DIV => 2,
        Token::EXP => 3,
        _ => 0
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
            '+' => tokens.push(Token::ADD),
            '-' => tokens.push(Token::SUB),
            '*' => tokens.push(Token::MUL),
            '/' => tokens.push(Token::DIV),
            '^' => tokens.push(Token::EXP),
            '(' => tokens.push(Token::LPR),
            ')' => tokens.push(Token::RPR),
            _ => ()
        }
    }

    return tokens;
}

fn main() {
    println!("cli-calc version 1.0");
    println!(":help for commands");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if input.contains(":help") {
            println!(":exit to close");
            println!(":help for commands");
            continue;
        } else if input.contains(":exit") {
            process::exit(0);
        }

        let tokens: Vec<Token> = analyze(input);
        for token in tokens.iter() {
            println!("{:?}", token);
        }
    }
}