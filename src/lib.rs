mod types;

use types::ast::*;
use types::enfa::*;
use types::error::Error;
use types::re::*;

pub fn get_regex(pattern: &str) -> Result<RegEx, Error> {
    let ast = AstNode::from_regex(pattern)?;
    let enfa = Nfa::from_ast(&ast);

    todo!(
        "Generated AST: {:#?}\n\
        Generated ε-NFA: {:#?}\n\
        The rest is a work in progress",
        ast,
        enfa,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_regex() {
        assert!(get_regex("aa|b").is_ok())
    }
}
