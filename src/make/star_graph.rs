use crate::give::*;
use crate::make::{EmptyGraph, Make};

#[derive(Debug)]
pub struct StarGraph {
    node_count: usize,
}

impl StarGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Make<S> for StarGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = EmptyGraph::new(self.node_count).append(storage, start);

        for other in (start.0..next.0).skip(1) {
            storage.add_edge(start, NodeID(other))
        }

        next
    }
}
