use crate::token::Token;

use logos::Logos;

#[derive(Debug)]
pub struct Interpreter;

impl Interpreter {
	pub fn interpret(line: &str) {
		let mut lexer = Token::lexer(line);
		
		loop {

			let tokn = &lexer.token;
			if *tokn == Token::End {break}

			dbg!(tokn);

			lexer.advance();
		}
	}
}
