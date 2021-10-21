mod hyperedge_dir_k_uniform;
mod hyperedge_k_uniform;

use std::convert::TryFrom;

pub use hyperedge_dir_k_uniform::*;
pub use hyperedge_k_uniform::*;

pub trait KElementCollection<T, const K: usize>: PartialEq + Eq + TryFrom<Vec<T>> {
    fn contains_value(&self, value: &T) -> bool;

    fn replace(&mut self, value: &T, other: T);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T: PartialEq + Eq, const K: usize> KElementCollection<T, K> for [T; K] {
    fn contains_value(&self, value: &T) -> bool {
        self.contains(value)
    }

    fn replace(&mut self, value: &T, other: T) {
        if let Some(index) = self.iter().position(|v| v == value) {
            self[index] = other;
        }
    }

    fn len(&self) -> usize {
        K
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}
