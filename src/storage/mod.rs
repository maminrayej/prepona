mod adj_matrix;

pub use adj_matrix::AdjMatrix;

use magnitude::Magnitude;
use std::any::Any;

pub enum Storage {
    AdjMatrix,
}

impl Storage {
    pub fn init_storage<W: Any>(&self) -> impl GraphStorage<W> {
        match self {
            Storage::AdjMatrix => AdjMatrix::init(),
        }
    }
}

pub trait GraphStorage<W> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize, edge_weight: Magnitude<W>);

    fn remove_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize) -> Magnitude<W>;

    fn vertex_count(&self) -> usize;
}
