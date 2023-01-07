use itertools::Itertools;

use crate::gen::{EmptyGraph, Generate};
use crate::provide::*;

#[derive(Debug)]
pub struct PathGraph {
    node_count: usize,
}

impl PathGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Generate<S> for PathGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn generate_into(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = EmptyGraph::new(self.node_count).generate_into(storage, start);

        for (src, dst) in (start.0..next.0).tuple_windows() {
            storage.add_edge(NodeID(src), NodeID(dst));
        }

        next
    }
}
