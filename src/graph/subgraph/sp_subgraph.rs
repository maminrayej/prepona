use std::collections::{HashMap, HashSet};

use magnitude::Magnitude;
use provide::{Edges, Graph, Neighbors, Vertices};

use super::{AsFrozenSubgraph, Subgraph};
use crate::graph::{Edge, EdgeDir};
use crate::provide;

/// Subgraph containing edges and vertices that participate in the shortest path tree.
///
/// It also carries a distance map to answer queries about shortest paths from source vertex to any destination vertex in O(1).
/// This subgraph will be returned from algorithms like [`Dijkstra`](crate::algo::Dijkstra) or [BellmanFord](crate::algo::BellmanFord).
pub struct ShortestPathSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    distance_map: HashMap<usize, Magnitude<W>>,
    subgraph: Subgraph<'a, W, E, Dir, G>,
}

impl<'a, W, E, Dir, G> ShortestPathSubgraph<'a, W, E, Dir, G>
where
    W: Copy,
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
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
        edges: Vec<(usize, usize, usize)>,
        vertices: HashSet<usize>,
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

/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Neighbors for ShortestPathSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn neighbors(&self, src_id: usize) -> anyhow::Result<Vec<usize>> {
        self.subgraph.neighbors(src_id)
    }

    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        self.subgraph.neighbors_unchecked(src_id)
    }
}

/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Vertices for ShortestPathSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn vertices(&self) -> Vec<usize> {
        self.subgraph.vertices()
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.subgraph.contains_vertex(vertex_id)
    }
}

/// `ShortestPathSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Edges<W, E> for ShortestPathSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn edges_from(&self, src_id: usize) -> anyhow::Result<Vec<(usize, &E)>> {
        self.subgraph.edges_from(src_id)
    }

    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.subgraph.edges_from_unchecked(src_id)
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> anyhow::Result<Vec<&E>> {
        self.subgraph.edges_between(src_id, dst_id)
    }

    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.subgraph.edges_between_unchecked(src_id, dst_id)
    }

    fn edge_between(
        &self,
        src_id: usize,
        dst_id: usize,
        edge_id: usize,
    ) -> anyhow::Result<&E> {
        self.subgraph.edge_between(src_id, dst_id, edge_id)
    }

    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E {
        self.subgraph
            .edge_between_unchecked(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> anyhow::Result<&E> {
        self.subgraph.edge(edge_id)
    }

    fn edge_unchecked(&self, edge_id: usize) -> &E {
        self.subgraph.edge_unchecked(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> anyhow::Result<bool> {
        self.subgraph.has_any_edge(src_id, dst_id)
    }

    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        self.subgraph.has_any_edge_unchecked(src_id, dst_id)
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

    fn contains_edge(&self, edge_id: usize) -> bool {
        self.subgraph.contains_edge(edge_id)
    }
}

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for ShortestPathSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
}
