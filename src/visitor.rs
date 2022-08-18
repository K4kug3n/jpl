use crate::parser::{NodeExpression, Operator};

pub trait Visitor {
	fn visit_int(&mut self, value: i64);
	fn visit_float(&mut self, value: f64);
	fn visit_binary_op(&mut self, op: &Operator, left: &NodeExpression, right: &NodeExpression);
}

pub trait Visitable {
	fn accept(&self, interpretor: &mut dyn Visitor);
}