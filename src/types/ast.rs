use crate::Error;
use std::collections::hash_set::HashSet;
use std::ops::{Add, BitOr};

#[derive(Debug, PartialEq, Clone)]
pub enum Character {
    Literal(char),
    Pattern(CharacterPattern),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharacterPattern {
    Include(HashSet<char>),
    Exclude(HashSet<char>),
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Character(Character),
    Concat(Box<AstNode>, Box<AstNode>),
    Alternate(Box<AstNode>, Box<AstNode>),  // or `|`
}

use Character::*;
use CharacterPattern::*;

impl AstNode {
    pub fn literal(character: char) -> Self {
        Self::Character(Literal(character))
    }

    pub fn character_pattern_inclusive(pattern: HashSet<char>) -> Self {
        Self::Character(Pattern(Include(pattern)))
    }

    pub fn character_pattern_exclusive(pattern: HashSet<char>) -> Self {
        Self::Character(Pattern(Exclude(pattern)))
    }

    fn regex_to_ast(pattern: &str, index: usize, total_size: usize) -> Result<Self, Error> {
        let first_character = pattern.chars().next().ok_or(Error::UnexpectedEof(index))?;
        let mut next_index = 1;

        let node = {
            match first_character {
                '[' => {
                    let pattern_is_inclusive = pattern.chars().nth(1).ok_or(Error::UnexpectedEof(index))? != '^';
                    let mut pattern_set: HashSet<char> = HashSet::new();
                    let mut iter = pattern.chars();
                    iter.next();
                    next_index += 1;
                    if !pattern_is_inclusive {
                        iter.next();
                        next_index += 1;
                    }
                    let mut closed = false;
                    while let Some(character) = iter.next() {
                        next_index += 1;

                        if character == '\\' {
                            pattern_set.insert(iter.next().ok_or(Error::UnexpectedEof(index))?);
                            next_index += 1;
                        } else if character == ']' {
                            closed = true;
                            break;
                        } else {
                            pattern_set.insert(character);
                        }
                    }
                    if !closed {
                        return Err(Error::UnmatchedBracket(index));
                    }
                    if pattern_is_inclusive {
                        Ok(Self::character_pattern_inclusive(pattern_set))
                    } else {
                        Ok(Self::character_pattern_exclusive(pattern_set))
                    }
                }
                '|' => {
                    return Err(Error::OrRefactor(Self::regex_to_ast(&pattern[next_index..], index + next_index, total_size)?))
                }
                _ => Ok(Self::literal(first_character)),
            }
        };

        if index + next_index >= total_size {
            node
        } else {
            match Self::regex_to_ast(&pattern[next_index..], index + next_index, total_size) {
                Ok(next_node) => {
                    Ok(node? + next_node)
                }
                Err(e) => match e {
                    Error::OrRefactor(next_node) => {
                        Ok(node? | next_node)
                    },
                    _ => Err(e)
                }
            }
        }
    }

    pub fn from_regex(pattern: &str) -> Result<Self, Error> {
        Self::regex_to_ast(pattern, 0, pattern.len())
    }
}

impl Add for AstNode {
    type Output = AstNode;

    /// The add function is always **left-associative**, but the AST generates right-associative.
    /// Whenever you're adding two nodes, always remember the brackets.
    ///
    /// If the input is `abc`, `from_regex` will return `a + (b + c)`.
    fn add(self, rhs: Self) -> Self::Output {
        AstNode::Concat(Box::new(self), Box::new(rhs))
    }
}

impl BitOr for AstNode {
    type Output = AstNode;

    /// This is not a bitwise operation. It's a workaround to overload the `or` operator.
    fn bitor(self, rhs: Self) -> Self::Output {
        AstNode::Alternate(Box::new(self), Box::new(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn literals() {
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
