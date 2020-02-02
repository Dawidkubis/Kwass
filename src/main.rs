mod token;
mod interpreter;
mod tokens;

use interpreter::Interpreter;

use anyhow::Result;
use structopt::StructOpt;

use std::fs::read_to_string;
use std::io::{self, Write};

#[derive(Debug, StructOpt)]
struct Opt {
    /// file to process
    file: Option<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if let None = opt.file {
        loop {
            print!("//> ");

            io::stdout().flush().unwrap();

            let mut input: String = String::new();
            io::stdin().read_line(&mut input)?;

			Interpreter::interpret(&input);
        }
    }

    //let input: Vec<String> = read_to_string(opt.file)?
    //.lines()
    //.map(String::from)
    //.collect();

    Ok(())
}
