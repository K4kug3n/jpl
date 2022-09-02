#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
	Add,
	Minus,
	Product,
	Divide,
	LogicalAnd,
	LogicalOr,
	Equal,
	NotEqual,
	LowerOrEq,
	GreaterOrEq,
	Lower,
	Greater,
	Not
}

impl Operator {
	pub fn precedence(&self) -> i16 {
		match self {
			Operator::LogicalAnd => 0,
			Operator::LogicalOr => 0,
			Operator::LowerOrEq => 1,
			Operator::GreaterOrEq => 1,
			Operator::Lower => 1,
			Operator::Greater => 1,
			Operator::Equal => 1,
			Operator::NotEqual => 1,
			Operator::Add => 2,
			Operator::Minus => 2,
			Operator::Product => 3,
			Operator::Divide => 3,
			Operator::Not => 4
		}
	}
}