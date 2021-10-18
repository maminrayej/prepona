use crate::storage::edge::Direction;
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;
use std::convert::TryFrom;
use std::marker::PhantomData;

use super::EdgeDescriptor;

pub trait KElementCollection<T, const K: usize>: PartialEq + Eq + TryFrom<Vec<T>> {
    fn contains_value(&self, item: &T) -> bool;

    fn replace(&mut self, item: &T, other: T);

    fn len(&self) -> usize;

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T: PartialEq + Eq, const K: usize> KElementCollection<T, K> for [T; K] {
    fn contains_value(&self, item: &T) -> bool {
        self.contains(item)
    }

    fn replace(&mut self, item: &T, other: T) {
        if let Some(index) = self.iter().position(|v| v == item) {
            self[index] = other;
        }
    }

    fn len(&self) -> usize {
        K
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

#[derive(PartialEq, Eq)]
pub struct KUniformHyperedge<VT, C, const K: usize>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    collection: C,

    phantom_vt: PhantomData<VT>,
}

impl<VT, C, const K: usize> KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    pub fn try_init(item: impl Iterator<Item = VT>) -> Result<Self> {
        let items = item.collect::<Vec<VT>>();
        let items_count = items.len();

        if let Ok(collection) = C::try_from(items) {
            Ok(KUniformHyperedge {
                collection,
                phantom_vt: PhantomData,
            })
        } else {
            Err(StorageError::NotKElement(items_count, K).into())
        }
    }
}

impl<VT, C, const K: usize> Direction<false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

impl<VT, C, const K: usize> EdgeDescriptor<VT, false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.collection.iterator())
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        self.get_sources()
    }

    fn is_source(&self, vt: &VT) -> bool {
        self.collection.contains_value(vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.is_source(vt)
    }

    fn contains(&self, vt: &VT) -> bool {
        self.is_source(vt)
    }

    fn sources_count(&self) -> usize {
        self.collection.len()
    }

    fn destinations_count(&self) -> usize {
        self.sources_count()
    }
}
