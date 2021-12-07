
/// An AST node representing expressions
/// 
/// The expressions are as follows:
///   * `String`
#[derive(Debug, Clone)]
pub enum Expr {
	String(String)
}

impl Expr {

	// Produce a string expression
	pub fn string(s: &str) -> Self {
		Self::String(s.into())
	}
}