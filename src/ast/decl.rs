use crate::ast::{Expr, Ident};

/// An AST node representing program declarations
/// 
/// The types are as follows:
///   * `ExprPattern`: a pattern that evaluates to an
///     expression
#[derive(Debug, Clone)]
pub enum Decl {
	ExprPattern(Ident, Expr)
}