mod adj_matrix;

pub use adj_matrix::AdjMatrix;

use magnitude::Magnitude;
use std::any::Any;

use crate::graph::EdgeType;

pub enum Storage {
    AdjMatrix,
}

impl Storage {
    pub fn init<W: Any + Copy>(&self, edge_type: EdgeType) -> Box<dyn GraphStorage<W>> {
        Box::new(match self {
            Storage::AdjMatrix => AdjMatrix::<W>::init(edge_type),
        })
    }
}

pub trait GraphStorage<W> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize, edge_weight: Magnitude<W>);

    fn remove_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize) -> Magnitude<W>;

    fn vertex_count(&self) -> usize;

    fn vertices(&self) -> Vec<usize>;

    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)>;

    fn edges_from(&self, src_index: usize) -> Vec<(usize, Magnitude<W>)>;

    fn neighbors(&self, src_index: usize) -> Vec<usize>;
}
