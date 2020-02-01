mod tokens;

use tokens::Interpreter;

use structopt::StructOpt;
use anyhow::Result;

use std::fs::read_to_string;

#[derive(Debug, StructOpt)]
struct Opt {
	/// file to process
	file: String,
}

fn main() -> Result<()>{
	let opt = Opt::from_args();

	let input: Vec<String> = read_to_string(opt.file)?
		.lines()
		.map(String::from)
		.collect();

	for i in input {
		if let Some(s) = Interpreter::interpret(i) {
			print!("{}", s);
		}
	}

	Ok(())
}
