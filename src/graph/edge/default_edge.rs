use magnitude::Magnitude;
use quickcheck::Arbitrary;

use crate::graph::edge::Edge;

/// Represent a default edge with only weight.
#[derive(Copy, Clone)]
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

use std::convert::From;
use std::{any::Any, fmt::Debug};
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

impl<W: Arbitrary> Arbitrary for DefaultEdge<W> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        DefaultEdge::init(W::arbitrary(g).into())
    }
}

impl<W: Debug> Debug for DefaultEdge<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({},{:?})", self.get_id(), self.get_weight()))
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
