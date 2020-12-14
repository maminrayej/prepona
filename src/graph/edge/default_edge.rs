use magnitude::Magnitude;

use crate::graph::edge::Edge;

/// Represent a default edge with only weight.
#[derive(Debug, Copy, Clone)]
pub struct DefaultEdge<W> {
    id: usize,
    weight: Magnitude<W>,
}

/// For documentation about each function checkout [`Edge`](crate::graph::Edge) trait.
impl<W> Edge<W> for DefaultEdge<W> {
    fn init(weight: Magnitude<W>) -> Self {
        DefaultEdge { id: 0, weight }
    }

    /// # Complexity
    /// O(1)
    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    /// # Complexity
    /// O(1)
    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }

    /// # Complexity
    /// O(1)
    fn set_id(&mut self, id: usize) {
        self.id = id
    }

    /// # Complexity
    /// O(1)
    fn get_id(&self) -> usize {
        self.id
    }
}

use std::any::Any;
use std::convert::From;
impl<W: Any> From<W> for DefaultEdge<W> {
    /// Construct a `DefaultEdge` from `Any` value.
    fn from(weight: W) -> Self {
        DefaultEdge::init(weight.into())
    }
}

impl<W: PartialEq> PartialEq for DefaultEdge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let edge = DefaultEdge::init(2.into());

        assert_eq!(edge.get_weight(), &2.into());
    }

    #[test]
    fn set_weight() {
        let mut edge = DefaultEdge::init(2.into());

        edge.set_weight(3.into());

        assert_eq!(edge.get_weight(), &3.into());
    }

    #[test]
    fn from_triplet() {
        let edge: DefaultEdge<usize> = 2.into();

        assert_eq!(edge.get_weight(), &2.into());
    }
}
