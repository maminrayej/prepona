use crate::give::*;
use crate::make::{EmptyGraph, Make};

#[derive(Debug)]
pub struct CompleteGraph {
    node_count: usize,
}

impl CompleteGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Make<S> for CompleteGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = EmptyGraph::new(self.node_count).append(storage, start);

        for src in start.0..next.0 {
            for dst in start.0..next.0 {
                if (S::Dir::is_undirected() && src < dst) || src != dst {
                    storage.add_edge(NodeID(src), NodeID(dst));
                }
            }
        }

        next
    }
}
