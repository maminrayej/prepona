use std::any::Any;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, Edge, FlowEdge};
use crate::provide;
use crate::storage::{FlowMat, GraphStorage, Mat};

/// A `SimpleGraph` with a `DefaultEdge` that uses `AdjMatrix` as its storage.
pub type MatGraph<W> = SimpleGraph<W, DefaultEdge<W>, Mat<W>>;

/// A `SimpleGraph` with a `FlowEdge` that uses `AdjMatrix` as its storage.
pub type FlowMatGraph<W> = SimpleGraph<W, FlowEdge<W>, FlowMat<W>>;

/// By simple graph we mean a graph without loops and multiple edges.
///
/// Unlike its formal definition which indicates that a simple graph is an unweighted, undirected graph containing no graph loops or multiple edges.\
/// But this can be achieved easily with our `SimpleGraph` too.
///
/// # Generic Parameters:
/// `W`: Weight of the edge.
/// `E`: Edge of the graph.
/// `S`: Storage that graph uses.
pub struct SimpleGraph<W, E: Edge<W>, S: GraphStorage<W, E>> {
    // Backend storage to store graph data
    storage: S,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
}

impl<W: Any + Copy, E: Edge<W>, S: GraphStorage<W, E>> SimpleGraph<W, E, S> {
    /// Initialize the graph with specified `storage`.
    ///
    /// # Arguments:
    /// * `storage`: Storage that graph will use to store its data.
    ///
    /// # Returns:
    /// * Initialized graph.
    pub fn init(storage: S) -> Self {
        SimpleGraph {
            storage,

            phantom_e: PhantomData,
            phantom_w: PhantomData,
        }
    }
}

impl<W, E: Edge<W>, S: GraphStorage<W, E>> provide::Neighbors for SimpleGraph<W, E, S> {
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

impl<W, E: Edge<W>, S: GraphStorage<W, E>> provide::Vertices for SimpleGraph<W, E, S> {
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

impl<W, E: Edge<W>, S: GraphStorage<W, E>> provide::Edges<W, E> for SimpleGraph<W, E, S> {
    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `edge`).
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges(&self, doubles: bool) -> Vec<(usize, usize, &E)> {
        self.storage.edges(doubles)
    }

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `edge`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.storage.edges_from(src_id)
    }
}

impl<W, E: Edge<W>, S: GraphStorage<W, E>> provide::Graph<W, E> for SimpleGraph<W, E, S> {
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
    /// * `edge`: Edge between `src_id` and `dst_id`.
    ///
    /// # Panics:
    /// * If `src_id` == `dst_id`: because it causes a loop.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) {
        if src_id == dst_id {
            panic!("Can not create loop in simple graph")
        }

        self.storage.add_edge(src_id, dst_id, edge);
    }

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The removed edge.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        self.storage.remove_edge(src_id, dst_id)
    }

    /// # Returns:
    /// `true`: If edges stored in the graph are directed, `false` otherwise.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn is_directed(&self) -> bool {
        self.storage.is_directed()
    }
}
