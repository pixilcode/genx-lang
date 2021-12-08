
/// An AST node representing expressions
/// 
/// The expressions are as follows:
///   * `String`
#[derive(Debug, Clone)]
pub enum Expr {
	String(String),
	Int(i32),
}

impl Expr {

	// Produce a string expression
	pub fn string(s: &str) -> Self {
		Self::String(s.into())
	}

	// Produce an integer expression
	pub fn int(i: i32) -> Self {
		Self::Int(i)
	}
}