use core::panic;
use std::collections::HashMap;

use crate::operator::{Operator};
use crate::node::{Node};
use crate::visitor::{Visitor, Visitable};

#[derive(Clone, Copy, Debug)]
enum ExpressionResult {
	Int(i64),
	Float(f64),
	Bool(bool)
}

#[derive(Debug)]
struct Function {
	params: Vec<String>,
	body: Option<Node>
}

pub struct InterpretorVisitor { 
	result: ExpressionResult,
	memory: HashMap<String, ExpressionResult>,
	functions: HashMap<String, Function>,
}

impl InterpretorVisitor {
	pub fn new() -> InterpretorVisitor {
		// TODO: Fixme this awfull placeholder
		InterpretorVisitor {
			result: ExpressionResult::Float(0.0),
			memory: HashMap::new(),
			functions: HashMap::new(),
		}
	}

	pub fn interpret(&mut self, ast : Node) {
		ast.accept(self);

		println!("{:?}", self.memory);
		println!("{:?}", self.functions);
	}

	fn apply_binary_op_float(op: &Operator, lhs: f64, rhs: f64) -> ExpressionResult {
		match op {
			Operator::Add => ExpressionResult::Float(lhs + rhs),
			Operator::Minus => ExpressionResult::Float(lhs - rhs),
			Operator::Product =>  ExpressionResult::Float(lhs * rhs),
			Operator::Divide => ExpressionResult::Float(lhs / rhs),
			Operator::LowerOrEq => ExpressionResult::Bool(lhs <= rhs),
			Operator::GreaterOrEq => ExpressionResult::Bool(lhs >= rhs),
			Operator::Equal => ExpressionResult::Bool(lhs == rhs),
			Operator::NotEqual => ExpressionResult::Bool(lhs != rhs),
			Operator::Lower => ExpressionResult::Bool(lhs < rhs),
			Operator::Greater => ExpressionResult::Bool(lhs > rhs),
			_ => panic!("Wrong op")
		}
	}

	fn apply_binary_op_int(op: &Operator, lhs: i64, rhs: i64) -> ExpressionResult {
		match op {
			Operator::Add => ExpressionResult::Int(lhs + rhs),
			Operator::Minus => ExpressionResult::Int(lhs - rhs),
			Operator::Product => ExpressionResult::Int(lhs * rhs),
			Operator::Divide => ExpressionResult::Int(lhs / rhs),
			Operator::LowerOrEq => ExpressionResult::Bool(lhs <= rhs),
			Operator::GreaterOrEq => ExpressionResult::Bool(lhs >= rhs),
			Operator::Equal => ExpressionResult::Bool(lhs == rhs),
			Operator::NotEqual => ExpressionResult::Bool(lhs != rhs),
			Operator::Lower => ExpressionResult::Bool(lhs < rhs),
			Operator::Greater => ExpressionResult::Bool(lhs > rhs),
			_ => panic!("Wrong op")
		}
	}

	fn apply_binary_op_bool(op: &Operator, lhs: bool, rhs: bool) -> ExpressionResult {
		match op {
			Operator::LogicalAnd => ExpressionResult::Bool(lhs && rhs),
			Operator::LogicalOr => ExpressionResult::Bool(lhs || rhs),
			Operator::Equal => ExpressionResult::Bool(lhs == rhs),
			Operator::NotEqual => ExpressionResult::Bool(lhs != rhs),
			_ => panic!("Wrong op")
		}
	}

	fn apply_unary_op_bool(op: &Operator, rhs: bool) -> ExpressionResult{
		match op {
			Operator::Not => ExpressionResult::Bool(!rhs),
			_ => panic!("No valid opertaor for bool")
		}
	}

	fn apply_unary_op_int(op: &Operator, rhs: i64) -> ExpressionResult {
		match op {
			Operator::Minus => ExpressionResult::Int(-rhs),
			_ => panic!("No valid opertaor for int")
		}
	}

	fn apply_unary_op_float(op: &Operator, rhs: f64) -> ExpressionResult {
		match op {
			Operator::Minus => ExpressionResult::Float(-rhs),
			_ => panic!("No valid opertaor for float")
		}
	}
}

impl Visitor for InterpretorVisitor {
	fn visit_int(&mut self, value: i64) {
		self.result = ExpressionResult::Int(value);
	}

	fn visit_float(&mut self, value: f64) {
		self.result = ExpressionResult::Float(value);
	}

	fn visit_bool(&mut self, value: bool) {
		self.result = ExpressionResult::Bool(value);
	}

	fn visit_identifier(&mut self, name: &String) {
		let result = self.memory.get(name);
		match result {
			Some(x) => { self.result = *x },
			None => panic!("Identifier not declared")
		}
	}

	fn visit_binary_op(&mut self, op: &Operator, left: &Node, right: &Node) {
		left.accept(self);
		let left_result = self.result;

		right.accept(self);
		let right_result = self.result;
		
		match left_result {
			ExpressionResult::Int(lhs) => {
				if let ExpressionResult::Int(rhs) = right_result {
					self.result = Self::apply_binary_op_int(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
			},
			ExpressionResult::Float(lhs) => {
				if let ExpressionResult::Float(rhs) = right_result {
					self.result = Self::apply_binary_op_float(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
			},
			ExpressionResult::Bool(lhs) => {
				if let ExpressionResult::Bool(rhs) = right_result {
					self.result = Self::apply_binary_op_bool(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
			}
		}
	}

	fn visit_unary_op(&mut self, op: &Operator, right: &Node) {
		right.accept(self);

		match self.result {
			ExpressionResult::Bool(rhs) => {
				self.result = Self::apply_unary_op_bool(op, rhs);
			},
			ExpressionResult::Int(rhs) => {
				self.result = Self::apply_unary_op_int(op, rhs);
			},
			ExpressionResult::Float(rhs) => {
				self.result = Self::apply_unary_op_float(op, rhs);
			}
		}
	}

	fn visit_var_declaration(&mut self, name: &String, value: &Node) {
		value.accept(self);

		self.memory.insert(name.clone(), self.result);
	}

	fn visit_var_assignation(&mut self, name: &String, value: &Node) {
		value.accept(self);

		self.memory.entry(name.clone()).and_modify(|e| { *e = self.result });
	}

	fn visit_if_statement(&mut self, condition: &Node, body: &Option<Node>) {
		if let Some(instruction_list) = body {
			condition.accept(self);
			if let ExpressionResult::Bool(result) = self.result {
				if result {
					instruction_list.accept(self);
				}
			}
			else {
				panic!("Except bool value as condition");
			}
		}
	}

	fn visit_instruction_list(&mut self, current: &Node, next: &Option<Node>) {
		current.accept(self);

		if let Some(x) = next {
			x.accept(self);
		}
	}

	fn visit_function_declaration(&mut self, name: &String, args: &Vec<String>, body: &Option<Node>) {
		self.functions.insert(name.clone(), Function {
			params: args.clone(),
			body: body.clone(),
		});
	}

	fn visit_function_call(&mut self, name: &String, args: &Vec<Node>) {
		let result = self.functions.get(name);
		match result {
			Some(func) => {
				if func.params.len() != args.len() {
					panic!("Wrong number of args to call {}", name);
				}

				match func.body.clone() {
					Some(body) => {
						let func_params = func.params.clone();
						for (i, arg) in args.iter().enumerate() {
							arg.accept(self);

							self.memory.insert(func_params[i].clone(), self.result.clone());
						}

						body.accept(self)
					},
					None => {}
				}
			},
			None => panic!("Function not declared")
		}
	}
}