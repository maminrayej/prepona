use crate::gen::Generate;
use crate::provide::*;

#[derive(Debug)]
pub struct EmptyGraph {
    node_count: usize,
}

impl EmptyGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Generate<S> for EmptyGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn generate_into(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = start.0 + self.node_count;

        for i in start.0..next {
            storage.add_node(NodeID(i));
        }

        NodeID(next)
    }
}
