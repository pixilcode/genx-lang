use crate::ast;
use std::collections::HashMap;
use std::rc::Rc;

type Env = HashMap<ast::Ident, Rc<ast::Expr>>;

pub fn eval(program: ast::Program) -> Result<Rc<ast::Expr>, EvalError> {
	let env = build_env(program);

	let main = get_main(&env)?;

	Ok(main)
}

fn build_env(program: ast::Program) -> Env {
	program.fold_decls(HashMap::new(), |mut env, decl| {
		match decl {
			ast::Decl::ExprPattern(id, expr) => env.insert(id.clone(), Rc::new(expr.clone())),
		};

		env
	})
}

fn get_main(env: &Env) -> Result<Rc<ast::Expr>, EvalError> {
	env.get("Main")
		.map(|expr| Rc::clone(expr))
		.ok_or(EvalError::NoMain)
}

#[derive(Debug, Clone, Copy)]
pub enum EvalError {
	NoMain,
}