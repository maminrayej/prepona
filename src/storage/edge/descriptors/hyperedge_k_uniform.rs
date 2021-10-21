use super::{CheckedFixedSizeMutEdgeDescriptor, EdgeDescriptor, FixedSizeMutEdgeDescriptor};
use crate::storage::edge::Direction;
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;
use std::convert::TryFrom;
use std::marker::PhantomData;

pub trait KElementCollection<T, const K: usize>: PartialEq + Eq + TryFrom<Vec<T>> {
    fn contains_value(&self, value: &T) -> bool;

    fn replace(&mut self, value: &T, other: T);

    fn len(&self) -> usize;

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
    pub fn try_init(value: impl Iterator<Item = VT>) -> Result<Self> {
        let values = value.collect::<Vec<VT>>();
        let values_count = values.len();

        match C::try_from(values) {
            Ok(collection) => Ok(KUniformHyperedge {
                collection,
                phantom_vt: PhantomData,
            }),

            _ => Err(StorageError::NotKElement(values_count, K).into()),
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

impl<VT, C, const K: usize> FixedSizeMutEdgeDescriptor<VT, false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.collection.replace(src_vt, vt);
    }

    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.replace_src(dst_vt, vt);
    }
}

impl<VT, C, const K: usize> CheckedFixedSizeMutEdgeDescriptor<VT, false>
    for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}
