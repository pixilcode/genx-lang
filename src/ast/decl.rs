use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Decl {
	ExprPattern(Expr)
}