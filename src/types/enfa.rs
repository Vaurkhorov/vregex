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

        Self {
            graph,
            root,
            final_state: root,
        }
    }

    pub fn push(&mut self, root: NodeIndex, condition: Condition) -> NodeIndex {
        let new_node = self.graph.add_node(());
        self.graph.add_edge(root, new_node, condition);
        new_node
    }

    pub fn add_node(&mut self) -> NodeIndex {
        self.graph.add_node(())
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, condition: Condition) {
        self.graph.add_edge(a, b, condition);
    }

    pub fn set_node_final(&mut self, node: NodeIndex) {
        self.final_state = node
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
            }
        }

        nfa
    }
}
