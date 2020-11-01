mod adj_matrix;

pub use adj_matrix::{AdjMatrix, FlowMat, Mat};

use crate::graph::Edge;

/// Trait that describes a struct that can act as a storage for graph data.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
/// * `E`: Edge of the graph.
pub trait GraphStorage<W, E: Edge<W>> {
    /// Adds a vertex to the storage.
    ///
    /// # Returns:
    /// Id of the newly inserted vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes a vertex from the storage.
    ///
    /// # Arguments:
    /// * `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize);

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    /// * `edge`: The edge between `src_id` and `dst_id`.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E);

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The removed edge.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E;

    /// # Returns:
    /// Number of vertices present in the graph.
    fn vertex_count(&self) -> usize;

    /// # Returns:
    /// Vector of vertex ids that are present in the graph.
    fn vertices(&self) -> Vec<usize>;

    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `edge`).
    fn edges(&self) -> Vec<(usize, usize, &E)>;

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `edge`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)>;

    /// # Returns:
    /// Id of neighbors of the vertex with `src_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    fn neighbors(&self, src_id: usize) -> Vec<usize>;

    /// # Returns:
    /// `true`: If edges stored in the matrix are directed `false` otherwise.
    fn is_directed(&self) -> bool;

    /// # Returns:
    /// `true`: If edges stored in the matrix are undirected `false` otherwise.
    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
