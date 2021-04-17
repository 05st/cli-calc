use crate::lexer::*;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Bool(bool),
    Variable(String),
    Function(String, Vec<ASTNode>),
    Unary(Operator, Box<ASTNode>),
    Binary(Operator, Box<ASTNode>, Box<ASTNode>),
    Comparison(Vec<Operator>, Vec<ASTNode>)
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn parse_and_or(&mut self) -> Result<ASTNode, String> {
        let mut comp: ASTNode = self.parse_comparison()?;
        while let Token::Operator(peek) = self.lexer.peek() {
            match peek {
                Operator::And | Operator::Or => {
                    self.lexer.next_token();
                    comp = ASTNode::Binary(peek, Box::new(comp), Box::new(self.parse_comparison()?));
                }
                _ => break,
            }
        }
        Ok(comp)
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let expr: ASTNode = self.parse_expression()?;
        let mut operators: Vec<Operator> = vec![];
        let mut operands: Vec<ASTNode> = vec![];
        while let Token::Operator(peek) = self.lexer.peek() {
            match peek {
                Operator::Equal | Operator::NotEqual | Operator::Greater | Operator::Lesser | Operator::GreaterEqual | Operator::LesserEqual => {
                    self.lexer.next_token();
                    operators.push(peek);
                    operands.push(self.parse_expression()?);
                }
                _ => break
            }
        }
        if !operators.is_empty() && !operands.is_empty() {
            operands.insert(0, expr);
            return Ok(ASTNode::Comparison(operators, operands));
        }
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut term: ASTNode = self.parse_term()?;
        while let Token::Operator(peek) = self.lexer.peek() {
            match peek {
                Operator::Add | Operator::Subtract => {
                    self.lexer.next_token();
                    term = ASTNode::Binary(peek, Box::new(term), Box::new(self.parse_term()?));
                }
                _ => break,
            }
        }
        Ok(term)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut factor: ASTNode = self.parse_factor()?;
        while let Token::Operator(peek) = self.lexer.peek() {
            match peek {
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    self.lexer.next_token();
                    factor = ASTNode::Binary(peek, Box::new(factor), Box::new(self.parse_factor()?));
                }
                _ => break,
            }
        }
        Ok(factor)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let mut item: ASTNode = if let Token::Operator(op) = self.lexer.peek() {
            match op {
                Operator::Add | Operator::Subtract | Operator::Not => {
                    self.lexer.next_token();
                    ASTNode::Unary(op, Box::new(self.parse_factor()?))
                }
                _ => return Err(String::from("Parse error"))
            }
        } else {
            self.parse_item()?
        };
        while let Token::Operator(peek) = self.lexer.peek() {
            if let Operator::Exponent = peek {
                self.lexer.next_token();
                item = ASTNode::Binary(peek, Box::new(item), Box::new(self.parse_factor()?));
            } else {
                break;
            }
        }
        Ok(item)
    }

    fn parse_item(&mut self) -> Result<ASTNode, String> {
        let token: Token = self.lexer.next_token();
        match token {
            Token::Number(x) => Ok(ASTNode::Number(x)),
            Token::Bool(x) => Ok(ASTNode::Bool(x)),
            Token::Identifier(x) => {
                if let Token::LeftParen = self.lexer.peek() {
                    let mut args: Vec<ASTNode> = Vec::new();
                    self.lexer.next_token(); // Consume LeftParen
                    loop {
                        args.push(self.parse_from_top()?);
                        if let Token::Comma = self.lexer.peek() {
                            self.lexer.next_token(); // Consume Comma
                        } else {
                            break;
                        }
                    }
                    self.lexer.next_token(); // Consume RPA
                    Ok(ASTNode::Function(x, args))
                } else {
                    Ok(ASTNode::Variable(x))
                }
            }
            Token::LeftParen => {
                let expr: ASTNode = self.parse_from_top()?;
                self.lexer.next_token(); // Consume RPA
                Ok(expr)
            }
            Token::EOF => {
                println!("Encountered Token::EOF");
                Err(String::from("Parse error"))
            }
            _ => Err(String::from("Parse error")),
        }
    }

    pub fn parse_from_top(&mut self) -> Result<ASTNode, String> {
        self.parse_and_or()
    }

    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }
}
