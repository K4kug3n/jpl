use core::panic;

use crate::operator::{Operator};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::visitor::{Visitable, Visitor};

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
	UnaryOp {
		op: Operator,
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
	IfStatement {
		condition: Box<Node>,
		body: Box<Option<Node>>
	},
	InstructionList {
		current: Box<Node>,
		next: Box<Option<Node>>
	},
	FunctionDeclaration {
		name: String,
		args: Vec<String>,
		body: Box<Option<Node>>
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
			Node::UnaryOp { op, right } => visitor.visit_unary_op(op, right),
			Node::VarDeclaration { name, value } => visitor.visit_var_declaration(name, value),
			Node::VarAssignation { name, value } => visitor.visit_var_assignation(name, value),
			Node::IfStatement { condition, body } => visitor.visit_if_statement(condition, body),
			Node::InstructionList { current, next } => visitor.visit_instruction_list(current, next),
			Node::FunctionDeclaration { name, args, body } => visitor.visit_function_declaration(name, args, body),
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

				self.advance();

				return Node::Int(value);
			},
			TokenKind::Float => {
				let value = self.current_token.value.parse::<f64>().unwrap(); // TODO: Check it

				self.advance();

				return Node::Float(value);
			},
			TokenKind::Bool => {
				let value = to_bool(&self.current_token.value);

				self.advance();

				return Node::Bool(value);
			},
			TokenKind::Identifier => {
				let name = self.current_token.value.clone();

				self.advance();

				return Node::Identifier(name);
			},
			TokenKind::LParenthesis => {
				self.advance();

				let lhs = self.primary();
				let exp = self.parse_expression(lhs, 0);

				self.eat(TokenKind::RParenthesis);

				return exp;
			},
			TokenKind::Operator(op) => {
				match op {
					Operator::Not => {
						self.advance();

						return Node::UnaryOp { 
							op: Operator::Not,
							right: Box::new(self.primary())
						};
					},
					Operator::Minus => {
						self.advance();

						return Node::UnaryOp { 
							op: Operator::Minus,
							right: Box::new(self.primary())
						};
					}
					_ => panic!("No valid primary op")
				}
			}
			_ => {
				panic!("No valid primary token kind");
			}
		}
	}

	fn parse_expression(&mut self, mut lhs: Node, precedence: i16) -> Node {
		
		while let TokenKind::Operator(op) = self.current_token.kind {
			if op.precedence() < precedence {
				break;
			}
			self.advance();

			let mut rhs = self.primary();
			while let TokenKind::Operator(lookahead) = self.current_token.kind {
				if lookahead.precedence() > op.precedence() {
					rhs = self.parse_expression(rhs, op.precedence() + 1);
				}
				else {
					break;
				}
			}

			lhs = Node::BinaryOp { 
				op: op,
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
			},
			TokenKind::If => {
				self.advance();

				let lhs = self.primary();
				let value = self.parse_expression(lhs, 0);

				self.eat(TokenKind::LBracket);

				let body = self.list_instr();

				self.eat(TokenKind::RBracket);

				Node::IfStatement { 
					condition: Box::new(value), 
					body: Box::new(body)
				}
			}
			TokenKind::Fn => {
				self.advance();

				let name = self.current_token.value.clone();
				self.eat(TokenKind::Identifier);
			
				self.eat(TokenKind::LParenthesis);

				let mut args : Vec<String> = Vec::new();
				if self.current_token.kind == TokenKind::Identifier {
					args.push(self.current_token.value.clone());

					self.advance();

					while self.current_token.kind == TokenKind::Coma {
						self.advance();

						args.push(self.current_token.value.clone());

						self.eat(TokenKind::Identifier);
					}
				}

				self.eat(TokenKind::RParenthesis);
				self.eat(TokenKind::LBracket);

				let body = self.list_instr();

				self.eat(TokenKind::RBracket);

				Node::FunctionDeclaration { 
					name: name, 
					args: args, 
					body: Box::new(body),
				}
			}
			_ => { panic!("instr : no valid token kind {:?}", self.current_token); }
		}
	}

	fn list_instr(&mut self) -> Option<Node> {
		if self.expect(TokenKind::Eof) || self.expect(TokenKind::RBracket) {
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
	fn if_statement_parsing(){
		let mut lexer = Lexer::new("if condition == 2 { let test = 3; }");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::IfStatement { 
					condition: Box::new(Node::BinaryOp { 
						op: Operator::Equal, 
						left: Box::new(
							Node::Identifier(String::from("condition"))
						), 
						right: Box::new(
							Node::Int(2)
						) 
					}),
					body: Box::new(Some(Node::InstructionList { 
						current: Box::new(
							Node::VarDeclaration { 
								name: String::from("test"),
								value: Box::new(Node::Int(3))
							}
						), 
						next: Box::new(None) 
					})) 
				}),
				next: Box::new(None)
			}
		));
	}

	#[test]
	fn function_declaration_parsing(){
		let mut lexer = Lexer::new("fn foo(arg1, arg2, arg3) { }");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::FunctionDeclaration { 
					name: String::from("foo"), 
					args: Vec::from([String::from("arg1"), String::from("arg2"), String::from("arg3")]), 
					body: Box::new(None) 
				}),
				next: Box::new(None)
			}
		));
	}

	#[test]	
	fn math_exp_parsing(){
		let mut lexer = Lexer::new("let math = -1 * 3 + 4 * 2;");

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
									left: Box::new(Node::UnaryOp {
										op: Operator::Minus,
										right: Box::new(Node::Int(1)) 
									}), 
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