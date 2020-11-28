use std::io;
use std::io::Write;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Token { NUM(f64), OPE(Operator), IDE(String), LPA, RPA, EOF }
#[derive(Debug, Clone)]
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
enum ASTNode { NUM(f64), IDE(String), MON(Operator, Box<ASTNode>), BIN(Operator, Box<ASTNode>, Box<ASTNode>) }

struct Parser {
    lexer: Lexer
}

impl Parser {
    fn parse_item(&mut self) -> Result<ASTNode, String> {
        let token: Token = self.lexer.next_token();

        match token {
            Token::NUM(x) => Ok(ASTNode::NUM(x)),
            Token::IDE(x) => Ok(ASTNode::IDE(x)),
            Token::LPA => {
                let expr: ASTNode = self.parse_expr()?;
                self.lexer.next_token(); // Consume RPA
                Ok(expr)
            },
            Token::EOF => {
                println!("Encountered Token::EOF");
                Err(String::from("Parse error"))
            },
            _ => Err(String::from("Parse error"))
        }
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut item_node: ASTNode = self.parse_item()?;

        while let Token::OPE(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::MUL | Operator::DIV | Operator::MOD => {
                    if let Token::OPE(op) = self.lexer.next_token() {
                        item_node = ASTNode::BIN(op, Box::new(item_node), Box::new(self.parse_item()?));
                    }
                },
                _ => break
            }
        }

        Ok(item_node)
    }

    fn parse_expr(&mut self) -> Result<ASTNode, String> {
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

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let lexer: Lexer = Lexer::new(input);
        let mut parser: Parser = Parser {lexer};
        match parser.parse_expr() {
            Ok(n) => println!("{:?}", n),
            Err(m) => println!("{}", m)
        }
    }
}