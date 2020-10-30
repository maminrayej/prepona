use magnitude::Magnitude;

use crate::graph::edge::AsEdge;

pub struct DefaultEdge<W> {
    weight: Magnitude<W>,
}

impl<W> DefaultEdge<W> {
    pub fn init(weight: Magnitude<W>) -> Self {
        DefaultEdge { weight }
    }
}

impl<W> AsEdge<W> for DefaultEdge<W> {
    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }
}
