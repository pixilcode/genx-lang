use crate::ast::Decl;

/// An collection of declarations that compose
/// a program.
#[derive(Debug)]
pub struct Program {
	decls: Vec<Decl>
}

impl Program {

	// Create a new program from given declarations
	pub fn new(decls: Vec<Decl>) -> Self {
		Self {
			decls
		}
	}
}