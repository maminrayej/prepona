use crate::give::*;
use crate::make::{CycleGraph, Make};

#[derive(Debug)]
pub struct WheelGraph {
    node_count: usize,
}

impl WheelGraph {
    pub fn new(node_count: usize) -> Self {
        if node_count < 4 {
            panic!("Cannot generate a wheel graph with less than 4 vertex: {node_count} < 4");
        }

        Self { node_count }
    }
}

impl<S> Make<S> for WheelGraph
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn append(&self, storage: &mut S, start: NodeID) -> NodeID {
        let center = CycleGraph::new(self.node_count - 1).append(storage, start);

        storage.add_node(center);

        for other in start.0..center.0 {
            storage.add_edge(center, NodeID(other));
        }

        NodeID(center.0 + 1)
    }
}
