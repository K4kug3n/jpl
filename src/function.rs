use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Function {
	pub params: Vec<String>,
	pub body: Option<Node>
}