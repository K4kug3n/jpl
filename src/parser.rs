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

	fn get_precedence(kind: &TokenKind) -> i32 {
		match kind {
			TokenKind::LogicalAnd => 0,
			TokenKind::LogicalOr => 0,
			TokenKind::LowerOrEq => 1,
			TokenKind::GreaterOrEq => 1,
			TokenKind::Lower => 1,
			TokenKind::Greater => 1,
			TokenKind::Equal => 1,
			TokenKind::NotEqual => 1,
			TokenKind::Add => 2,
			TokenKind::Minus => 2,
			TokenKind::Product => 3,
			TokenKind::Divide => 3,
			_ => panic!("Not a op")
		}
	}

	fn is_op(kind: &TokenKind) -> bool {
		*kind == TokenKind::Add || *kind == TokenKind::Minus || *kind == TokenKind::Product || *kind == TokenKind::Divide ||
		*kind == TokenKind::LowerOrEq || *kind == TokenKind::GreaterOrEq || *kind == TokenKind::Lower || *kind == TokenKind::Greater ||
		*kind == TokenKind::Equal || *kind == TokenKind::NotEqual || *kind == TokenKind::LogicalAnd || *kind == TokenKind::LogicalOr
	}

	fn to_op(kind: &TokenKind) -> Operator {
		match kind {
			TokenKind::LogicalAnd => Operator::LogicalAnd,
			TokenKind::LogicalOr => Operator::LogicalOr,
			TokenKind::LowerOrEq => Operator::LowerOrEq,
			TokenKind::GreaterOrEq => Operator::GreaterOrEq,
			TokenKind::Lower => Operator::Lower,
			TokenKind::Greater => Operator::Greater,
			TokenKind::Equal => Operator::Equal,
			TokenKind::NotEqual => Operator::NotEqual,
			TokenKind::Add => Operator::Add,
			TokenKind::Minus => Operator::Minus,
			TokenKind::Product => Operator::Product,
			TokenKind::Divide => Operator::Divide,
			_ => panic!("Not a op")
		}
	}

	fn advance(&mut self) {
		self.current_token = self.lexer.next_token();
	}

	fn eat(&mut self, kind: TokenKind) {
		if self.current_token.kind != kind {
			panic!("Can't eat this token kind");
		}

		self.advance();
	}

	fn expect(&self, kind: TokenKind) -> bool {
		self.current_token.kind == kind
	}

	fn primary(&mut self) -> Node {
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

				let lhs = self.primary();
				let exp = self.parse_expression(lhs, 0);

				self.eat(TokenKind::RParenthesis);

				return exp;
			},
			_ => {
				panic!("No valid primary token kind");
			}
		}
	}

	fn parse_expression(&mut self, mut lhs: Node, precedence: i32) -> Node {
		while Self::is_op(&self.current_token.kind) && Self::get_precedence(&self.current_token.kind) >= precedence {
			let op = self.current_token.clone();
			self.advance();

			let mut rhs = self.primary();
			while Self::is_op(&self.current_token.kind) && Self::get_precedence(&self.current_token.kind) > Self::get_precedence(&op.kind) {
				rhs = self.parse_expression(rhs, Self::get_precedence(&op.kind) + 1);
			}

			lhs = Node::BinaryOp { 
				op: Self::to_op(&op.kind),
				left: Box::new(lhs), 
				right: Box::new(rhs)
			};
		}

		lhs
	}

	fn instr(&mut self) -> Node {
		match self.current_token.kind {
			TokenKind::Let => {
				self.eat(TokenKind::Let);
				
				let name = self.current_token.value.clone();

				self.eat(TokenKind::Identifier);
				self.eat(TokenKind::Assign);

				let lhs = self.primary();
				let value = self.parse_expression(lhs, 0);

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

				let lhs = self.primary();
				let value = self.parse_expression(lhs, 0);

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