use crate::lexer::{Lexer, Token, TokenKind};

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
		if self.expect(TokenKind::INTEGER) {
			let value = self.current_token.value.parse::<i64>().unwrap();

			self.eat(TokenKind::INTEGER);

			return NodeExpression::Int(value);
		}
		else if self.expect(TokenKind::FLOAT) {
			let value = self.current_token.value.parse::<f64>().unwrap();

			self.eat(TokenKind::FLOAT);

			return NodeExpression::Float(value);
		}
		// TODO: parenthesis

		panic!("F : no valid token kind");
	}

	fn g(&mut self, previous : NodeExpression) -> Option<NodeExpression> {
		if self.expect(TokenKind::PRODUCT) {
			self.eat(TokenKind::PRODUCT);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Product, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::DIVIDE) {
			self.eat(TokenKind::DIVIDE);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Divide, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn d(&mut self, previous : NodeExpression) -> Option<NodeExpression> {
		if self.expect(TokenKind::ADD) {
			self.eat(TokenKind::ADD);

			return Some(NodeExpression::BinaryOp { 
				op:Operator::Add, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::MINUS) {
			self.eat(TokenKind::MINUS);

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