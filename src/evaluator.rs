use crate::ast;
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;

type Env = HashMap<ast::Ident, Rc<ast::Expr>>;

pub fn eval(program: ast::Program) -> Result<Value, EvalError> {
	let env = build_env(program);

	let main = get_main(&env)?;

	let result = eval_expr(&main, &env)?;

	Ok(result)
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

fn eval_expr(expr: &Rc<ast::Expr>, _env: &Env) -> Result<Value, EvalError> {
	match expr.borrow() {
		ast::Expr::String(s) => Ok(Value::string(s)),
	}
}

/// Values used by the evaluator to evaluate
/// the AST
#[derive(Debug, Clone)]
pub enum Value {
	String(String),
}

impl Value {

	/// Create a string value
	fn string(s: &str) -> Self {
		Self::String(s.into())
	}
}



/// An evaluation error
/// 
/// This includes:
///   * `NoMain`: no main pattern was found
#[derive(Debug, Clone, Copy)]
pub enum EvalError {
	NoMain,
}