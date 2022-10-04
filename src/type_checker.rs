use crate::node::Node;
use crate::operator::Operator;
use crate::visitor::{Visitor, Visitable};
use crate::expression_result::ExpressionResult;
use crate::scope::Scope;

pub struct TypeCheckerVisitor {
    result: ExpressionResult,
    scopes: Vec<Scope>
}

impl TypeCheckerVisitor {
    pub fn new() -> TypeCheckerVisitor {
        // TODO: Replace by void
        TypeCheckerVisitor { 
            result: ExpressionResult::Float(0.0),
            scopes: Vec::new(),
        }
    }

    pub fn check(&mut self, ast: &Node) {
        ast.accept(self);
    }

    // TODO: Code duplication
    fn insert_var(&mut self, name: &String, value: ExpressionResult) {
        let current = self.scopes.last_mut();

		if let Some(scope) = current {
			scope.memory.insert(String::from(name), value);
		}
		else {
			panic!("No scope");
		}
    }

    // TODO: Code duplication
    fn resolve_scope_var(&mut self, name: &str) -> Option<&mut ExpressionResult> {
		for scope in self.scopes.iter_mut().rev() {
			let result = scope.memory.get_mut(name);

			if let Some(_) = result {
				return result;
			}
		}

		None
	}
}

impl Visitor for TypeCheckerVisitor {
    fn visit_int(&mut self, value: i64) {
        self.result = ExpressionResult::Int(value);
    }

    fn visit_float(&mut self, value: f64) {
        self.result = ExpressionResult::Float(value);
    }

    fn visit_identifier(&mut self, name: &String) {
        
    }

    fn visit_bool(&mut self, value: bool) {

    }

    fn visit_binary_op(&mut self, _: &Operator, left: &Node, right: &Node) {

    }

	fn visit_unary_op(&mut self, op: &Operator, right: &Node) {

    }

	fn visit_var_assignation(&mut self, name: &String, value: &Node) {
        // TODO: Check type here
        value.accept(self);
		let result = self.result;

		match self.resolve_scope_var(name) {
			Some(var) => {
				*var = result;
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

        // TODO: Add void
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

	fn visit_function_declaration(&mut self, name: &String, args: &Vec<String>, body: &Option<Node>) {
        // TODO: Add parameters / return type
    }
	
    fn visit_function_call(&mut self, name: &String, args: &Vec<Node>) {
        // TODO: Check arguments / return type

        
    }
}