use crate::give::*;
use crate::make::Make;

#[derive(Debug)]
pub struct EmptyGraph {
    node_count: usize,
}

impl EmptyGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Make<S> for EmptyGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = start.0 + self.node_count;

        for i in start.0..next {
            storage.add_node(NodeID(i));
        }

        NodeID(next)
    }
}
