use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Identifier(String),
    Bool(bool),
    LeftParen,
    RightParen,
    Comma,
    EOF,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    And,
    Or,
    Not,
    BWLeftShift,
    BWRightShift,
    BWOr,
    BWXor,
    BWAnd,
    BWNot,
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

        let mut skip_next: bool = false;

        for (index, character) in text.chars().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if (character.is_alphabetic() && ide_buffer.is_empty()) || (character.is_alphanumeric() && !ide_buffer.is_empty()) {
                ide_buffer += &character.to_string();
                continue;
            } else if !ide_buffer.is_empty() {
                match ide_buffer.as_str() {
                    "true" => tokens.push_front(Token::Bool(true)),
                    "false" => tokens.push_front(Token::Bool(false)),
                    _ => tokens.push_front(Token::Identifier(ide_buffer.clone())),
                }
                ide_buffer.clear();
            }

            if character.is_digit(10) || character == '.' {
                num_buffer += &character.to_string();
                continue;
            } else if !num_buffer.is_empty() {
                tokens.push_front(Token::Number(num_buffer.parse::<f64>().expect("Failed to parse String to f64")));
                num_buffer.clear();
            }

            let next_character = text.chars().nth(index + 1).unwrap_or('\0'); // Just default to a character we ignore

            match character {
                '>' => tokens.push_front(match next_character {
                    '=' => { skip_next = true; Token::Operator(Operator::GreaterEqual) },
                    '>' => { skip_next = true; Token::Operator(Operator::BWRightShift) },
                    _ => Token::Operator(Operator::Greater),
                }),
                '<' => tokens.push_front(match next_character {
                    '=' => { skip_next = true; Token::Operator(Operator::LesserEqual) },
                    '<' => { skip_next = true; Token::Operator(Operator::BWLeftShift) },
                    _ => Token::Operator(Operator::Lesser),
                }),
                '|' => tokens.push_front(match next_character {
                    '|' => { skip_next = true; Token::Operator(Operator::Or) },
                    '^' => { skip_next = true; Token::Operator(Operator::BWXor) },
                    _ => Token::Operator(Operator::BWOr),
                }),
                '!' => tokens.push_front(if next_character == '=' { skip_next = true; Token::Operator(Operator::NotEqual) } else { Token::Operator(Operator::Not) }),
                '&' => tokens.push_front(if next_character == '&' { skip_next = true; Token::Operator(Operator::And) } else { Token::Operator(Operator::BWAnd) }),
                '=' if next_character == '=' => { skip_next = true; tokens.push_front(Token::Operator(Operator::Equal)) },
                '+' => tokens.push_front(Token::Operator(Operator::Add)),
                '-' => tokens.push_front(Token::Operator(Operator::Subtract)),
                '*' => tokens.push_front(Token::Operator(Operator::Multiply)),
                '/' => tokens.push_front(Token::Operator(Operator::Divide)),
                '%' => tokens.push_front(Token::Operator(Operator::Modulo)),
                '^' => tokens.push_front(Token::Operator(Operator::Exponent)),
                '~' => tokens.push_front(Token::Operator(Operator::BWNot)),
                '(' => tokens.push_front(Token::LeftParen),
                ')' => tokens.push_front(Token::RightParen),
                ',' => tokens.push_front(Token::Comma),
                _ => (),
            }
        }

        Lexer { tokens }
    }
}
