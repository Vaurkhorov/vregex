use crate::AstNode;

#[derive(Debug)]
pub enum Error {
    /// Bracket opened at the returned index was unmatched.
    UnmatchedBracket(usize),
    /// The given pattern ended with a syntax error.
    UnexpectedEof(usize),

    /// A `|` was encountered, refactor the returned node if possible
    OrRefactor(AstNode),
}
