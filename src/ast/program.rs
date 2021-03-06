use crate::ast::Decl;

/// An collection of declarations that compose
/// a program.
#[derive(Debug)]
pub struct Program {
	decls: Vec<Decl>
}

impl Program {

	/// Create a new program from given declarations
	pub fn new(decls: Vec<Decl>) -> Self {
		Self {
			decls
		}
	}

	/// Given a transformation function, fold the declarations
	/// to produce a singular structure
	pub fn fold_decls<B, F>(&self, init: B, f: F) -> B
		where F: FnMut(B, &Decl) -> B {
		self.decls.iter().fold(init, f)
	}
}