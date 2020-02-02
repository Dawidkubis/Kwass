mod convert;
mod scope;

use convert::Convert;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
	scope: Scope,
	conversions: Vec<Convert>,
}

impl Interpreter {
	pub fn new(scope: Scope) -> Self {
		Self {
			scope,
			conversions: vec![],
		}
	}

	pub fn interpret(&mut self, line: String) -> Option<String> {
		//TODO
	}
}

