use std::fmt;

use crate::parser::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// Any integer
    Int(i32),
    /// The "+" symbol
    Plus,
    /// The "-" symbol
    Minus,
    /// The "(" symbol
    LeftParen,
    /// The ")" symbol
    RightParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Token::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    /// Creates a new token stream from the given set of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {tokens}
    }

    /// Returns the next token or an error if none are left
    pub fn next(&mut self) -> Result<Token, ParseError> {
        if self.tokens.is_empty() {
            Err(ParseError::UnexpectedEndOfInput)
        } else {
            Ok(self.tokens.remove(0))
        }
    }
}
