use crate::parser::{Node, Operator};

pub trait Visitor {
	fn visit_int(&mut self, value: i64);
	fn visit_float(&mut self, value: f64);
	fn visit_binary_op(&mut self, op: &Operator, left: &Node, right: &Node);
}

pub trait Visitable {
	fn accept(&self, visitor: &mut dyn Visitor);
}