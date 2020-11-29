use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Token {
    NUM(f64),
    OPE(Operator),
    IDE(String),
    LPA,
    RPA,
    COM,
    EOF,
}
#[derive(Debug, Clone)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EXP,
}

pub struct Lexer {
    tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        self.tokens.pop_back().unwrap_or(Token::EOF)
    }

    pub fn peek(&self) -> Token {
        self.tokens.back().unwrap_or(&Token::EOF).clone()
    }

    pub fn new(text: String) -> Lexer {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        let mut num_buffer: String = String::new();
        let mut ide_buffer: String = String::new();

        for character in text.chars() {
            if character.is_digit(10) || character == '.' {
                num_buffer += &character.to_string();
                continue;
            } else if !num_buffer.is_empty() {
                tokens.push_front(Token::NUM(
                    num_buffer
                        .parse::<f64>()
                        .expect("Failed to parse String to f64"),
                ));
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
                ',' => tokens.push_front(Token::COM),
                _ => (),
            }
        }

        return Lexer { tokens };
    }
}
