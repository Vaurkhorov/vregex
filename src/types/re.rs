use crate::Error;
use crate::Nfa;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct RegEx {
    nfa: Nfa,
}

impl RegEx {
    pub fn from_nfa(nfa: &Nfa) -> Self {
        Self { nfa: nfa.clone() }
    }

    pub fn from_pattern(regex: &str) -> Result<Self, Error> {
        let ast = crate::AstNode::from_pattern(regex)?;
        let nfa = crate::Nfa::from_ast(&ast);
        Ok(Self::from_nfa(&nfa))
    }

    pub fn recognise(&self, string: &str) -> bool {
        let mut current_states = self.nfa.get_null_closure(self.nfa.get_root());

        for character in string.chars() {
            let mut next_states: HashSet<NodeIndex> = HashSet::new();
            for state in current_states.iter() {
                let transition_states = self.nfa.get_transition(
                    *state,
                    super::enfa::Condition::Char(crate::types::ast::Character::Literal(character)),
                );
                for next_state in transition_states {
                    next_states.extend(self.nfa.get_null_closure(next_state));
                }
            }

            if next_states.contains(&self.nfa.get_final_state()) {
                return true;
            }

            current_states = next_states
        }

        false
    }

    pub fn search(&self, haystack: &str) -> Option<usize> {
        let length = haystack.len();

        (0..length).find(|&i| self.recognise(&haystack[i..]))
    }
}
