use magnitude::Magnitude;

use crate::graph::edge::Edge;

/// Represents an edge containing weight, capacity and flow which makes it suitable for flow computation.
///
/// # Generic Parameters:
/// `W`: Weight of the edge.
pub struct FlowEdge<W> {
    src_id: usize,
    dst_id: usize,
    weight: Magnitude<W>,
    capacity: usize,
    flow: isize,
}

impl<W> FlowEdge<W> {
    /// Initializes a flow edge with the given `weight`, `capacity` and `flow`.
    ///
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    /// * `capacity`: Capacity of the edge.
    /// * `flow`: Flow of the edge.
    ///
    /// # Panics:
    /// If `flow` is greater than `capacity`.
    ///
    /// # Returns:
    /// * Initialized edge.
    pub fn init_with(
        src_id: usize,
        dst_id: usize,
        weight: Magnitude<W>,
        capacity: usize,
        flow: isize,
    ) -> Self {
        if flow > capacity as isize {
            panic!(
                "Flow of the edge can not be greater than the capacity: {} > {}",
                flow, capacity
            );
        }

        FlowEdge {
            src_id,
            dst_id,
            weight,
            capacity,
            flow,
        }
    }

    /// # Returns:
    /// Flow of the edge.
    pub fn get_flow(&self) -> isize {
        self.flow
    }

    /// # Returns:
    /// Capacity of the edge.
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /// Updates the flow of the edge.
    ///
    /// # Arguments:
    /// * `flow`: New flow of the edge.
    ///
    /// # Panics:
    /// If `flow` is greater than the current capacity of the edge.
    pub fn set_flow(&mut self, flow: isize) {
        if flow > self.get_capacity() as isize {
            panic!("Flow of the edge can not be greater than the current capacity of the edge: {} > {}", flow, self.get_capacity());
        }

        self.flow = flow;
    }

    /// Updates the capacity of the edge.
    ///
    /// # Arguments:
    /// * `capacity`: New capacity of the edge.
    ///
    /// # Panics:
    /// If `capacity` is smaller than the current flow of the edge.
    pub fn set_capacity(&mut self, capacity: usize) {
        if (capacity as isize) < self.get_flow() {
            panic!("Capacity of the edge can not be smaller than the current flow of the edge: {} < {}", capacity, self.get_flow());
        }
        self.capacity = capacity
    }
}

impl<W> Edge<W> for FlowEdge<W> {
    /// Initializes a flow edge with the given `weight` and flow and capacity of 0.
    ///
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// * Initialized edge.
    fn init(src_id: usize, dst_id: usize, weight: Magnitude<W>) -> Self {
        FlowEdge::init_with(src_id, dst_id, weight, 0, 0)
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

    fn get_src_id(&self) -> usize {
        self.src_id
    }

    fn get_dst_id(&self) -> usize {
        self.dst_id
    }
}

use std::any::Any;
use std::convert::{From, TryFrom};
/// Construct flow edge with capacity and flow of 0 and the specified `weight`.
impl<W: Any> From<(usize, usize, W)> for FlowEdge<W> {
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// Initialized flow edge.
    fn from((src_id, dst_id, weight): (usize, usize, W)) -> Self {
        FlowEdge::init(src_id, dst_id, weight.into())
    }
}

/// Construct flow edge with specified `weight`, `capacity` and `flow`.
impl<W: Any> TryFrom<(usize, usize, W, usize, isize)> for FlowEdge<W> {
    type Error = String;

    /// # Arguments:
    /// * `weight`: Weight of the edge.
    /// * `capacity`: Capacity of the edge.
    /// * `flow`: Flow of the edge.
    ///
    /// # Returns
    /// * Ok: If `flow` <= `capacity`.
    /// * Err: If `flow` > `capacity`.
    fn try_from(
        (src_id, dst_id, weight, capacity, flow): (usize, usize, W, usize, isize),
    ) -> Result<Self, Self::Error> {
        if flow > capacity as isize {
            Err(format!(
                "Flow of the edge can not be greater than the capacity: {} > {}",
                flow, capacity
            ))
        } else {
            Ok(FlowEdge::init_with(
                src_id,
                dst_id,
                weight.into(),
                capacity,
                flow,
            ))
        }
    }
}
