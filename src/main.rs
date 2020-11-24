use std::io::Write;
use std::io;

#[derive(Debug)]
enum TokenType { NUM, ADD, SUB, MUL, DIV, LPA, RPA }

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String
}

fn analyze(text: String) -> Vec<Token> {
    let mut num_buffer: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();

    for c in text.chars() {
        if c.is_digit(10) || c == '.' {
            num_buffer += &c.to_string();
            continue;
        } else if !num_buffer.is_empty() {
            tokens.push(Token{
                token_type: TokenType::NUM,
                lexeme: num_buffer
            });
            num_buffer = String::new();
        }

        let token_type = match c {
            '+' => Some(TokenType::ADD),
            '-' => Some(TokenType::SUB),
            '*' => Some(TokenType::MUL),
            '/' => Some(TokenType::DIV),
            '(' => Some(TokenType::LPA),
            ')' => Some(TokenType::RPA),
            _ => None
        };

        match token_type {
            Some(t) => tokens.push(Token {token_type: t, lexeme: c.to_string()}),
            None => ()
        };
    }

    return tokens;
}

fn main() {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); 

        let tokens: Vec<Token> = analyze(input);
        for token in tokens.iter() {
            println!("{:?}", token);
        }
    }
}