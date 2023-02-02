use crate::give::*;
use crate::make::{Make, PathGraph};

#[derive(Debug)]
pub struct CycleGraph {
    node_count: usize,
}

impl CycleGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Make<S> for CycleGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = PathGraph::new(self.node_count).append(storage, start);

        storage.add_edge(NodeID(next.0 - 1), start);

        next
    }
}
