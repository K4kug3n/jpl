use std::collections::HashMap;

use crate::operator::{Operator};
use crate::parser::{Node};
use crate::visitor::{Visitor, Visitable};

#[derive(Clone, Copy, Debug)]
enum ExpressionResult {
	Int(i64),
	Float(f64),
	Bool(bool)
}

pub struct InterpretorVisitor { 
	result: ExpressionResult,
	memory: HashMap<String, ExpressionResult>
}

impl InterpretorVisitor {
	pub fn new() -> InterpretorVisitor {
		// TODO: Fixme this awfull placeholder
		InterpretorVisitor {
			result: ExpressionResult::Float(0.0),
			memory: HashMap::new()
		}
	}

	pub fn interpret(&mut self, ast : Node) {
		ast.accept(self);

		println!("{:?}", self.memory);
	}

	fn apply_op_float(&mut self, op: &Operator, lhs: f64, rhs: f64) {
		match op {
			Operator::Add => { 
				self.result = ExpressionResult::Float(lhs + rhs);
			},
			Operator::Minus => {
				self.result = ExpressionResult::Float(lhs - rhs);
			},
			Operator::Product => { 
				self.result = ExpressionResult::Float(lhs * rhs);
			},
			Operator::Divide => {
				self.result = ExpressionResult::Float(lhs / rhs);
			},
			Operator::LowerOrEq => {
				self.result = ExpressionResult::Bool(lhs <= rhs);
			},
			Operator::GreaterOrEq => {
				self.result = ExpressionResult::Bool(lhs >= rhs);
			},
			Operator::Equal => {
				self.result = ExpressionResult::Bool(lhs == rhs);
			},
			Operator::NotEqual => {
				self.result = ExpressionResult::Bool(lhs != rhs);
			},
			Operator::Lower => {
				self.result = ExpressionResult::Bool(lhs < rhs);
			},
			Operator::Greater => {
				self.result = ExpressionResult::Bool(lhs > rhs);
			},
			_ => panic!("Wrong type")
		}
	}

	fn apply_op_int(&mut self, op: &Operator, lhs: i64, rhs: i64) {
		match op {
			Operator::Add => { 
				self.result = ExpressionResult::Int(lhs + rhs);
			},
			Operator::Minus => {
				self.result = ExpressionResult::Int(lhs - rhs);
			},
			Operator::Product => { 
				self.result = ExpressionResult::Int(lhs * rhs);
			},
			Operator::Divide => {
				self.result = ExpressionResult::Int(lhs / rhs);
			},
			Operator::LowerOrEq => {
				self.result = ExpressionResult::Bool(lhs <= rhs);
			},
			Operator::GreaterOrEq => {
				self.result = ExpressionResult::Bool(lhs >= rhs);
			},
			Operator::Equal => {
				self.result = ExpressionResult::Bool(lhs == rhs);
			},
			Operator::NotEqual => {
				self.result = ExpressionResult::Bool(lhs != rhs);
			},
			Operator::Lower => {
				self.result = ExpressionResult::Bool(lhs < rhs);
			},
			Operator::Greater => {
				self.result = ExpressionResult::Bool(lhs > rhs);
			},
			_ => panic!("Wrong type")
		}
	}

	fn apply_op_bool(&mut self, op: &Operator, lhs: bool, rhs: bool) {
		match op {
			Operator::LogicalAnd => { 
				self.result = ExpressionResult::Bool(lhs && rhs);
			},
			Operator::LogicalOr => {
				self.result = ExpressionResult::Bool(lhs || rhs);
			},
			Operator::Equal => { 
				self.result = ExpressionResult::Bool(lhs == rhs);
			},
			Operator::NotEqual => {
				self.result = ExpressionResult::Bool(lhs != rhs);
			},
			_ => panic!("Wrong type")
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
					self.apply_op_int(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
			},
			ExpressionResult::Float(lhs) => {
				if let ExpressionResult::Float(rhs) = right_result {
					self.apply_op_float(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
			},
			ExpressionResult::Bool(lhs) => {
				if let ExpressionResult::Bool(rhs) = right_result {
					self.apply_op_bool(op, lhs, rhs);
				}
				else {
					panic!("Wrong type") // TODO: Type checking
				}
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

	fn visit_instruction_list(&mut self, current: &Node, next: &Option<Node>) {
		current.accept(self);

		if let Some(x) = next {
			x.accept(self);
		}
	}
}