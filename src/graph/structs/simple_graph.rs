use std::any::Any;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, Edge, EdgeType, FlowEdge};
use crate::provide;
use crate::storage::{FlowMat, GraphStorage, Mat};

/// A `SimpleGraph` with a `DefaultEdge` that uses `AdjMatrix` as its storage.
pub type MatGraph<W, Ty> = SimpleGraph<W, DefaultEdge<W>, Ty, Mat<W, Ty>>;
// pub type DiMatGraph<W> = SimpleGraph<W, DefaultEdge<W>, DirectedEdge, DiMat<W>>;

/// A `SimpleGraph` with a `FlowEdge` that uses `AdjMatrix` as its storage.
pub type FlowMatGraph<W, Ty> = SimpleGraph<W, FlowEdge<W>, Ty, FlowMat<W>>;
// pub type DiFlowMatGraph<W> = SimpleGraph<W, FlowEdge<W>, DirectedEdge, DiFlowMat<W>>;

/// By simple graph we mean a graph without loops and multiple edges.
///
/// Unlike its formal definition which indicates that a simple graph is an unweighted, undirected graph containing no graph loops or multiple edges.\
/// But this can be achieved easily with our `SimpleGraph` too.
///
/// # Generic Parameters:
/// `W`: Weight of the edge.
/// `E`: Edge of the graph.
/// `S`: Storage that graph uses.
pub struct SimpleGraph<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> {
    // Backend storage to store graph data
    storage: S,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_ty: PhantomData<Ty>,
}

impl<W: Any + Copy, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> SimpleGraph<W, E, Ty, S> {
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
            phantom_ty: PhantomData,
        }
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Neighbors
    for SimpleGraph<W, E, Ty, S>
{
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

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Vertices
    for SimpleGraph<W, E, Ty, S>
{
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

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Edges<W, E>
    for SimpleGraph<W, E, Ty, S>
{
    fn edge(&self, src_id: usize, dst_id: usize) -> Option<&E> {
        self.storage.edge(src_id, dst_id)
    }

    fn has_edge(&self, src_id: usize, dst_id: usize) -> bool {
        self.storage.has_edge(src_id, dst_id)
    }

    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `edge`).
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges(&self) -> Vec<&E> {
        self.storage.edges()
    }

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `edge`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Complexity:
    /// Depends on the storage type.
    fn edges_from(&self, src_id: usize) -> Vec<&E> {
        self.storage.edges_from(src_id)
    }
}

impl<W, E: Edge<W>, Ty: EdgeType, S: GraphStorage<W, E, Ty>> provide::Graph<W, E, Ty>
    for SimpleGraph<W, E, Ty, S>
{
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
    fn add_edge(&mut self, edge: E) {
        if edge.get_src_id() == edge.get_dst_id() {
            panic!("Can not create loop in simple graph")
        }

        if self.storage.has_edge(edge.get_src_id(), edge.get_dst_id()) {
            panic!("Can not add multiple edges between two vertices in simple graph");
        }

        self.storage.add_edge(edge);
    }

    fn update_edge(&mut self, edge: E) {
        self.storage.update_edge(edge);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provide::*;

    #[test]
    #[should_panic(expected = "Can not create loop in simple graph")]
    fn add_loop() {
        // Given: An empty graph.
        let mut graph = MatGraph::init(Mat::<usize>::init());

        // When: Adding an edge from a vertex to itself.
        graph.add_edge((0, 0, 1).into());

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Can not add multiple edges between two vertices in simple graph")]
    fn add_multiple_edge() {
        // Given: Graph
        //
        //      a  --- b
        // 
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge((a,b,1).into());

        // When: Trying to add another edge between a and b.
        graph.add_edge((a,b,1).into());

        // Then: Code should panic.
    }
}