mod types;

use types::ast::*;
use types::error::Error;
use types::re::*;

pub fn get_regex(pattern: &str) -> Result<RegEx, Error> {
    let ast = AstNode::from_regex(pattern)?;

    todo!(
        "Generated ast: {:#?}\
        The rest is a work in progress",
        ast
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn literal() {
        assert_eq!(
            AstNode::from_regex("aab").unwrap(),
            AstNode::literal('a') + (AstNode::literal('a') + AstNode::literal('b'))
        );
    }

    #[test]
    fn bracket_expression_inclusive() {
        assert_eq!(
            AstNode::from_regex("a[bc]").unwrap(),
            AstNode::literal('a')
                + AstNode::character_pattern_inclusive(HashSet::from_iter("bc".chars()))
        );
    }

    #[test]
    fn bracket_expression_exclusive() {
        assert_eq!(
            AstNode::from_regex("a[^bc]").unwrap(),
            AstNode::literal('a')
                + AstNode::character_pattern_exclusive(HashSet::from_iter("bc".chars()))
        );
    }
}
