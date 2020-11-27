use std::io;
use std::io::Write;
use std::collections::VecDeque;

#[derive(Debug)]
enum Token { NUM(f64), OPE(Operator), IDE(String), LPA, RPA, EOF }
#[derive(Debug)]
enum Operator { ADD, SUB, MUL, DIV, MOD, EXP }

impl Operator {
    fn precedence(self) -> u8 {
        match self {
            Operator::ADD | Operator::SUB => 1,
            Operator::MUL | Operator::DIV | Operator::MOD => 2,
            Operator::EXP => 3
        }
    }

    fn is_left_associative(self) -> bool {
        match self {
            Operator::ADD | Operator::SUB | Operator::MUL | Operator::DIV | Operator::MOD => true,
            Operator::EXP => false
        }
    }
}

struct Lexer {
    tokens: VecDeque<Token>
}

impl Lexer {
    fn next_token(&mut self) -> Token {
        self.tokens.pop_back().unwrap_or(Token::EOF)
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

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut lexer: Lexer = Lexer::new(input);
        
        loop {
            let token: Token = lexer.next_token();
            println!("{:?}", token);
            if let Token::EOF = token {
                break;
            }
        }
    }
}