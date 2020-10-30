use magnitude::Magnitude;
use std::any::Any;

use crate::graph::EdgeType;
use crate::provide;
use crate::storage::{GraphStorage, Storage};

/// By simple graph we mean a graph without loops and multiple edges.
/// 
/// Unlike its formal definition which indicates that a simple graph is an unweighted, undirected graph containing no graph loops or multiple edges.\
/// But this can be achieved easily with our `SimpleGraph` too.
pub struct SimpleGraph<W> {
    // Backend storage to store graph data
    storage: Box<dyn GraphStorage<W>>,
}

impl<W: Any + Copy> SimpleGraph<W> {
    /// Initialize the graph with specified `storage` and `edge_type`.
    ///
    /// # Arguments:
    /// * `storage`: Indicates what type of storage should this graph use for storing its data.
    /// * `edge_type`: Indicates edges of the graph are directed or undirected.
    ///
    /// # Returns:
    /// * Initialized graph.
    pub fn init(storage: Storage, edge_type: EdgeType) -> Self {
        SimpleGraph {
            storage: storage.init::<W>(edge_type),
        }
    }

    /// Initializes the graph with a custom storage.
    ///
    /// # Arguments:
    /// * `storage`: The storage for graph to use to store its data.
    ///
    /// # Returns:
    /// * Initialized graph.
    pub fn init_with_storage(storage: Box<dyn GraphStorage<W>>) -> Self {
        SimpleGraph { storage }
    }
}

impl<W> provide::Neighbors for SimpleGraph<W> {
    /// # Returns:
    /// Id of neighbors of the vertex with `src_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.storage.neighbors(src_id)
    }
}

impl<W> provide::Vertices for SimpleGraph<W> {
    /// # Returns:
    /// Vector of vertex ids that are present in the graph.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn vertices(&self) -> Vec<usize> {
        self.storage.vertices()
    }

    /// # Returns:
    /// Number of vertices present in the graph.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn vertex_count(&self) -> usize {
        self.storage.vertex_count()
    }
}

impl<W> provide::Edges<W> for SimpleGraph<W> {
    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `weight`).
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)> {
        self.storage.edges()
    }

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `weight`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges_from(&self, src_id: usize) -> Vec<(usize, Magnitude<W>)> {
        self.storage.edges_from(src_id)
    }
}

impl<W> provide::Graph<W> for SimpleGraph<W> {
    /// Adds a vertex into the graph.
    ///
    /// # Returns:
    /// Id of the newly inserted vertex.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn add_vertex(&mut self) -> usize {
        self.storage.add_vertex()
    }

    /// Removes a vertex from the graph.
    ///
    /// # Arguments:
    /// * `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn remove_vertex(&mut self, vertex_id: usize) {
        self.storage.remove_vertex(vertex_id);
    }

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    /// * `weight`: Weight of the edge between `src_id` and `dst_id`.
    ///
    /// # Panics:
    /// * If `src_id` == `dst_id`: because it causes a loop.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, weight: Magnitude<W>) {
        if src_id == dst_id {
            panic!("Can not create loop in simple graph")
        }

        self.storage.add_edge(src_id, dst_id, weight);
    }

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The weight of the removed edge.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> Magnitude<W> {
        self.storage.remove_edge(src_id, dst_id)
    }

    /// # Returns:
    /// `true`: If edges stored in the graph are directed `false` otherwise.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn is_directed(&self) -> bool {
        self.storage.is_directed()
    }
}
