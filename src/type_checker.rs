use core::panic;
use std::collections::HashMap;

use crate::node::Node;
use crate::operator::Operator;
use crate::visitor::{Visitor, Visitable};
use crate::r#type::Type;

// TODO: Duplication
struct TypeScope {
	pub variables: HashMap<String, Type>,
}

impl TypeScope {
	fn new() -> TypeScope {
		TypeScope { 
			variables: HashMap::new(),
		}
	}
}

pub struct TypeCheckerVisitor {
	result: Type,
	scopes: Vec<TypeScope>
}

impl TypeCheckerVisitor {
	pub fn new() -> TypeCheckerVisitor {
		TypeCheckerVisitor { 
			result: Type::Void,
			scopes: Vec::from([ TypeScope::new() ]),
		}
	}

	pub fn check(&mut self, ast: &Node) {
		ast.accept(self);
	}

	// TODO: Code duplication
	fn insert_var(&mut self, name: &String, value: Type) {
		let current = self.scopes.last_mut();

		if let Some(scope) = current {
			scope.variables.insert(String::from(name), value);
		}
		else {
			panic!("No scope");
		}
	}

	fn resolve_scope_var(&self, name: &str) -> Option<&Type> {
		for scope in self.scopes.iter().rev() {
			let result = scope.variables.get(name);

			if let Some(_) = result {
				return result;
			}
		}

		None
	}

	fn apply_op(op: &Operator, original_type: Type) -> Type {
		match op {
			Operator::Equal => Type::Bool,
			Operator::Greater => Type::Bool,
			Operator::GreaterOrEq => Type::Bool,
			Operator::LogicalAnd => Type::Bool,
			Operator::LogicalOr => Type::Bool,
			Operator::Lower => Type::Bool,
			Operator::LowerOrEq => Type::Bool,
			Operator::NotEqual => Type::Bool,
			Operator::Not => Type::Bool,
			_ => original_type
		}
	}
}

impl Visitor for TypeCheckerVisitor {
	fn visit_int(&mut self, _: i64) {
		self.result = Type::Int;
	}

	fn visit_float(&mut self, _: f64) {
		self.result = Type::Float;
	}

	fn visit_identifier(&mut self, name: &String) {
		match self.resolve_scope_var(name) {
			Some(var) => {
				self.result = *var;
			},
			None => panic!("Not declared identifier {}", name)
		}
	}

	fn visit_bool(&mut self, _: bool) {
		self.result = Type::Bool;
	}

	fn visit_binary_op(&mut self, op: &Operator, left: &Node, right: &Node) {
		left.accept(self);
		let rhs_type = self.result;

		right.accept(self);

		if self.result != rhs_type {
			// TODO: Better error display
			panic!("Wrong type operand");
		}

		self.result = TypeCheckerVisitor::apply_op(op, rhs_type);
	}

	fn visit_unary_op(&mut self, op: &Operator, right: &Node) {
		right.accept(self);

		if *op == Operator::Not && self.result != Type::Bool {
			// TODO: Better error display
			panic!("Can't use Operator::Not on other type than Type::Bool");
		}
		else if *op != Operator::Not && self.result == Type::Bool {
			// TODO: Better error display
			panic!("Type::Bool only support Operator::Not as unary op");
		}
	}

	fn visit_var_assignation(&mut self, name: &String, value: &Node) {
		value.accept(self);
		let result = &self.result;

		match self.resolve_scope_var(name) {
			Some(var) => {
				if var != result {
					// TODO: Better error display
					panic!("Wrong type assignation");
				}
			},
			None => panic!("Not declared identifier {}", name)
		}
	}

	fn visit_var_declaration(&mut self, name: &String, value: &Node) {
		value.accept(self);

		self.insert_var(name, self.result);
	}

	fn visit_return_statement(&mut self, value: &Option<Node>) {
		if let Some(exp) = value {
			exp.accept(self);
		}
		else {
			self.result = Type::Void;
		}
	}

	fn visit_if_statement(&mut self, _: &Node, body: &Option<Node>) {
		if let Some(instruction_list) = body {
			instruction_list.accept(self);
		}
	}

	fn visit_instruction_list(&mut self, current: &Node, next: &Option<Node>) {
		current.accept(self);

		if let Some(x) = next {
			x.accept(self);
		}
	}

	fn visit_function_declaration(&mut self, _: &String, _: &Vec<String>, _: &Option<Node>) {
		// TODO: Add parameters / return type
	}
	
	fn visit_function_call(&mut self, _: &String, _: &Vec<Node>) {
		// TODO: Check arguments / return type
	}
}