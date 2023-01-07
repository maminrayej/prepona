use crate::gen::{Generate, PathGraph};
use crate::provide::*;

#[derive(Debug)]
pub struct CycleGraph {
    node_count: usize,
}

impl CycleGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Generate<S> for CycleGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn generate_into(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = PathGraph::new(self.node_count).generate_into(storage, start);

        storage.add_edge(NodeID(next.0 - 1), start);

        next
    }
}
