use crate::lexer::*;

#[derive(Debug)]
pub enum ASTNode {
    NUM(f64),
    VAR(String),
    FUN(String, Vec<ASTNode>),
    UNA(Operator, Box<ASTNode>),
    BIN(Operator, Box<ASTNode>, Box<ASTNode>),
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn parse_item(&mut self) -> Result<ASTNode, String> {
        let token: Token = self.lexer.next_token();

        match token {
            Token::NUM(x) => Ok(ASTNode::NUM(x)),
            Token::IDE(x) => {
                if let Token::LPA = self.lexer.peek() {
                    let mut args: Vec<ASTNode> = Vec::new();
                    self.lexer.next_token(); // Consume LPA
                    loop {
                        args.push(self.parse_expression()?);
                        if let Token::COM = self.lexer.peek() {
                            self.lexer.next_token(); // Consume COM
                        } else {
                            break;
                        }
                    }
                    self.lexer.next_token(); // Consume RPA
                    Ok(ASTNode::FUN(x, args))
                } else {
                    Ok(ASTNode::VAR(x))
                }
            }
            Token::LPA => {
                let expr: ASTNode = self.parse_expression()?;
                self.lexer.next_token(); // Consume RPA
                Ok(expr)
            }
            Token::OPE(x) => match x {
                Operator::ADD | Operator::SUB => Ok(ASTNode::UNA(x, Box::new(self.parse_item()?))),
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
        while let Token::OPE(op) = self.lexer.peek() {
            if let Operator::EXP = op {
                self.lexer.next_token();
                item_node = ASTNode::BIN(op, Box::new(item_node), Box::new(self.parse_factor()?));
            } else {
                break;
            }
        }
        Ok(item_node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut factor_node: ASTNode = self.parse_factor()?;

        while let Token::OPE(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::MUL | Operator::DIV | Operator::MOD => {
                    if let Token::OPE(op) = self.lexer.next_token() {
                        factor_node =
                            ASTNode::BIN(op, Box::new(factor_node), Box::new(self.parse_factor()?));
                    }
                }
                _ => break,
            }
        }

        Ok(factor_node)
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut term_node: ASTNode = self.parse_term()?;

        while let Token::OPE(op_peek) = self.lexer.peek() {
            match op_peek {
                Operator::ADD | Operator::SUB => {
                    if let Token::OPE(op) = self.lexer.next_token() {
                        term_node =
                            ASTNode::BIN(op, Box::new(term_node), Box::new(self.parse_term()?));
                    }
                }
                _ => break,
            }
        }

        Ok(term_node)
    }

    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }
}
