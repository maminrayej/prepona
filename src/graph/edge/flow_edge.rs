use magnitude::Magnitude;

use crate::graph::edge::Edge;

/// Represent a flow edge with weight, flow and capacity.
#[derive(Debug, Copy, Clone)]
pub struct FlowEdge<W> {
    id: usize,
    weight: Magnitude<W>,
    capacity: usize,
    flow: isize,
}

impl<W> FlowEdge<W> {
    /// # Arguments
    /// * `weight`: Weight of the edge.
    /// * `capacity`: Capacity of the edge.
    /// * `flow`: Flow of the edge.
    ///
    /// # Returns
    /// Initialized edge with specified `weight`, `capacity` and `flow`.
    ///
    /// # Panics
    /// If `flow` > `capacity`.
    pub fn init_with(weight: Magnitude<W>, capacity: usize, flow: isize) -> Self {
        if flow > capacity as isize {
            panic!(
                "Flow of the edge can not be greater than the capacity: {} > {}",
                flow, capacity
            );
        }

        FlowEdge {
            id: 0,
            weight,
            capacity,
            flow,
        }
    }

    /// # Returns
    /// Flow of the edge.
    pub fn get_flow(&self) -> isize {
        self.flow
    }

    /// # Returns
    /// Capacity of the edge.
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /// # Arguments
    /// `flow`: New flow of the edge.
    ///
    /// # Panics
    /// If `flow` > *current capacity*.
    pub fn set_flow(&mut self, flow: isize) {
        if flow > self.get_capacity() as isize {
            panic!("Flow of the edge can not be greater than the current capacity of the edge: {} > {}", flow, self.get_capacity());
        }

        self.flow = flow;
    }

    /// # Arguments
    /// `capacity`: New capacity of the edge.
    ///
    /// # Panics
    /// If `capacity` < *current flow*
    pub fn set_capacity(&mut self, capacity: usize) {
        if (capacity as isize) < self.get_flow() {
            panic!("Capacity of the edge can not be smaller than the current flow of the edge: {} < {}", capacity, self.get_flow());
        }
        self.capacity = capacity
    }
}

/// For documentation about each function checkout [`Edge`](crate::graph::Edge) trait.
impl<W> Edge<W> for FlowEdge<W> {
    fn init(weight: Magnitude<W>) -> Self {
        FlowEdge::init_with(weight, 0, 0)
    }

    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

use std::any::Any;
use std::convert::{From, TryFrom};
impl<W: Any> From<W> for FlowEdge<W> {
    /// Constructs a `FlowEdge` with specified `weight` and flow and capacity of 0.
    fn from(weight: W) -> Self {
        FlowEdge::init(weight.into())
    }
}

impl<W: Any> TryFrom<(W, usize, isize)> for FlowEdge<W> {
    type Error = String;

    /// # Returns
    /// * `Ok`: Containing a `FlowEdge` with specified `weight`, `capacity` and `flow`.
    /// * `Err`: If `flow` > `capacity`.
    fn try_from((weight, capacity, flow): (W, usize, isize)) -> Result<Self, Self::Error> {
        if flow > capacity as isize {
            Err(format!(
                "Flow of the edge can not be greater than the capacity: {} > {}",
                flow, capacity
            ))
        } else {
            Ok(FlowEdge::init_with(weight.into(), capacity, flow))
        }
    }
}

impl<W: PartialEq> PartialEq for FlowEdge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
            && self.id == other.id
            && self.flow == other.flow
            && self.capacity == other.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn init() {
        let edge = FlowEdge::init(2.into());

        assert_eq!(edge.get_weight(), &2.into());
        assert_eq!(edge.get_capacity(), 0);
        assert_eq!(edge.get_flow(), 0);
    }

    #[test]
    fn init_with() {
        let edge = FlowEdge::init_with(2.into(), 4, 3);

        assert_eq!(edge.get_weight(), &2.into());
        assert_eq!(edge.get_capacity(), 4);
        assert_eq!(edge.get_flow(), 3);
    }

    #[test]
    fn set_weight() {
        let mut edge = FlowEdge::init(2.into());

        edge.set_weight(3.into());

        assert_eq!(edge.get_weight(), &3.into());
    }

    #[test]
    fn set_capacity() {
        let mut edge = FlowEdge::init(2.into());

        edge.set_capacity(5);

        assert_eq!(edge.get_capacity(), 5);
    }

    #[test]
    fn set_flow() {
        let mut edge = FlowEdge::init(2.into());
        edge.set_capacity(5);

        edge.set_flow(4);

        assert_eq!(edge.get_flow(), 4);
    }

    #[test]
    fn from_triplet() {
        let edge: FlowEdge<usize> = 2.into();

        assert_eq!(edge.get_weight(), &2.into());
        assert_eq!(edge.get_capacity(), 0);
        assert_eq!(edge.get_flow(), 0);
    }

    #[test]
    fn from_quintuplet() {
        let edge: FlowEdge<usize> = (2, 4, 3).try_into().unwrap();

        assert_eq!(edge.get_weight(), &2.into());
        assert_eq!(edge.get_capacity(), 4);
        assert_eq!(edge.get_flow(), 3);
    }

    #[test]
    #[should_panic(expected = "Flow of the edge can not be greater than the capacity: 4 > 3")]
    fn init_with_flow_larger_than_capacity() {
        let _ = FlowEdge::init_with(2.into(), 3, 4);
    }

    #[test]
    #[should_panic(
        expected = "Flow of the edge can not be greater than the current capacity of the edge: 4 > 0"
    )]
    fn set_flow_larger_than_capacity() {
        let mut edge = FlowEdge::init(2.into());

        edge.set_flow(4);
    }

    #[test]
    #[should_panic(
        expected = "Capacity of the edge can not be smaller than the current flow of the edge: 0 < 4"
    )]
    fn set_capacity_smaller_than_flow() {
        let mut edge = FlowEdge::init_with(2.into(), 5, 4);

        edge.set_capacity(0);
    }

    #[test]
    fn from_quintuplet_with_flow_larger_than_capacity() {
        let edge_res: Result<FlowEdge<usize>, String> = (2, 0, 4).try_into();

        assert!(edge_res.is_err());
        assert_eq!(
            edge_res.unwrap_err(),
            "Flow of the edge can not be greater than the capacity: 4 > 0".to_string()
        )
    }
}
