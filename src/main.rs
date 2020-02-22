mod token;
mod expr;
mod parser;

use std::error::Error;

use token::{Token, TokenStream};
use expr::Expr;

fn main() -> Result<(), Box<dyn Error>> {
    use Token::*;
    let tokens = vec![Int(32), Plus, LeftParen, Int(-24), Minus, Int(10), RightParen];
    let mut token_stream = TokenStream::new(tokens);

    let expr = Expr::parse(&mut token_stream)?;
    println!("{:#?}", expr);

    Ok(())
}
