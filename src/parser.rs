use crate::lexer::{Lexer, Token, TokenKind};
use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Clone)]
pub enum Operator {
	Add,
	Minus,
	Product,
	Divide
}

#[derive(Debug, Clone)]
pub enum NodeExpression {
	Int(i64),
	Float(f64),
	BinaryOp {
		op: Operator,
		left: Box<NodeExpression>,
		right: Box<NodeExpression>
	}
}

impl Visitable for NodeExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            NodeExpression::Int(x) => visitor.visit_int(*x),
            NodeExpression::Float(x) => visitor.visit_float(*x),
            NodeExpression::BinaryOp { op, left, right } => visitor.visit_binary_op(op, left, right)
        }
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

	fn eat(&mut self, kind: TokenKind) {
		if self.current_token.kind != kind {
			panic!("Can't eat this token kind");
		}

		self.current_token = self.lexer.next_token();
	}

	fn expect(&self, kind: TokenKind) -> bool {
		self.current_token.kind == kind
	}

	fn t(&mut self) -> NodeExpression {
		let left = self.f();
		
		match self.g(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	fn f(&mut self) -> NodeExpression {
		if self.expect(TokenKind::Integer) {
			let value = self.current_token.value.parse::<i64>().unwrap();

			self.eat(TokenKind::Integer);

			return NodeExpression::Int(value);
		}
		else if self.expect(TokenKind::Float) {
			let value = self.current_token.value.parse::<f64>().unwrap();

			self.eat(TokenKind::Float);

			return NodeExpression::Float(value);
		}
		else if self.expect(TokenKind::LParenthesis) {
			self.eat(TokenKind::LParenthesis);

			let exp = self.e();

			self.eat(TokenKind::RParenthesis);

			return exp;
		}

		panic!("F : no valid token kind");
	}

	fn g(&mut self, previous : NodeExpression) -> Option<NodeExpression> {
		if self.expect(TokenKind::Product) {
			self.eat(TokenKind::Product);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Product, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Divide) {
			self.eat(TokenKind::Divide);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Divide, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn d(&mut self, previous : NodeExpression) -> Option<NodeExpression> {
		if self.expect(TokenKind::Add) {
			self.eat(TokenKind::Add);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Add, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Minus) {
			self.eat(TokenKind::Minus);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Minus, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn e(&mut self) -> NodeExpression {
		let left = self.t();
		
		match self.d(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	pub fn ast(&mut self) -> NodeExpression {
		return self.e();
	}
}