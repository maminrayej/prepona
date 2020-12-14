mod mr_subgraph;
mod sp_subgraph;

use crate::provide::{Edges, Graph, Neighbors, Vertices};
pub use mr_subgraph::MultiRootSubgraph;
pub use sp_subgraph::ShortestPathSubgraph;

use crate::graph::Edge;
use std::marker::PhantomData;

use super::EdgeDir;

/// Implementing this trait allows the algorithms to get executed on subgraphs as well.
pub trait AsSubgraph<W, E: Edge<W>>: Neighbors + Vertices + Edges<W, E> {}

/// Provides functionalities to mutate the subgraph in a shrinking manner.
pub trait AsMutSubgraph<W, E: Edge<W>>: AsSubgraph<W, E> {
    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize);

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize);
}

/// Default subgraph struct.
///
/// ## Note
/// From now on:
/// * |E|: Means number of edges that are in the subgraph.
/// * |V|: Means number of vertices that are in the subgraph.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `G`: **G**raph type that subgraph is representing.
pub struct Subgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    #[allow(dead_code)]
    graph: &'a G,

    edges: Vec<(usize, usize, &'a E)>,
    vertices: Vec<usize>,

    phantom_w: PhantomData<W>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Subgraph<'a, W, E, Dir, G> {
    /// # Arguments
    /// * `graph`: Graph that owns the `edges` and `vertices`.
    /// * `edges`: Edges that are in the subgraph in the format of: (src_id, dst_id, edge).
    /// * `vertices`: Vertices that are in the subgraph.
    ///
    /// # Returns
    /// Initialized subgraph containing the specified `edges` and `vertices`.
    pub fn init(graph: &'a G, edges: Vec<(usize, usize, &'a E)>, vertices: Vec<usize>) -> Self {
        Subgraph {
            graph,
            edges,
            vertices,

            phantom_w: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

/// For documentation about each function checkout [`AsMutSubgraph`](crate::graph::subgraph::AsMutSubgraph) trait.
/// Here only complexity of each function is provided.
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> AsMutSubgraph<W, E>
    for Subgraph<'a, W, E, Dir, G>
{
    /// # Complexity
    /// O(|E|)
    fn remove_edge(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, edge)| edge.get_id() != edge_id);
    }

    /// # Complexity
    /// O(|V| + |E|)
    fn remove_vertex(&mut self, vertex_id: usize) {
        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);

        self.vertices.retain(|v_id| *v_id != vertex_id);
    }
}

/// For documentation about each function checkout [`Neighbors`](crate::provide::Neighbors) trait.
/// Here only complexity of each function is provided.
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Neighbors
    for Subgraph<'a, W, E, Dir, G>
{
    /// # Complexity
    //// O(|E|)
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|(s_id, dst_id, _)| if *s_id == src_id { Some(*dst_id) } else { None })
            .collect()
    }
}

/// For documentation about each function checkout [`Vertices`](crate::provide::Vertices) trait.
/// Here only complexity of each function is provided.
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Vertices for Subgraph<'a, W, E, Dir, G> {
    /// # Complexity
    /// O(|V|)
    fn vertices(&self) -> Vec<usize> {
        self.vertices.iter().copied().collect()
    }
}

/// For documentation about each function checkout [`Edges`](crate::provide::Edges) trait.
/// Here only complexity of each function is provided.
impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> Edges<W, E>
    for Subgraph<'a, W, E, Dir, G>
{
    /// # Complexity
    /// O(|E|)
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.edges
            .iter()
            .filter_map(|(s_id, dst_id, edge)| {
                if *s_id == src_id {
                    Some((*dst_id, *edge))
                } else {
                    None
                }
            })
            .collect()
    }

    /// # Complexity
    /// O(|E|)
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.edges
            .iter()
            .filter_map(|(s_id, d_id, edge)| {
                if *s_id == src_id && *d_id == dst_id {
                    Some(*edge)
                } else {
                    None
                }
            })
            .collect()
    }

    /// # Complexity
    /// O(|E|)
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
    }

    /// # Complexity
    /// O(|E|)
    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .and_then(|(_, _, edge)| Some(*edge))
    }

    /// # Complexity
    /// O(|E|)
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.edges()
        } else {
            self.edges
                .iter()
                .flat_map(|(src_id, dst_id, edge)| {
                    vec![(*src_id, *dst_id, *edge), (*dst_id, *src_id, *edge)]
                })
                .collect()
        }
    }

    /// # Complexity
    /// O(|E|)
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    /// # Complexity
    /// O(|E|)
    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.edges.iter().copied().collect()
    }

    /// # Complexity
    /// O(1)
    fn edges_count(&self) -> usize {
        self.edges.len()
    }
}

impl<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> AsSubgraph<W, E>
    for Subgraph<'a, W, E, Dir, G>
{
}
