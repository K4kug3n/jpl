use core::panic;

use crate::lexer::{Lexer, Token, TokenKind};
use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Add,
	Minus,
	Product,
	Divide,
	LogicalAnd,
	LogicalOr,
	Equal,
	NotEqual,
	LowerOrEq,
	GreaterOrEq,
	Lower,
	Greater
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Int(i64),
	Float(f64),
	Identifier(String),
	Bool(bool),
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
			Node::Bool(value) => visitor.visit_bool(*value),
            Node::BinaryOp { op, left, right } => visitor.visit_binary_op(op, left, right),
			Node::VarDeclaration { name, value } => visitor.visit_var_declaration(name, value),
			Node::VarAssignation { name, value } => visitor.visit_var_assignation(name, value),
			Node::InstructionList { current, next } => visitor.visit_instruction_list(current, next),
        }
    }
}

fn to_bool(value: &str) -> bool {
	value == "true"
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

	fn f(&mut self) -> Node {
		match self.current_token.kind {
			TokenKind::Integer => {
				let value = self.current_token.value.parse::<i64>().unwrap(); // TODO: Check it

				self.eat(TokenKind::Integer);

				return Node::Int(value);
			},
			TokenKind::Float => {
				let value = self.current_token.value.parse::<f64>().unwrap(); // TODO: Check it

				self.eat(TokenKind::Float);

				return Node::Float(value);
			},
			TokenKind::Bool => {
				let value = to_bool(&self.current_token.value);

				self.eat(TokenKind::Bool);

				return Node::Bool(value);
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

	fn j(&mut self, previous : Node) -> Option<Node> {
		match self.current_token.kind {
			TokenKind::Equal => {
				self.eat(TokenKind::Equal);

				Some(Node::BinaryOp { 
					op: Operator::Equal, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			TokenKind::NotEqual => {
				self.eat(TokenKind::NotEqual);

				Some(Node::BinaryOp { 
					op: Operator::NotEqual, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			TokenKind::LowerOrEq => {
				self.eat(TokenKind::LowerOrEq);

				Some(Node::BinaryOp { 
					op: Operator::LowerOrEq, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			TokenKind::GreaterOrEq => {
				self.eat(TokenKind::GreaterOrEq);

				Some(Node::BinaryOp { 
					op: Operator::GreaterOrEq, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			TokenKind::Greater => {
				self.eat(TokenKind::Greater);

				Some(Node::BinaryOp { 
					op: Operator::Greater, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			TokenKind::Lower => {
				self.eat(TokenKind::Lower);

				Some(Node::BinaryOp { 
					op: Operator::Lower, 
					left: Box::new(previous), 
					right: Box::new(self.e())
				})
			},
			_ => None
		}
	}

	fn h(&mut self) -> Node {
		let left = self.f();

		match self.d(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	fn t(&mut self) -> Node {
		let left = self.h();
		
		match self.g(left.clone()) {
			Some(x) => x,
			None => left
		}
	}

	fn g(&mut self, previous : Node) -> Option<Node> {
		if self.expect(TokenKind::Product) {
			self.eat(TokenKind::Product);

			return Some(Node::BinaryOp { 
				op: Operator::Product, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Divide) {
			self.eat(TokenKind::Divide);

			return Some(Node::BinaryOp { 
				op: Operator::Divide, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::LogicalAnd) {
			self.eat(TokenKind::LogicalAnd);

			return Some(Node::BinaryOp { 
				op: Operator::LogicalAnd, 
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
				op: Operator::Add, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}
		else if self.expect(TokenKind::Minus) {
			self.eat(TokenKind::Minus);

			return Some(Node::BinaryOp { 
				op: Operator::Minus, 
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});

		}
		else if self.expect(TokenKind::LogicalOr) {
			self.eat(TokenKind::LogicalOr);

			return Some(Node::BinaryOp { 
				op: Operator::LogicalOr,
				left: Box::new(previous), 
				right: Box::new(self.e()) 
			});
		}

		return None;
	}

	fn e(&mut self) -> Node {
		let left = self.t();
		
		match self.j(left.clone()) {
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
				self.eat(TokenKind::Assign);

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

				self.eat(TokenKind::Assign);

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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]	
	fn condition_parsing(){
		let mut lexer = Lexer::new("let condition = 2 == 2 || 3.5 != 3.6;");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::VarDeclaration { 
					name: String::from("condition"),
					value: Box::new(
						Node::BinaryOp { 
							op: Operator::LogicalOr,
							left: Box::new(
								Node::BinaryOp { 
									op: Operator::Equal, 
									left: Box::new(Node::Int(2)), 
									right: Box::new(Node::Int(2)) 
								}
							),
							right: Box::new(
								Node::BinaryOp { 
									op: Operator::NotEqual, 
									left: Box::new(Node::Float(3.5)), 
									right: Box::new(Node::Float(3.6))
								}
							) 
						}
					)
				}),
				next: Box::new(None)
			}
		));
	}

	#[test]	
	fn math_exp_parsing(){
		let mut lexer = Lexer::new("let math = 1 * 3 + 4 * 2;");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::VarDeclaration { 
					name: String::from("math"),
					value: Box::new(
						Node::BinaryOp { 
							op: Operator::Add,
							left: Box::new(
								Node::BinaryOp { 
									op: Operator::Product, 
									left: Box::new(Node::Int(1)), 
									right: Box::new(Node::Int(3)) 
								}
							),
							right: Box::new(
								Node::BinaryOp { 
									op: Operator::Product, 
									left: Box::new(Node::Int(4)), 
									right: Box::new(Node::Int(2))
								}
							) 
						}
					)
				}),
				next: Box::new(None)
			}
		));
	}
}