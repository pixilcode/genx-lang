use std::path::PathBuf;
use std::fs;
use std::io;

use structopt::StructOpt;

mod ast;
mod parser;
mod evaluator;

fn main() -> io::Result<()> {
    let config = Config::from_args();

    let code = &fs::read(config.code_file)?;
    let code = String::from_utf8_lossy(code);

    let ast = parser::parse(&code);

    match ast {
        Ok(ast) => {
            let result = evaluator::eval(ast);

            match result {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{}", err),
            }
        },
        Err(e) => { println!("{}", e); }
    }
    
    Ok(())
}

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(parse(from_os_str))]
    code_file: PathBuf,
}