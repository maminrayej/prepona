mod hyperedge;
mod hyperedge_dir;
mod uniform;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub use hyperedge::*;
pub use hyperedge_dir::*;
pub use uniform::*;

/// Describes an unordered set container.
///
/// # Generic parameters
/// `T`: Type of value to be stored.
///
/// # Required traits
/// * `PartialEq`, `Eq`: An unordered set must be comparable to another unordered set with the same type.
/// * `FromIterator`: An unordered set must be capable to be initialized from an iterator of values.
/// * `Extend`: An unordered set must be extendable given an iterator of values.
pub trait UnorderedSet<T>: PartialEq + Eq + FromIterator<T> + Extend<T> {
    /// # Arguments
    /// `value`: Value to search for in the unordered set.
    ///
    /// # Returns
    /// * `true`: If unordered set contains the `value`.
    /// * `false`: Otherwise.
    fn contains(&self, value: &T) -> bool;

    /// If `value` already exists in the unordered set, nothing must happen.
    ///
    /// # Arguments
    /// `value`: Value to be inserted into the unordered set.
    fn insert(&mut self, value: T);

    /// If the value does not exist in the unordered set, nothing must happen.
    ///
    /// # Arguments
    /// `value`: Value to be removed from the unordered set.
    fn remove(&mut self, value: &T);

    /// If `target` does not exist in the unordered set, nothing must happen.
    ///
    /// # Arguments
    /// * `target`: Target value to be replaced with `value`.
    /// * `value`: New value to replace the `target`.
    fn replace(&mut self, target: &T, value: T);

    /// # Returns
    /// Number of values stored in the unordered set.
    fn len(&self) -> usize;

    /// # Returns
    /// * `true`: If unordered set does not contain any value.
    /// * `false`: Otherwise.
    fn is_empty(&self) -> bool;

    /// # Returns
    /// An iterator over the values stored in the unordered set.
    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T> UnorderedSet<T> for HashSet<T>
where
    T: Hash + Eq,
{
    /// # Complexity
    /// O([`HashSet::contains`])
    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }

    /// # Complexity
    /// O([`HashSet::insert`])
    fn insert(&mut self, value: T) {
        self.insert(value);
    }

    /// # Complexity
    /// O([`HashSet::remove`])
    fn remove(&mut self, value: &T) {
        self.remove(value);
    }

    /// # Complexity
    /// O([`HashSet::remove`] + [`HashSet::insert`])
    fn replace(&mut self, target: &T, value: T) {
        if self.remove(target) {
            self.insert(value);
        }
    }

    /// # Complexity
    /// O([`HashSet::len`])
    fn len(&self) -> usize {
        self.len()
    }

    /// # Complexity
    /// O([`HashSet::is_empty`])
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    /// # Complexity
    /// O([`HashSet::iter`])
    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}
