use crate::lexer::{Lexer, Token, TokenKind};

#[derive(Debug)]
pub enum Operator {
    Add,
    Minus,
    Product,
    Divide
}

#[derive(Debug)]
pub enum NodeExpression {
    Int(i64),
    Float(f64),
    BinaryOp {
        op: Operator,
        left: Box<NodeExpression>,
        right: Box<NodeExpression>
    }
}


pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token
}

impl Parser<'_> {
    pub fn new<'a>(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let next_token = lexer.next_token();

        Parser {
            lexer: lexer,
            current_token: next_token
        }
    }

    fn eat(&mut self, kind: TokenKind){
        if self.current_token.kind != kind {
            panic!("Can't eat this token kind");
        }

        self.current_token = self.lexer.next_token();
    }

    fn A(&mut self) -> NodeExpression {
        if self.current_token.kind == TokenKind::INTEGER {
            let value = self.current_token.value.parse::<i64>().unwrap();

            self.eat(TokenKind::INTEGER);

            match self.E(NodeExpression::Int(value)) {
                Some(x) => return x,
                None => return NodeExpression::Int(value)
            }
        }
        else if self.current_token.kind == TokenKind::FLOAT {
            let value = self.current_token.value.parse::<f64>().unwrap();

            self.eat(TokenKind::FLOAT);

            match self.E(NodeExpression::Float(value)) {
                Some(x) => return x,
                None => return NodeExpression::Float(value)
            }
        }

        panic!("A : No valid token kind");
    }

    fn E(&mut self, previous : NodeExpression) -> Option<NodeExpression> {
        if self.current_token.kind == TokenKind::ADD {
            self.eat(TokenKind::ADD);

            return Some(NodeExpression::BinaryOp { 
                op: Operator::Add,
                left: Box::new(previous),
                right: Box::new(self.A()) 
            });
        }
        else if self.current_token.kind == TokenKind::MINUS {
            self.eat(TokenKind::MINUS);

            return Some(NodeExpression::BinaryOp { 
                op: Operator::Minus,
                left: Box::new(previous),
                right: Box::new(self.A()) 
            });
        }
        else if self.current_token.kind == TokenKind::PRODUCT {
            self.eat(TokenKind::PRODUCT);

            return Some(NodeExpression::BinaryOp { 
                op: Operator::Product,
                left: Box::new(previous),
                right: Box::new(self.A()) 
            });
        }
        else if self.current_token.kind == TokenKind::DIVIDE {
            self.eat(TokenKind::DIVIDE);

            return Some(NodeExpression::BinaryOp { 
                op: Operator::Divide,
                left: Box::new(previous),
                right: Box::new(self.A()) 
            });
        }
        else if self.current_token.kind == TokenKind::EOF {
            return None;
        }

        panic!("E : No valid token kind");
    }

    pub fn ast(&mut self) -> NodeExpression {
        return self.A();
    }
}