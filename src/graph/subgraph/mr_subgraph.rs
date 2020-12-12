use crate::provide::{Edges, Graph, Neighbors, Vertices};

use super::{AsSubgraph, Subgraph};
use crate::graph::{Edge, EdgeDir};

pub struct MultiRootSubgraph<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> {
    roots: Vec<usize>,
    subgraph: Subgraph<'a, W, E, Ty, G>,
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> MultiRootSubgraph<'a, W, E, Ty, G> {
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, &'a E)>,
        vertices: Vec<usize>,
        roots: Vec<usize>,
    ) -> Self {
        MultiRootSubgraph {
            roots,
            subgraph: Subgraph::init(graph, edges, vertices),
        }
    }

    pub fn roots(&self) -> &Vec<usize> {
        &self.roots
    }

    pub fn is_root(&self, vertex_id: usize) -> bool {
        self.roots
            .iter()
            .find(|root_id| **root_id == vertex_id)
            .is_some()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Neighbors
    for MultiRootSubgraph<'a, W, E, Ty, G>
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.subgraph.neighbors(src_id)
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Vertices
    for MultiRootSubgraph<'a, W, E, Ty, G>
{
    fn vertices(&self) -> Vec<usize> {
        self.subgraph.vertices()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Edges<W, E>
    for MultiRootSubgraph<'a, W, E, Ty, G>
{
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

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.subgraph.as_directed_edges()
    }

    fn edges_count(&self) -> usize {
        self.subgraph.edges_count()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> AsSubgraph<W, E>
    for MultiRootSubgraph<'a, W, E, Ty, G>
{
}
