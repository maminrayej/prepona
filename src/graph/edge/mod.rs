mod default_edge;
mod flow_edge;

use magnitude::Magnitude;
use std::any::Any;

pub use default_edge::DefaultEdge;
pub use flow_edge::FlowEdge;

/// Trait to guarantee a struct can act as edge of a graph.
pub trait AsEdge<W> {
    /// # Returns:
    /// Weight of the edge.
    fn get_weight(&self) -> &Magnitude<W>;

    /// # Updates weight of the edge.
    ///
    /// # Arguments:
    /// * `weight`: New weight.
    fn set_weight(&mut self, weight: Magnitude<W>);
}

/// Enumerate different types of edges supported.
pub enum Edge {
    /// A simple edge with only a weight value.
    DefaultEdge,

    /// An edge with weight, capacity and flow.
    FlowEdge,
}

impl Edge {
    /// Initialize an edge with a weight.
    ///
    /// # Arguments:
    /// `weight`: Weight of the newly created edge.
    ///
    /// # Returns:
    /// * A struct that can act as edge of a graph based on the value of the `Edge` enum.
    pub fn init<W: Any>(&self, weight: Magnitude<W>) -> Box<dyn AsEdge<W>> {
        match self {
            Edge::DefaultEdge => Box::new(DefaultEdge::init(weight)),
            Edge::FlowEdge => Box::new(FlowEdge::init(weight)),
        }
    }
}
