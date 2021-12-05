use crate::ast::Decl;

#[derive(Debug)]
pub struct Program {
	decls: Vec<Decl>
}

impl Program {
	pub fn new(decls: Vec<Decl>) -> Self {
		Self {
			decls
		}
	}
}