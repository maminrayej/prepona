mod adj_matrix;

pub use adj_matrix::AdjMatrix;

use magnitude::Magnitude;
use std::any::Any;

use crate::graph::EdgeType;

/// Different types of storage a graph can use to store its data
pub enum Storage {
    /// An adjacency matrix
    AdjMatrix,
}

impl Storage {
    /// Initializes a storage.
    ///
    /// # Arguments:
    /// * `edge_type`: indicates wether the storage stores directed or undirected edges.
    ///
    /// # Returns:
    /// A struct that can act as a storage for graph data
    pub fn init<W: Any + Copy>(&self, edge_type: EdgeType) -> Box<dyn GraphStorage<W>> {
        Box::new(match self {
            Storage::AdjMatrix => AdjMatrix::<W>::init(edge_type),
        })
    }
}

/// Trait that describes a struct that can act as a storage for graph data.
pub trait GraphStorage<W> {
    /// Adds a vertex into the adjacency matrix.
    ///
    /// # Returns:
    /// id of the newly inserted vertex
    fn add_vertex(&mut self) -> usize;

    /// Removes a vertex from the adjacency matrix.
    ///
    /// # Arguments:
    /// * `vertex_id`: id of the vertex to be removed
    fn remove_vertex(&mut self, vertex_id: usize);

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: id of the vertex at the start of the edge
    /// * `dst_id`: id of the vertex at the end of the edge
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge_weight: Magnitude<W>);

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: id of the vertex at the start of the edge
    /// * `dst_id`: id of the vertex at the end of the edge
    ///
    /// # Returns:
    /// The weight of the removed edge
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> Magnitude<W>;

    /// # Returns:
    /// number of vertices present in the graph
    fn vertex_count(&self) -> usize;

    /// # Returns:
    /// vector of vertex ids that are present in the graph
    fn vertices(&self) -> Vec<usize>;

    /// # Returns:
    /// vector of edges in the format of (`src_id`, `dst_id`, `weight`)
    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)>;

    /// # Returns:
    /// Vectors of edges from vertex with `src_id` in the format of (`dst_id`, `weight`)
    ///
    /// # Arguments:
    /// * `src_id`: id of the source vertex
    fn edges_from(&self, src_id: usize) -> Vec<(usize, Magnitude<W>)>;

    /// # Returns:
    /// Id of neighbors of the vertex with `src_id`
    ///
    /// # Arguments:
    /// * `src_id`: id of the source vertex
    fn neighbors(&self, src_id: usize) -> Vec<usize>;

    /// # Returns:
    /// `true` if edges stored in the matrix is directed `false` otherwise
    fn is_directed(&self) -> bool;

    /// # Returns:
    /// `true` if edges stored in the matrix is undirected `false` otherwise
    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}
