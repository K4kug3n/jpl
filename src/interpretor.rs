use crate::parser::{Node, Operator};
use crate::visitor::{Visitor, Visitable};

#[derive(Clone, Copy, Debug)]
enum ExpressionResult {
	Int(i64),
	Float(f64)
}

pub struct InterpretorVisitor { 
	result: ExpressionResult,
}

impl InterpretorVisitor {
	pub fn new() -> InterpretorVisitor {
		// TODO: Fixme this awfull placeholder
		InterpretorVisitor {
			result: ExpressionResult::Float(0.0)
		}
	}

	pub fn interpret(&mut self, ast : Node) {
		ast.accept(self);

		println!("{:?}", self.result);
	}

	fn apply_op_float(op: &Operator, lhs: f64, rhs: f64) -> f64{
		match op {
			Operator::Add => lhs + rhs,
			Operator::Minus => lhs - rhs,
			Operator::Product => lhs * rhs,
			Operator::Divide => lhs / rhs
		}
	}

	fn apply_op_int(op: &Operator, lhs: i64, rhs: i64) -> i64{
		match op {
			Operator::Add => lhs + rhs,
			Operator::Minus => lhs - rhs,
			Operator::Product => lhs * rhs,
			Operator::Divide => lhs / rhs
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

	fn visit_binary_op(&mut self, op: &Operator, left: &Node, right: &Node) {
		left.accept(self);
		let left_result = self.result;

		right.accept(self);
		let right_result = self.result;
		
		match left_result {
			ExpressionResult::Int(lhs) => {
				match right_result {
					ExpressionResult::Int(rhs) => {
						self.result = ExpressionResult::Int(InterpretorVisitor::apply_op_int(op, lhs, rhs));
					},
					ExpressionResult::Float(rhs) => {
						self.result = ExpressionResult::Int(InterpretorVisitor::apply_op_int(op, lhs, rhs as i64));
					},
				}
			},
			ExpressionResult::Float(lhs) => {
				match right_result {
					ExpressionResult::Int(rhs) => {
						self.result = ExpressionResult::Float(InterpretorVisitor::apply_op_float(op, lhs, rhs as f64));
						
					},
					ExpressionResult::Float(rhs) => {
						self.result = ExpressionResult::Float(InterpretorVisitor::apply_op_float(op, lhs, rhs));
					}
				}
			}
		}
	}
}