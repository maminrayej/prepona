use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::{
    graph::error::Error,
    provide::{Edges, Graph, Neighbors, Vertices},
};

use super::{AsFrozenSubgraph, AsSubgraph, Subgraph};
use crate::graph::{Edge, EdgeDir};

/// A subgraph with some vertices elected as root.
///
/// ## Note
/// From now on:
/// * |R|: Means number of roots in the subgraph.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `G`: **G**raph type that subgraph is representing.
pub struct MultiRootSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    roots: Vec<usize>,
    subgraph: Subgraph<'a, W, E, Dir, G>,
}

impl<'a, W, E, Dir, G> MultiRootSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors + Vertices,
{
    /// # Arguments
    /// * `graph`: Graph that owns the `edges` and `vertices`.
    /// * `edges`: Edges that are in the subgraph in the format of: (src_id, dst_id, edge).
    /// * `vertices`: Vertices that are in the subgraph.
    /// * `roots`: Roots of the subgraph.
    ///
    /// # Returns
    /// Initialized subgraph containing the specified `edges` and `vertices` and `roots` as roots of the subgraph.
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, usize)>,
        vertices: HashSet<usize>,
        roots: Vec<usize>,
    ) -> Self {
        MultiRootSubgraph {
            roots,
            subgraph: Subgraph::init(graph, edges, vertices),
        }
    }

    /// # Returns
    /// Roots of the subgraph.
    ///
    /// # Complexity
    /// O(1)
    pub fn roots(&self) -> &Vec<usize> {
        &self.roots
    }

    /// # Arguments
    /// `vertex_id`: Id of the vertex to be checked wether is a root or not.
    ///
    /// # Returns
    /// * `true`: If vertex with id: `vertex_id` is a root.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O(|R|)
    pub fn is_root(&self, vertex_id: usize) -> bool {
        self.roots.contains(&vertex_id)
    }

    /// Adds a new root to the set of roots.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added as root.
    ///
    /// # Returns
    /// * `Err`:
    ///     * If root with specified id already exists
    ///     * Error of calling `add_vertex_from_graph`.
    /// * `Ok`:
    pub fn add_root(&mut self, vertex_id: usize) -> Result<()> {
        if self.is_root(vertex_id) {
            Err(Error::new_rae(vertex_id)).with_context(|| "MultiRootSubgraph failed")?
        } else {
            self.add_vertex_from_graph(vertex_id)?;

            self.roots.push(vertex_id);

            Ok(())
        }
    }

    /// Adds a new root to the set of roots.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added as root.
    pub fn add_root_uncheckec(&mut self, vertex_id: usize) {
        self.add_vertex_from_graph_unchecked(vertex_id);

        self.roots.push(vertex_id);
    }
}

/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Neighbors for MultiRootSubgraph<'a, W, E, Dir, G>
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

/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Vertices for MultiRootSubgraph<'a, W, E, Dir, G>
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

/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> Edges<W, E> for MultiRootSubgraph<'a, W, E, Dir, G>
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

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> anyhow::Result<&E> {
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

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for MultiRootSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
}

/// `MultiRootSubgraph` uses `Subgraph` internally so for complexity of each function checkout [`Subgraph`](crate::graph::subgraph::Subgraph).
impl<'a, W, E, Dir, G> AsSubgraph<W, E> for MultiRootSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors + Vertices,
{
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        self.subgraph.remove_edge(src_id, dst_id, edge_id)
    }

    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        self.subgraph.remove_edge_unchecked(src_id, dst_id, edge_id)
    }

    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        self.subgraph.remove_vertex(vertex_id)?;

        self.roots.retain(|v_id| *v_id != vertex_id);

        Ok(())
    }

    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        self.subgraph.remove_vertex_unchecked(vertex_id);

        self.roots.retain(|v_id| *v_id != vertex_id);
    }

    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()> {
        self.subgraph.add_vertex_from_graph(vertex_id)
    }

    fn add_vertex_from_graph_unchecked(&mut self, vertex_id: usize) {
        self.subgraph.add_vertex_from_graph_unchecked(vertex_id)
    }

    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        self.subgraph.add_edge_from_graph(src_id, dst_id, edge_id)
    }

    fn add_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        self.subgraph
            .add_edge_from_graph_unchecked(src_id, dst_id, edge_id)
    }
}
