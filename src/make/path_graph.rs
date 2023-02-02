use itertools::Itertools;

use crate::give::*;
use crate::make::{EmptyGraph, Make};

#[derive(Debug)]
pub struct PathGraph {
    node_count: usize,
}

impl PathGraph {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl<S> Make<S> for PathGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let next = EmptyGraph::new(self.node_count).append(storage, start);

        for (src, dst) in (start.0..next.0).tuple_windows() {
            storage.add_edge(NodeID(src), NodeID(dst));
        }

        next
    }
}
