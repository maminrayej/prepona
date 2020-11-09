mod default_edge;
mod flow_edge;

use magnitude::Magnitude;

pub use default_edge::DefaultEdge;
pub use flow_edge::FlowEdge;

pub trait EdgeType {
    fn is_directed() -> bool;
    fn is_undirected() -> bool;
}

pub struct DirectedEdge;
impl EdgeType for DirectedEdge {
    fn is_directed() -> bool {
        true
    }

    fn is_undirected() -> bool {
        false
    }
}

pub struct UndirectedEdge;
impl EdgeType for UndirectedEdge {
    fn is_directed() -> bool {
        false
    }

    fn is_undirected() -> bool {
        true
    }
}

/// Trait to guarantee a struct can act as edge of a graph.
///
/// `W`: Weight of the edge.
pub trait Edge<W> {
    /// Initializes an edge with the given `weight`.
    ///
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// * Initialized edge.
    fn init(src_id: usize, dst_id: usize, weight: Magnitude<W>) -> Self;

    /// # Returns:
    /// Weight of the edge.
    fn get_weight(&self) -> &Magnitude<W>;

    /// # Updates weight of the edge.
    ///
    /// # Arguments:
    /// * `weight`: New weight.
    fn set_weight(&mut self, weight: Magnitude<W>);

    fn get_src_id(&self) -> usize;

    fn get_dst_id(&self) -> usize;
}
