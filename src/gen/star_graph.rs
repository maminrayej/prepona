use crate::gen::{EmptyGraph, Generate};
use crate::provide::*;

#[derive(Debug)]
pub struct StarGraph {
    node_count: usize,
}

impl StarGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Generate<S> for StarGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn generate_into(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = EmptyGraph::new(self.node_count).generate_into(storage, start);

        for other in (start.0..next.0).skip(1) {
            storage.add_edge(start, NodeID(other))
        }

        next
    }
}
