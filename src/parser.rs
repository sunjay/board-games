use std::fmt;
use std::error::Error;

use crate::expr::Expr;
use crate::token::{Token, TokenStream};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfInput,
    UnexpectedToken {
        found: Token,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        match self {
            UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            UnexpectedToken {found} => write!(f, "Unexpected token: {}", found)
        }
    }
}

impl Error for ParseError {}

impl Expr {
    /// Parses a set of tokens into an expression
    pub fn parse(tokens: &mut TokenStream) -> Result<Expr, ParseError> {
        use Token::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Operator {
            Plus,
            Minus,
        }

        let left = match tokens.next()? {
            Int(value) => Expr::Int(value),
            LeftParen => {
                let inner_expr = Expr::parse(tokens)?;

                // Expect to get a right paren
                match tokens.next()? {
                    Token::RightParen => {},
                    found => return Err(ParseError::UnexpectedToken {found}),
                }

                inner_expr
            },
            found => return Err(ParseError::UnexpectedToken {found}),
        };

        let op = match tokens.next()? {
            Plus => Operator::Plus,
            Minus => Operator::Minus,
            found => return Err(ParseError::UnexpectedToken {found}),
        };

        let right = match tokens.next()? {
            Int(value) => Expr::Int(value),
            LeftParen => {
                let inner_expr = Expr::parse(tokens)?;

                // Expect to get a right paren
                match tokens.next()? {
                    Token::RightParen => {},
                    found => return Err(ParseError::UnexpectedToken {found}),
                }

                inner_expr
            },
            found => return Err(ParseError::UnexpectedToken {found}),
        };

        Ok(match op {
            Operator::Plus => Expr::Add {left: Box::new(left), right: Box::new(right)},
            Operator::Minus => Expr::Sub {left: Box::new(left), right: Box::new(right)},
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_expr() {
        unimplemented!()
    }

    #[test]
    fn unexpected_token_plus() {
        unimplemented!()
    }

    #[test]
    fn unexpected_token_minus() {
        unimplemented!()
    }

    #[test]
    fn unexpected_token_int() {
        unimplemented!()
    }

    #[test]
    fn unexpected_eoi() {
        unimplemented!()
    }

    #[test]
    fn unexpected_tokens_after_expr() {
        unimplemented!()
    }

    #[test]
    fn unclosed_parens() {
        unimplemented!()
    }

    #[test]
    fn too_many_right_parens() {
        unimplemented!()
    }
}
