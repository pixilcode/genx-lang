use std::path::PathBuf;
use std::fs;
use std::io;

use structopt::StructOpt;

mod ast;
mod parser;

fn main() -> io::Result<()> {
    let config = Config::from_args();

    let code = &fs::read(config.code_file)?;
    let code = String::from_utf8_lossy(code);

    println!("{:?}", parser::parse(&code));
    Ok(())
}

#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(parse(from_os_str))]
    code_file: PathBuf,
}