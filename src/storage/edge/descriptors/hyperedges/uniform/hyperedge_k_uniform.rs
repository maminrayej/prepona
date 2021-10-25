use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, Direction, EdgeDescriptor, FixedSizeMutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;
use std::marker::PhantomData;

use super::KElementCollection;

/// A [`KUniformHyperedge`] that uses an array of size `K` as its [`KElementCollection`].
pub type ArrKUniformHyperedge<VT, const K: usize> = KUniformHyperedge<VT, [VT; K], K>;

/// An edge that can connect exactly `K` vertices.
///
/// A `KUniformHyperedge` is a non-empty subset of exactly `K` vertices. All vertices participating in a hyperedge are connected together(for further reading see [here]).
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
/// * `C`: A collection that contains `K` elements at all times.
/// * `K`: Number of connected vertices.
///
/// [here]: https://en.wikipedia.org/wiki/Hypergraph
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
    /// # Arguments
    /// `values`: An iterator that contains exactly `K` elements.
    ///
    /// # Returns
    /// `Ok`: Containing the constructed `KUniformHyperedge` if `values` contains exactly `K` elements.
    /// `Err`: Specifically [`StorageError::NotKElement`] error if `values` does not contain exactly `K` elements.
    pub fn try_init(values: impl Iterator<Item = VT>) -> Result<Self> {
        let values = values.collect::<Vec<VT>>();
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
