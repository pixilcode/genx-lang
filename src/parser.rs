use crate::ast::Program;
use crate::ast::Decl;

pub fn parse(code: &str) -> Result<Program, peg::error::ParseError<peg::str::LineCol>> {
	program_parser::program(code)
}

peg::parser! {
	grammar program_parser() for str {

		/// The rule for parsing a program
		pub rule program() -> Program
			= decls:decl()* { Program::new(
				decls.into_iter()
					.flatten() // Get rid of all of the `None`s
					.collect()
			)}
		
		/// The rule for parsing a declaration
		/// Returns `None` if it parses a comment
		/// or a newline
		rule decl() -> Option<Decl>
			= _:("#" [^ '\n']*)? "\n" { None }
			/ "pat" _ id:ident() _ "=" __ pat:pattern() ___ { None }
		
		/// The rule for parsing a pattern
		rule pattern() -> ()
			= "\"Hello, world!\""
		
		/// The rule for parsing an identifier
		rule ident() -> String
			= id:$(
				(['a'..='z'] / ['A'..='Z']) // Starts with a alpha
				(['a'..='z'] / ['A'..='Z'] / ['0'..='9'])* // Can contain alphanumeric
			) { id.to_string() }

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