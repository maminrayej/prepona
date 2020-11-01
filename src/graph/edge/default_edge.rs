use magnitude::Magnitude;

use crate::graph::edge::Edge;

/// Represents an edge with a weight.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
pub struct DefaultEdge<W> {
    weight: Magnitude<W>,
}

impl<W> Edge<W> for DefaultEdge<W> {
    /// Initializes a default edge with the given `weight`.
    ///
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// * Initialized edge.
    fn init(weight: Magnitude<W>) -> Self {
        DefaultEdge { weight }
    }

    /// # Returns:
    /// Weight of the edge.
    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    /// # Updates weight of the edge.
    ///
    /// # Arguments:
    /// * `weight`: New weight.
    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }
}

use std::any::Any;
use std::convert::From;
/// Construct a default edge from any value.
impl<W: Any> From<W> for DefaultEdge<W> {
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// An initialized default edge with the specified `weight`.
    fn from(weight: W) -> Self {
        DefaultEdge::init(weight.into())
    }
}
