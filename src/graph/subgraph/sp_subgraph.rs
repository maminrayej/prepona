use std::collections::HashMap;

use magnitude::Magnitude;

use super::{AsSubgraph, Subgraph};
use crate::graph::Edge;
use crate::provide;

pub struct ShortestPathSubgraph<'a, W, E: Edge<W>> {
    distance_map: HashMap<usize, Magnitude<W>>,
    subgraph: Subgraph<'a, W, E>,
}

impl<'a, W: Copy, E: Edge<W>> ShortestPathSubgraph<'a, W, E> {
    pub fn init(edges: Vec<(usize, usize, &'a E)>, vertices: Vec<usize>, distance_map: HashMap<usize, Magnitude<W>>) -> Self {
        ShortestPathSubgraph {
            distance_map,
            subgraph: Subgraph::init(edges, vertices),
        }
    }

    pub fn distance_to(&self, dst_id: usize) -> Option<Magnitude<W>> {
        self.distance_map.get(&dst_id).copied()
    }
}

impl<'a, W, E: Edge<W>> provide::Neighbors for ShortestPathSubgraph<'a, W, E> {
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.subgraph.neighbors(src_id)
    }
}

impl<'a, W, E: Edge<W>> provide::Vertices for ShortestPathSubgraph<'a, W, E> {
    fn vertices(&self) -> Vec<usize> {
        self.subgraph.vertices()
    }
}

impl<'a, W, E: Edge<W>> provide::Edges<W, E> for ShortestPathSubgraph<'a, W, E> {
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.subgraph.edges_from(src_id)
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.subgraph.edges_between(src_id, dst_id)
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.subgraph.edge_between(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.subgraph.edge(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        self.subgraph.has_any_edge(src_id, dst_id)
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.subgraph.edges()
    }

    fn edges_count(&self) -> usize {
        self.subgraph.edges_count()
    }
}

impl<'a, W, E: Edge<W>> AsSubgraph<W, E> for ShortestPathSubgraph<'a, W, E> {}
