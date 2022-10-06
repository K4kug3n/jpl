use core::panic;

use crate::operator::{Operator};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::node::{Node};
use crate::r#type::Type;

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

	fn parse_args(&mut self) -> Vec<Node> {
		let mut args : Vec<Node> = Vec::new();
		if self.current_token.kind != TokenKind::RParenthesis {
			args.push(self.parse_expression());

			while self.current_token.kind == TokenKind::Coma {
				self.advance();

				args.push(self.parse_expression());
			}
		}

		args
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

				if self.current_token.kind != TokenKind::LParenthesis {
					return Node::Identifier(name);
				}

				self.advance();

				let args = self.parse_args();

				self.eat(TokenKind::RParenthesis);

				Node::FunctionCall { 
					name: name,
					args: args,
				}
			},
			TokenKind::LParenthesis => {
				self.advance();

				let exp = self.parse_expression();

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

	fn parse_expression(&mut self) -> Node {
		let lhs = self.primary();
		
		self.expression(lhs, 0)
	}

	fn expression(&mut self, mut lhs: Node, precedence: i16) -> Node {
		
		while let TokenKind::Operator(op) = self.current_token.kind {
			if op.precedence() < precedence {
				break;
			}
			self.advance();

			let mut rhs = self.primary();
			while let TokenKind::Operator(lookahead) = self.current_token.kind {
				if lookahead.precedence() > op.precedence() {
					rhs = self.expression(rhs, op.precedence() + 1);
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

				let mut declared_type = None;
				if self.expect(TokenKind::Colon) {
					self.advance();

					match self.current_token.kind {
						TokenKind::Type(explicit_type) => {
							declared_type =	Some(explicit_type);
						},
						_ => panic!("Not a valid type") // TODO: Better error handling
					}

					self.advance();
				}

				self.eat(TokenKind::Assign);

				let value = self.parse_expression();

				self.eat(TokenKind::Semilicon);

				Node::VarDeclaration { 
					name: name,
					declared_type: declared_type,
					value: Box::new(value)
				}
			},
			TokenKind::Identifier => {
				let name = self.current_token.value.clone();
				self.advance();

				match self.current_token.kind {
					TokenKind::Assign => {
						self.advance();

						let value = self.parse_expression();

						self.eat(TokenKind::Semilicon);

						Node::VarAssignation { 
							name: name,
							value: Box::new(value)
						}
					},
					TokenKind::LParenthesis => {
						self.advance();

						let args = self.parse_args();

						self.eat(TokenKind::RParenthesis);
						self.eat(TokenKind::Semilicon);

						Node::FunctionCall { 
							name: name,
							args: args,
						}
					},
					_ => panic!("Wront kind after Identifier")
				}				
			},
			TokenKind::Return => {
				self.advance();

				match self.current_token.kind {
					TokenKind::Semilicon => {
						self.advance();

						Node::ReturnStatement { value: Box::new(None) }
					},
					_ => {
						let value = self.parse_expression();

						self.eat(TokenKind::Semilicon);

						Node::ReturnStatement { value: Box::new(Some(value)) }
					},
				}
			},
			TokenKind::If => {
				self.advance();

				let value = self.parse_expression();

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

				let mut param_names : Vec<String> = Vec::new();
				let mut param_types : Vec<Type> = Vec::new();
				if self.current_token.kind == TokenKind::Identifier {
					param_names.push(self.current_token.value.clone());
					self.advance();

					self.eat(TokenKind::Colon);

					match self.current_token.kind {
						TokenKind::Type(declared_type) => {
							param_types.push(declared_type);
							self.advance();
						}
						_ => panic!("No type declared") // TODO: Better error handling
					}

					while self.current_token.kind == TokenKind::Coma {
						self.advance();

						param_names.push(self.current_token.value.clone());
						self.eat(TokenKind::Identifier);

						self.eat(TokenKind::Colon);

						match self.current_token.kind {
							TokenKind::Type(declared_type) => {
								param_types.push(declared_type);
								self.advance();
							}
							_ => panic!("No type declared") // TODO: Better error handling
						}
					}
				}

				self.eat(TokenKind::RParenthesis);

				let mut return_type = Type::Void;
				if self.expect(TokenKind::Arrow) {
					self.advance();

					match self.current_token.kind {
						TokenKind::Type(declared_type) => {
							return_type = declared_type;
							self.advance();
						}
						_ => panic!("No type declared") // TODO: Better error handling
					}
				}

				self.eat(TokenKind::LBracket);

				let body = self.list_instr();

				self.eat(TokenKind::RBracket);

				Node::FunctionDeclaration { 
					name: name, 
					param_names: param_names, 
					param_types: param_types,
					return_type: return_type,
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

		//TODO: May not be managed by parser (see CFG)
		if self.expect(TokenKind::Return) {
			let result = Node::InstructionList { 
				current: Box::new(self.instr()),
				next: Box::new(None) 
			};

			while !(self.expect(TokenKind::Eof) || self.expect(TokenKind::RBracket)){
				self.advance();
			}

			return Some(result);
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
					declared_type: None,
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
								declared_type: None,
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
		let mut lexer = Lexer::new("fn foo(arg1: int, arg2: int, arg3: int) -> int { return 2; let test = 2; }");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::FunctionDeclaration { 
					name: String::from("foo"), 
					param_names: Vec::from([String::from("arg1"), String::from("arg2"), String::from("arg3")]),
					param_types: Vec::from([Type::Int, Type::Int, Type::Int]),
					return_type: Type::Int,
					body: Box::new(Some(Node::InstructionList { 
						current: Box::new(Node::ReturnStatement { 
							value: Box::new(Some(Node::Int(2))) 
						}),
						next: Box::new(None) 
					}))
				}),
				next: Box::new(None)
			}
		));
	}

	#[test]
	fn function_call_parsing(){
		let mut lexer = Lexer::new("foo(arg1, arg2 + 2, arg3);");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::FunctionCall { 
					name: String::from("foo"),
					args: Vec::from([
						Node::Identifier(String::from("arg1")),
						Node::BinaryOp { 
							op: Operator::Add, 
							left: Box::new(Node::Identifier(String::from("arg2"))), 
							right: Box::new(Node::Int(2)) 
						},
						Node::Identifier(String::from("arg3")),
					])
				}),
				next: Box::new(None)
			}
		));
	}

	#[test]	
	fn math_exp_parsing(){
		let mut lexer = Lexer::new("let math: int = -1 * 3 + 4 * 2;");

		let mut parser = Parser::new(&mut lexer);

		let ast = parser.ast();

		assert_eq!(ast,Some(
			Node::InstructionList {
				current: Box::new(Node::VarDeclaration { 
					name: String::from("math"),
					declared_type: Some(Type::Int),
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