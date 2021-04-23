use std::any::Any;
use std::marker::PhantomData;

use anyhow::Result;
use provide::{Edges, Graph, Neighbors, Vertices};

use crate::graph::{error::Error, DefaultEdge, Edge, EdgeDir, FlowEdge};
use crate::provide;
use crate::storage::{FlowList, FlowMat, GraphStorage, List, Mat};

/// A `SimpleGraph` that uses [`Mat`](crate::storage::Mat) as its storage.
pub type MatGraph<W, Dir> = SimpleGraph<W, DefaultEdge<W>, Dir, Mat<W, Dir>>;

/// A `SimpleGraph` that uses [`List`](crate::storage::List) as its storage.
pub type ListGraph<W, Dir> = SimpleGraph<W, DefaultEdge<W>, Dir, List<W, Dir>>;

/// A `SimpleGraph` that uses [`FlowMat`](crate::storage::FlowMat) as its storage.
pub type FlowMatGraph<W, Dir> = SimpleGraph<W, FlowEdge<W>, Dir, FlowMat<W>>;

/// A `SimpleGraph` that uses [`FlowList`](crate::storage::FlowList) as its storage.
pub type FlowListGraph<W, Dir> = SimpleGraph<W, DefaultEdge<W>, Dir, FlowList<W, Dir>>;

/// Representing a graph that does not allow loops or multiple edges between two vertices.
///
/// ## Note
/// `SimpleGraph` forwards most of its function calls to its underlying storage. So the complexities of its functions are dependent to what storage you use to initialize the graph.
/// Therefore `SimpleGraph` only documents complexity of functions that it adds some additional logic to. For `SimpleGraph`, only `add_edge` function adds additional logic.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `S`: **S**torage to use: one of the storages defined in [`storage`](crate::storage) module or your custom storage.
pub struct SimpleGraph<W, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> {
    storage: S,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<W: Any, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> SimpleGraph<W, E, Dir, S> {
    /// `SimpleGraph` defines multiple types with different combination of values for generic parameters.
    /// These types are:
    /// * [`MatGraph`](crate::graph::MatGraph): A simple graph using [`Mat`](crate::storage::Mat) as its storage.
    /// * [`FlowMatGraph`](crate::graph::FlowMatGraph): A simple graph using [`FlowMat`](crate::storage::FlowMat) as its storage.
    /// * [`ListGraph`](crate::graph::ListGraph): A simple graph using [`List`](crate::storage::List) as its storage.
    /// * [`FlowListGraph`](crate::graph::FlowListGraph): A simple graph using [`FlowList`](crate::storage::FlowList) as its storage.
    ///
    /// # Arguments
    /// `storage`: Storage to use.
    ///
    /// # Returns
    /// An empty simple graph.
    ///
    /// # Examples
    /// ```
    /// use prepona::prelude::*;
    /// use prepona::storage::{Mat, DiList};
    /// use prepona::graph::{MatGraph, ListGraph};
    ///
    /// // A simple graph that uses a adjacency matrix as its storage with undirected edges of type usize.
    /// let mat_graph = MatGraph::init(Mat::<usize>::init());
    ///
    /// // A simple graph that uses adjacency list as its storage with directed edges of type u32.
    /// let list_graph = ListGraph::init(DiList::<u32>::init());
    /// ```
    pub fn init(storage: S) -> Self {
        SimpleGraph {
            storage,

            phantom_e: PhantomData,
            phantom_w: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

/// For documentation about each function checkout [`Neighbors`](crate::provide::Neighbors) trait and the storage you use.
impl<W, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> Neighbors
    for SimpleGraph<W, E, Dir, S>
{
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        self.storage.neighbors(src_id)
    }

    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        self.storage.neighbors_unchecked(src_id)
    }
}

/// For documentation about each function checkout [`Vertices`](crate::provide::Vertices) trait and the storage you use.
impl<W, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> Vertices
    for SimpleGraph<W, E, Dir, S>
{
    fn vertices(&self) -> Vec<usize> {
        self.storage.vertices()
    }

    fn vertex_count(&self) -> usize {
        self.storage.vertex_count()
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.storage.contains_vertex(vertex_id)
    }
}

/// For documentation about each function checkout [`Edges`](crate::provide::Edges) trait and the storage you use.
impl<W, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> Edges<W, E>
    for SimpleGraph<W, E, Dir, S>
{
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        self.storage.edges_from(src_id)
    }

    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.storage.edges_from_unchecked(src_id)
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        self.storage.edges_between(src_id, dst_id)
    }

    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.storage.edges_between_unchecked(src_id, dst_id)
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        self.storage.edge_between(src_id, dst_id, edge_id)
    }

    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E {
        self.storage.edge_between_unchecked(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> Result<&E> {
        self.storage.edge(edge_id)
    }

    fn edge_unchecked(&self, edge_id: usize) -> &E {
        self.storage.edge_unchecked(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        self.storage.has_any_edge(src_id, dst_id)
    }

    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        self.storage.has_any_edge_unchecked(src_id, dst_id)
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.storage.edges()
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.storage.as_directed_edges()
    }

    fn edges_count(&self) -> usize {
        self.storage.edge_count()
    }

    fn contains_edge(&self, edge_id: usize) -> bool {
        self.storage.contains_edge(edge_id)
    }
}

/// For documentation about each function checkout [`Graph`](crate::provide::Graph) trait.
impl<W, E: Edge<W>, Dir: EdgeDir, S: GraphStorage<W, E, Dir>> Graph<W, E, Dir>
    for SimpleGraph<W, E, Dir, S>
{
    fn add_vertex(&mut self) -> usize {
        self.storage.add_vertex()
    }

    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        self.storage.remove_vertex(vertex_id)
    }

    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        self.storage.remove_vertex_unchecked(vertex_id)
    }

    /// Adds an edge to the graph.
    ///
    /// # Arguments
    /// `src_id`: Id of the source vertex.
    /// `dst_id`: Id of the destination vertex.
    /// `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// * `Err`:
    ///     * If there is already an edge between source and destination.
    ///     * If source and destination are the same(edge is a loop)
    ///     * Error from calling `add_edge` on storage.
    /// * `Ok`: Id of the newly added edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize> {
        if self.has_any_edge(src_id, dst_id)? {
            Err(Error::new_me(src_id, dst_id))?
        } else if src_id == dst_id {
            Err(Error::new_l(src_id))?
        } else {
            self.storage.add_edge(src_id, dst_id, edge)
        }
    }

    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize {
        self.storage.add_edge_unchecked(src_id, dst_id, edge)
    }

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) -> Result<()> {
        self.storage.update_edge(src_id, dst_id, edge_id, edge)
    }

    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) {
        self.storage
            .update_edge_unchecked(src_id, dst_id, edge_id, edge)
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        self.storage.remove_edge(src_id, dst_id, edge_id)
    }

    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E {
        self.storage.remove_edge_unchecked(src_id, dst_id, edge_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provide::*;

    #[test]
    fn add_loop() {
        // Given: An empty graph.
        let mut graph = MatGraph::init(Mat::<usize>::init());

        // When: Adding an edge from a vertex to itself.
        assert!(graph.add_edge(0, 0, 1.into()).is_err());
    }

    #[test]
    fn add_multiple_edge() {
        // Given: Graph
        //
        //      a  --- b
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());

        // When: Trying to add another edge between a and b.
        assert!(graph.add_edge(a, b, 1.into()).is_err());

        // Then: Code should panic.
    }
}
