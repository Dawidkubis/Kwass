use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub enum Token {
    #[end]
    End,

    #[error]
    Error,

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
}
