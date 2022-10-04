#[derive(Clone, Copy, Debug)]
pub enum ExpressionResult {
	Int(i64),
	Float(f64),
	Bool(bool)
}