mod def_mut_subgraph;
mod def_subgraph;
mod mr_subgraph;
mod sp_subgraph;

use crate::graph::Edge;
use crate::provide::{Edges, Neighbors, Vertices};

use anyhow::Result;
pub use def_mut_subgraph::MutSubgraph;
pub use def_subgraph::Subgraph;
pub use mr_subgraph::MultiRootSubgraph;
pub use sp_subgraph::ShortestPathSubgraph;

/// Describes a subgraph that neither graph nor subgraph can be mutated.
pub trait AsFrozenSubgraph<W, E: Edge<W>>: Neighbors + Vertices + Edges<W, E> {}

/// Describes a subgraph that can mutate but the graph that it represents, can not mutate.
pub trait AsSubgraph<W, E: Edge<W>>: AsFrozenSubgraph<W, E> {
    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()>;

    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize);

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()>;

    fn remove_vertex_unchecked(&mut self, vertex_id: usize);

    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()>;

    fn add_vertex_from_graph_unchecked(&mut self, vertex_id: usize);

    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()>;

    fn add_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize);
}

/// Provides functionalities to mutate the subgraph in a shrinking manner.
pub trait AsMutSubgraph<W, E: Edge<W>>: AsSubgraph<W, E> {
    fn remove_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()>;

    fn remove_vertex_from_graph_unchecked(&mut self, vertex_id: usize);

    fn remove_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<Option<E>>;

    fn remove_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E>;

    fn add_vertex(&mut self) -> usize;

    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize>;

    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;
}
