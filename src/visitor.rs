use crate::node::{Node};
use crate::operator::{Operator};

pub trait Visitor {
	fn visit_int(&mut self, value: i64);
	fn visit_float(&mut self, value: f64);
	fn visit_identifier(&mut self, name: &String);
	fn visit_bool(&mut self, value: bool);
	fn visit_binary_op(&mut self, op: &Operator, left: &Node, right: &Node);
	fn visit_unary_op(&mut self, op: &Operator, right: &Node);
	fn visit_var_assignation(&mut self, name: &String, value: &Node);
	fn visit_var_declaration(&mut self, name: &String, value: &Node);
	fn visit_if_statement(&mut self, condition: &Node, body: &Option<Node>);
	fn visit_instruction_list(&mut self, current: &Node, next: &Option<Node>);
	fn visit_function_declaration(&mut self, name: &String, args: &Vec<String>, body: &Option<Node>);
	fn visit_function_call(&mut self, name: &String, args: &Vec<Node>);
}

pub trait Visitable {
	fn accept(&self, visitor: &mut dyn Visitor);
}