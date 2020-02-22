use std::fmt;
use std::error::Error;

#[derive(Debug)]
enum ParseError {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
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
struct TokenStream {
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

/// Represents each kind of expression that can be evaluated
#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    /// An integer
    Int(i32),
    /// The addition of two expressions
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// The subtraction of two expressions
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

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

fn main() -> Result<(), Box<dyn Error>> {
    use Token::*;
    let tokens = vec![Int(32), Plus, LeftParen, Int(-24), Minus, Int(10), RightParen];
    let mut token_stream = TokenStream::new(tokens);

    let expr = Expr::parse(&mut token_stream)?;
    println!("{:#?}", expr);

    Ok(())
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
