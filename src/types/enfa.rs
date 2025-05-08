use crate::{AstNode, Character};
use petgraph::stable_graph::{NodeIndex, StableDiGraph};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Condition {
    Char(Character),
    Epsilon,
}

#[derive(Debug)]
pub struct Nfa {
    graph: StableDiGraph<(), Condition>,
    root: NodeIndex,
    final_state: NodeIndex,
}

impl Nfa {
    pub fn new() -> Self {
        let mut graph = StableDiGraph::new();
        let root = graph.add_node(());
        let final_state = graph.add_node(());

        // I might need to add this ε transition to handle empty inputs.
        // graph.add_edge(root, final_state, Condition::Epsilon);

        Self {
            graph,
            root,
            final_state,
        }
    }

    pub fn add_node(&mut self) -> NodeIndex {
        self.graph.add_node(())
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, condition: Condition) {
        self.graph.add_edge(a, b, condition);
    }

    pub fn from_ast(ast: &AstNode) -> Self {
        let mut nfa = Self::new();
        let current_node = nfa.root;

        let mut queue: VecDeque<(&AstNode, NodeIndex, NodeIndex)> =
            VecDeque::from([(ast, current_node, nfa.final_state)]);

        while let Some((node, a, b)) = queue.pop_front() {
            match node {
                AstNode::Character(character) => {
                    nfa.add_edge(a, b, Condition::Char(character.clone()));
                }
                AstNode::Concat(first, second) => {
                    let intermediate = nfa.add_node();
                    queue.push_back((first, a, intermediate));
                    queue.push_back((second, intermediate, b));
                }
                AstNode::Alternate(first, second) => {
                    let x1 = nfa.add_node();
                    let x2 = nfa.add_node();
                    let y1 = nfa.add_node();
                    let y2 = nfa.add_node();

                    nfa.add_edge(a, x1, Condition::Epsilon);
                    nfa.add_edge(x2, b, Condition::Epsilon);
                    nfa.add_edge(a, y1, Condition::Epsilon);
                    nfa.add_edge(y2, b, Condition::Epsilon);

                    queue.push_back((first, x1, x2));
                    queue.push_back((second, y1, y2));
                },
            }
        }

        nfa
    }
}