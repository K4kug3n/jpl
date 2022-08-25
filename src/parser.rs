use core::panic;

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
pub enum Node {
	Int(i64),
	Float(f64),
	Identifier(String),
	BinaryOp {
		op: Operator,
		left: Box<Node>,
		right: Box<Node>
	},
	VarDeclaration {
		name: String,
		value: Box<Node>
	},
	VarAssignation {
		name: String,
		value: Box<Node>
	},
	InstructionList {
		current: Box<Node>,
		next: Box<Option<Node>>
	}
}

impl Visitable for Node {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Node::Int(x) => visitor.visit_int(*x),
            Node::Float(x) => visitor.visit_float(*x),
			Node::Identifier(name) => visitor.visit_identifier(name),
            Node::BinaryOp { op, left, right } => visitor.visit_binary_op(op, left, right),
			Node::VarDeclaration { name, value } => visitor.visit_var_declaration(name, value),
			Node::VarAssignation { name, value } => visitor.visit_var_assignation(name, value),
			Node::InstructionList { current, next } => visitor.visit_instruction_list(current, next),
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

	fn t(&mut self) -> Node {
		let left = self.f();
		
		match self.g(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	fn f(&mut self) -> Node {
		match self.current_token.kind {
			TokenKind::Integer => {
				let value = self.current_token.value.parse::<i64>().unwrap();

				self.eat(TokenKind::Integer);

				return Node::Int(value);
			},
			TokenKind::Float => {
				let value = self.current_token.value.parse::<f64>().unwrap();

				self.eat(TokenKind::Float);

				return Node::Float(value);
			},
			TokenKind::Identifier => {
				let name = self.current_token.value.clone();

				self.eat(TokenKind::Identifier);

				return Node::Identifier(name);
			},
			TokenKind::LParenthesis => {
				self.eat(TokenKind::LParenthesis);

				let exp = self.e();

				self.eat(TokenKind::RParenthesis);

				return exp;
			},
			_ => {
				panic!("F : no valid token kind");
			}
		}
	}

	fn g(&mut self, previous : Node) -> Option<Node> {
		if self.expect(TokenKind::Product) {
			self.eat(TokenKind::Product);

			return Some(Node::BinaryOp { 
				op:Operator::Product, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Divide) {
			self.eat(TokenKind::Divide);

			return Some(Node::BinaryOp { 
				op:Operator::Divide, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn d(&mut self, previous : Node) -> Option<Node> {
		if self.expect(TokenKind::Add) {
			self.eat(TokenKind::Add);

			return Some(Node::BinaryOp { 
				op:Operator::Add, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Minus) {
			self.eat(TokenKind::Minus);

			return Some(Node::BinaryOp { 
				op:Operator::Minus, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn e(&mut self) -> Node {
		let left = self.t();
		
		match self.d(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	fn instr(&mut self) -> Node {
		match self.current_token.kind {
			TokenKind::Let => {
				self.eat(TokenKind::Let);
				
				let name = self.current_token.value.clone();

				self.eat(TokenKind::Identifier);
				self.eat(TokenKind::Equal);

				let value = self.e();

				self.eat(TokenKind::Semilicon);

				Node::VarDeclaration { 
					name: name,
					value: Box::new(value)
				}
			},
			TokenKind::Identifier => {
				let name = self.current_token.value.clone();
				self.eat(TokenKind::Identifier);

				self.eat(TokenKind::Equal);

				let value = self.e();

				self.eat(TokenKind::Semilicon);

				Node::VarAssignation { 
					name: name,
					value: Box::new(value)
				}
			}
			_ => { panic!("instr : no valid token kind"); }
		}
	}

	fn list_instr(&mut self) -> Option<Node> {
		if self.expect(TokenKind::Eof) {
			return None;
		}

		Some(Node::InstructionList { 
			current: Box::new(self.instr()),
			next: Box::new(self.list_instr()) 
		})
	}

	fn prgm(&mut self) -> Option<Node> {
		self.list_instr()
	}

	pub fn ast(&mut self) -> Option<Node> {
		return self.prgm();
	}
}