use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[end]
    End,

    #[error]
    Error,

	#[token = "#"]
	Comment,

	#[token = "="]
	Assigment,

    #[regex = "\".*\""]
    Str,

    #[regex = r"[a-zA-z_]+"]
    Text,

    #[token = ";"]
    Semicolon,

    #[token = "+"]
    Plus,

    #[token = "-"]
    Minus,

    #[token = "*"]
    Times,

    #[token = "/"]
    Divide,

    #[token = "**"]
    Exponentiate,

    #[regex = r"\d+\.\d*"]
    Float,

    #[regex = r"\d+"]
    Integer,
}
