mod default_edge;
mod flow_edge;

use magnitude::Magnitude;

pub use default_edge::DefaultEdge;
pub use flow_edge::FlowEdge;

/// Defines functionalities to determine wether edges in the graph are directed or not.
pub trait EdgeDir {
    /// # Returns
    /// * `true`: If edge is directed.
    /// * `false`: Otherwise.
    fn is_directed() -> bool;

    /// # Returns
    /// * `true`: If edge is undirected.
    /// * `false`: Otherwise.
    fn is_undirected() -> bool;
}

/// Represents a directed edge type.
pub struct DirectedEdge;

/// For documentation about each function checkout [`EdgeDir`](crate::graph::EdgeDir) trait.
impl EdgeDir for DirectedEdge {
    /// # Complexity
    /// O(1)
    fn is_directed() -> bool {
        true
    }

    /// # Complexity
    /// O(1)
    fn is_undirected() -> bool {
        false
    }
}

/// Represents an undirected edge type.
pub struct UndirectedEdge;

/// For documentation about each function checkout [`EdgeDir`](crate::graph::EdgeDir) trait.
impl EdgeDir for UndirectedEdge {
    /// # Complexity
    /// O(1)
    fn is_directed() -> bool {
        false
    }

    /// # Complexity
    /// O(1)
    fn is_undirected() -> bool {
        true
    }
}

/// Define necessary functionality for a struct to act as an edge of a graph.
///
/// You can define your own custom edges and after implementing `Edge` trait for them, you can store them in any [`storage`](crate::storage).
/// As an example checkout [`FlowEdge`](crate::graph::FlowEdge).
pub trait Edge<W> {
    /// # Arguments
    /// `weight`: Weight of the edge.
    /// 
    /// # Returns
    /// Initialized edge with the specified `weight`.
    fn init(weight: Magnitude<W>) -> Self;

    /// # Returns
    /// The weight of the edge.
    fn get_weight(&self) -> &Magnitude<W>;

    /// # Arguments
    /// `weight`: New weight of the edge.
    fn set_weight(&mut self, weight: Magnitude<W>);

    /// # Arguments
    /// `id`: Unique id of the edge.
    fn set_id(&mut self, id: usize);

    /// # Returns
    /// Id of the edge.
    fn get_id(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directed_edge() {
        assert!(DirectedEdge::is_directed());
        assert!(!DirectedEdge::is_undirected());
    }

    #[test]
    fn test_undirected_edge() {
        assert!(UndirectedEdge::is_undirected());
        assert!(!UndirectedEdge::is_directed());
    }
}
