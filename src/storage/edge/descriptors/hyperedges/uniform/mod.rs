mod hyperedge_dir_k_uniform;
mod hyperedge_k_uniform;

use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

pub use hyperedge_dir_k_uniform::*;
pub use hyperedge_k_uniform::*;

/// Describes a collection that holds exactly `K` elements at all times.
///
/// # Generic parameters
/// * `T`: Type of value to be stored.
/// * `K`: Number of elements in the collection.
///
/// # Required traits
/// * `PartialEq`, `Eq`: A collection of `K` elements must be comparable to another collection of the same size and type.
/// * `TryFrom`: A collection of `K` elements must be capable of being initialized from a `Vec` with `K` elements.
/// * `Index`: A collection must be indexable in order to retrieve elements from it.
/// * `IndexMut`: A collection must mutably indexable in order to update elements in the collection using an index.
pub trait KElementCollection<T, const K: usize>:
    PartialEq + Eq + TryFrom<Vec<T>> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    /// # Argument
    /// `value`: Value to search for in the collection.
    ///
    /// # Returns
    /// * `true`: If collection contains `value`.
    fn contains_value(&self, value: &T) -> bool;

    /// If `target` does not exist in the collection, nothing must happen.
    ///
    /// # Arguments
    /// * `target`: Target value to be replaced with `value`.
    /// * `value`: New value to replace the `target`.
    fn replace(&mut self, target: &T, value: T);

    /// # Returns
    /// Number of values stored in the collection.
    fn len(&self) -> usize;

    /// # Returns
    /// * `true`: If collection does not contain any value.
    /// * `false`: Otherwise.
    fn is_empty(&self) -> bool;

    /// # Returns
    /// An iterator over the values stored in the collection.
    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T: PartialEq + Eq, const K: usize> KElementCollection<T, K> for [T; K] {
    fn contains_value(&self, value: &T) -> bool {
        self.contains(value)
    }

    fn replace(&mut self, target: &T, value: T) {
        if let Some(index) = self.iter().position(|v| v == target) {
            self[index] = value;
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
