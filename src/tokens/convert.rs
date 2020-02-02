use regex::Regex;
use anyhow::Result;

#[derive(Debug)]
pub struct Convert {
	from: Regex,
	to: String,
}

impl Convert {
	pub fn new(r:&str, to:String) -> Result<Self> {
		let regex = Regex::new(r)?;
		Ok(Convert {
			regex,
			to,
		})
	}

	pub fn convert(&self, line:impl AsRef<String>) -> String {
		self.regex.replace_all(line, self.to)
	}
}
