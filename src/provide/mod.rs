mod marker;

pub use marker::Graph;
pub trait Neighbors {
    fn neighbors(&self, v_index: usize) -> Vec<usize>;
}
