use crate::ast::{Program, Decl, Expr, Ident};

pub fn parse(code: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
	program_parser::program(code)
}

peg::parser! {
	grammar program_parser() for str {

		//  PROGRAM
		//  =======

		/// The rule for parsing a program
		pub rule program() -> Program
			= decls:decl()* { Program::new(
				decls.into_iter()
					.flatten() // Get rid of all of the `None`s
					.collect()
			)}
		
		
		//  DECLARATIONS
		//  ============

		/// The rule for parsing a declaration
		/// Returns `None` if it parses a comment
		/// or a newline
		rule decl() -> Option<Decl>
			= _comment:("#" [^ '\n']*)? "\n" { None }
			/ "pat" _ "$" id:ident() _ "=" __ pat:expr() ___ { Some(Decl::ExprPattern(id, pat)) }
		
		
		//  PATTERNS
		//  ========

		/// The rule for parsing a pattern
		rule expr() -> Expr
			= s:string() { Expr::string(&s) }
			
			
		//  TOKENS
		//  ======

		/// The rule for parsing a string
		rule string() -> String
			= "\"" s:$([^ '"']*) "\"" { s.into() }
		
		/// The rule for parsing an identifier
		rule ident() -> Ident
			= id:$(
				(['a'..='z'] / ['A'..='Z']) // Starts with a alpha
				(['a'..='z'] / ['A'..='Z'] / ['0'..='9'])* // Can contain alphanumeric
			) { id.to_string() }
		
		
		//  WHITESPACE
		//  ==========

		/// The rule for whitespace (newlines not allowed)
		rule _() -> Vec<()>
			= (" " / "\t")*
		
		/// The rule for whitespace (newlines optional)
		rule __() -> Vec<()>
			= (" " / "\t" / "\r"? "\n")*
		
		/// The rule for whitespace (newlines required)
		rule ___() -> Vec<()>
			= ((" " / "\t")* "\r"? "\n")+
	}
}