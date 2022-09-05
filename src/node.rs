use crate::operator::{Operator};
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
		params: Vec<String>,
		body: Box<Option<Node>>
	},
	FunctionCall {
		name: String,
		args: Vec<Node>
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
			Node::FunctionDeclaration { name, params: args, body } => visitor.visit_function_declaration(name, args, body),
			Node::FunctionCall { name, args } => visitor.visit_function_call(name, args),
        }
    }
}