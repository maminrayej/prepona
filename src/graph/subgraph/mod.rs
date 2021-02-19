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

/// Describes a subgraph that can not get mutated(Not itself nor the graph it's representing).
pub trait AsFrozenSubgraph<W, E: Edge<W>>: Neighbors + Vertices + Edges<W, E> {}

/// Describes a subgraph that can mutate but the graph that it represents, can not mutate.
///
/// Obviously you can remove vertices and edges from the subgaph but it does not remove them from the graph.
/// You can also add already existing vertices and edges from graph to subgraph.
pub trait AsSubgraph<W, E: Edge<W>>: AsFrozenSubgraph<W, E> {
    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()>;

    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize);

    /// Removes the vertex with id: `vertex_id` from subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()>;

    /// Removes the vertex with id: `vertex_id` from subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize);

    /// Adds a vertex that is already in the graph, to the subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added from graph to subgraph.
    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()>;

    /// Adds a vertex that is already in the graph, to the subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added from graph to subgraph.
    fn add_vertex_from_graph_unchecked(&mut self, vertex_id: usize);

    /// Adds an edge that is already in the graph, to the subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source vertex to destination vertex.
    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()>;

    /// Adds an edge that is already in the graph, to the subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source vertex to destination vertex.
    fn add_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize);
}

/// Describes a subgraph that can mutate and also can mutate the graph it's representing.
///
/// Adding an edge or a vertex to the subgraph that is completely new(is not already present in the subgraph), will be added to the graph as well.
/// Also removing an edge or vertex from the graph that is also present in the subgraph, will get removed from the subgarph as well. 
pub trait AsMutSubgraph<W, E: Edge<W>>: AsSubgraph<W, E> {
    /// Removes a vertex from the graph.
    /// If the vertex is present in the subgraph, It will get removed from the subgraph as well.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to get removed.
    fn remove_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()>;

    /// Removes a vertex from the graph.
    /// If the vertex is present in the subgraph, It will get removed from the subgraph as well.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to get removed.
    fn remove_vertex_from_graph_unchecked(&mut self, vertex_id: usize);

    /// Removes an edge from the graph.
    /// If the edge is present in the subgraph, It will get removed from the subgraph as well.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source vertex to destination vertex.
    ///
    /// # Returns
    /// * `Err`: 
    /// * `Ok`: Containing the removed edge.
    fn remove_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E>;

    /// Removes an edge from the graph.
    /// If the edge is present in the subgraph, It will get removed from the subgraph as well.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source vertex to destination vertex.
    ///
    /// # Returns
    /// The removed edge.
    fn remove_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E;

    /// Adds a new vertex to subgraph and the graph it's representing.
    ///
    /// # Returns
    /// Id of the new added vertex.
    fn add_vertex(&mut self) -> usize;

    /// Adds a new edge to subgraph and the graph it's representing.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added to subgraph.
    ///
    /// # Returns
    /// * `Err`:
    /// * `Ok`: Containing the id of the newly added edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize>;

    /// Adds a new edge to subgraph and the graph it's representing.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added to subgraph.
    ///
    /// # Returns
    /// The id of the newly added edge.
    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;
}
