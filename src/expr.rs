/// Represents each kind of expression that can be evaluated
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
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
