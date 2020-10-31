use magnitude::Magnitude;

use crate::graph::edge::Edge;

pub struct FlowEdge<W> {
    weight: Magnitude<W>,
    flow: isize,
    capacity: usize,
}

impl<W> FlowEdge<W> {
    pub fn init(weight: Magnitude<W>) -> Self {
        FlowEdge {
            weight,
            flow: 0,
            capacity: 0,
        }
    }

    pub fn get_flow(&self) -> isize {
        self.flow
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn set_flow(&mut self, flow: isize) {
        self.flow = flow;
    }

    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity
    }
}

impl<W> Edge<W> for FlowEdge<W> {
    fn init(weight: Magnitude<W>) -> Self {
        FlowEdge {
            weight,
            flow: 0,
            capacity: 0
        }
    }

    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }
}
