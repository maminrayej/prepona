use magnitude::Magnitude;

use crate::graph::edge::Edge;

#[derive(Debug, Copy, Clone)]
pub struct DefaultEdge<W> {
    weight: Magnitude<W>,
}

impl<W> Edge<W> for DefaultEdge<W> {
    fn init(weight: Magnitude<W>) -> Self {
        DefaultEdge { weight }
    }

    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }
}

use std::any::Any;
use std::convert::From;
impl<W: Any> From<W> for DefaultEdge<W> {
    fn from(weight: W) -> Self {
        DefaultEdge::init(weight.into())
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
