use std::collections::HashMap;

use magnitude::Magnitude;
use provide::{Edges, Graph, Neighbors, Vertices};

use super::{AsSubgraph, Subgraph};
use crate::graph::{Edge, EdgeDir};
use crate::provide;

/// Subgraph containing edges and vertices that participate in the shortest path tree.
/// It also carries a distance map to answer queries about shortest paths from source vertex to any destination vertex in O(1).
/// This subgraph will be returned from algorithms like [`Dijkstra`](crate::algo::Dijkstra) or [BellmanFord](crate::algo::BellmanFord).
pub struct ShortestPathSubgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    distance_map: HashMap<usize, Magnitude<W>>,
    subgraph: Subgraph<'a, W, E, Dir, G>,
}

impl<'a, W: Copy, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>>
    ShortestPathSubgraph<'a, W, E, Dir, G>
{
    /// # Arguments
    /// * `graph`: Graph that owns the `edges` and `vertices`.
    /// * `edges`: Edges that are in the subgraph in the format of: (src_id, dst_id, edge).
    /// * `vertices`: Vertices that are in the subgraph.
    /// * `distance_map`: Maps each vertex with id: `dst_id` to its (shortest)distance from vertex with id: `src_id`.
    ///
    /// # Returns
    /// Initialized subgraph containing the specified `edges` and `vertices`.
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, &'a E)>,
        vertices: Vec<usize>,
        distance_map: HashMap<usize, Magnitude<W>>,
    ) -> Self {
        ShortestPathSubgraph {
            distance_map,
            subgraph: Subgraph::init(graph, edges, vertices),
        }
    }

    /// # Arguments
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Some`: Containing the distance from source vertex to vertex with id: `dst_id`.
    /// * `None`: If distance map does not contain an entry about `dst_id`.
    ///
    /// # Complexity
    /// O(1)
    pub fn distance_to(&self, dst_id: usize) -> Option<Magnitude<W>> {
        self.distance_map.get(&dst_id).copied()
    }
}

/// For documentation about each function checkout [`Neighbors`](crate::provide::Neighbors) trait.
/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Neighbors
    for ShortestPathSubgraph<'a, W, E, Dir, G>
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.subgraph.neighbors(src_id)
    }
}

/// For documentation about each function checkout [`Vertices`](crate::provide::Vertices) trait.
/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Vertices
    for ShortestPathSubgraph<'a, W, E, Dir, G>
{
    fn vertices(&self) -> Vec<usize> {
        self.subgraph.vertices()
    }
}

/// For documentation about each function checkout [`Edges`](crate::provide::Edges) trait.
/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Edges<W, E>
    for ShortestPathSubgraph<'a, W, E, Dir, G>
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

impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> AsSubgraph<W, E>
    for ShortestPathSubgraph<'a, W, E, Dir, G>
{
}
