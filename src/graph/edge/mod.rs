mod default_edge;
mod flow_edge;

use magnitude::Magnitude;

pub use default_edge::DefaultEdge;
pub use flow_edge::FlowEdge;

/// Trait to guarantee a struct can act as edge of a graph.
pub trait Edge<W> {
    fn init(weight: Magnitude<W>) -> Self;

    /// # Returns:
    /// Weight of the edge.
    fn get_weight(&self) -> &Magnitude<W>;

    /// # Updates weight of the edge.
    ///
    /// # Arguments:
    /// * `weight`: New weight.
    fn set_weight(&mut self, weight: Magnitude<W>);
}
