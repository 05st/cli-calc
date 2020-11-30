use crate::lexer::*;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Bool(bool),
    Variable(String),
    Function(String, Vec<ASTNode>),
    Unary(Operator, Box<ASTNode>),
    Binary(Operator, Box<ASTNode>, Box<ASTNode>),
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
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
            Token::Operator(x) => match x {
                Operator::Add | Operator::Subtract | Operator::Not => Ok(ASTNode::Unary(x, Box::new(self.parse_item()?))),
                _ => Err(String::from("Parse error")),
            },
            Token::EOF => {
                println!("Encountered Token::EOF");
                Err(String::from("Parse error"))
            }
            _ => Err(String::from("Parse error")),
        }
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        let mut item_node: ASTNode = self.parse_item()?;
        while let Token::Operator(op) = self.lexer.peek() {
            if let Operator::Exponent = op {
                self.lexer.next_token();
                item_node = ASTNode::Binary(op, Box::new(item_node), Box::new(self.parse_factor()?));
            } else {
                break;
            }
        }
        Ok(item_node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut factor_node: ASTNode = self.parse_factor()?;

        while let Token::Operator(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::Multiply | Operator::Divide | Operator::Modulo => {
                    if let Token::Operator(op) = self.lexer.next_token() {
                        factor_node = ASTNode::Binary(op, Box::new(factor_node), Box::new(self.parse_factor()?));
                    }
                }
                _ => break,
            }
        }

        Ok(factor_node)
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut term_node: ASTNode = self.parse_term()?;

        while let Token::Operator(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::Add | Operator::Subtract => {
                    if let Token::Operator(op) = self.lexer.next_token() {
                        term_node = ASTNode::Binary(op, Box::new(term_node), Box::new(self.parse_term()?));
                    }
                }
                _ => break,
            }
        }

        Ok(term_node)
    }

    pub fn parse_equality(&mut self) -> Result<ASTNode, String> {
        let mut expr_node: ASTNode = self.parse_expression()?;

        while let Token::Operator(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::Equal | Operator::NotEqual => {
                    if let Token::Operator(op) = self.lexer.next_token() {
                        expr_node = ASTNode::Binary (op, Box::new(expr_node), Box::new(self.parse_expression()?));
                    }
                }
                _ => break,
            }
        }

        Ok(expr_node)
    }

    pub fn parse_logical_or(&mut self) -> Result<ASTNode, String> {
        let mut equality_node: ASTNode = self.parse_equality()?;

        while let Token::Operator(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::Or => {
                    if let Token::Operator(op) = self.lexer.next_token() {
                        equality_node = ASTNode::Binary(op, Box::new(equality_node), Box::new(self.parse_equality()?));
                    }
                }
                _ => break,
            }
        }

        Ok(equality_node)
    }

    pub fn parse_logical_and(&mut self) -> Result<ASTNode, String> {
        let mut or_node: ASTNode = self.parse_logical_or()?;

        while let Token::Operator(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::And => {
                    if let Token::Operator(op) = self.lexer.next_token() {
                        or_node = ASTNode::Binary(op, Box::new(or_node), Box::new(self.parse_equality()?));
                    }
                }
                _ => break,
            }
        }

        Ok(or_node)
    }

    pub fn parse_from_top(&mut self) -> Result<ASTNode, String> {
        self.parse_logical_and()
    }

    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }
}
