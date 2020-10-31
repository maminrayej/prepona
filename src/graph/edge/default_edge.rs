use magnitude::Magnitude;

use crate::graph::edge::Edge;

pub struct DefaultEdge<W> {
    weight: Magnitude<W>,
}

impl<W> DefaultEdge<W> {
    pub fn init(weight: Magnitude<W>) -> Self {
        DefaultEdge { weight }
    }
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
