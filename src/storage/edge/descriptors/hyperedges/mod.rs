mod hyperedge;
mod hyperedge_dir;
mod uniform;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub use hyperedge::*;
pub use hyperedge_dir::*;
pub use uniform::*;

pub trait UnorderedSet<T>: PartialEq + Eq + FromIterator<T> + Extend<T> {
    fn contains(&self, value: &T) -> bool;

    fn insert(&mut self, value: T);

    fn remove(&mut self, value: &T);

    fn replace(&mut self, target: &T, value: T);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T> UnorderedSet<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn contains(&self, value: &T) -> bool {
        self.contains(value)
    }

    fn insert(&mut self, value: T) {
        self.insert(value);
    }

    fn remove(&mut self, value: &T) {
        self.remove(value);
    }

    fn replace(&mut self, target: &T, value: T) {
        self.remove(target);
        self.insert(value);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}
