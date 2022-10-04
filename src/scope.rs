use std::collections::HashMap;

use crate::expression_result::ExpressionResult;
use crate::function::Function;

#[derive(Clone)]
pub struct Scope {
	pub memory: HashMap<String, ExpressionResult>,
	pub functions: HashMap<String, Function>
}

impl Scope {
	pub fn new() -> Scope {
		Scope { 
			memory: HashMap::new(),
			functions: HashMap::new(), // TODO: Allow nested function, may not keep it
		}
	}
}