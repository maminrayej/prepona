use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, Direction, EdgeDescriptor, FixedSizeMutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;

use super::KElementCollection;

#[derive(PartialEq, Eq)]
pub struct KUniformDirHyperedge<VT, C, const K: usize>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    tail: VT,
    heads: C,
}

impl<VT, C, const K: usize> KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    pub fn try_init(tail: VT, heads_iter: impl Iterator<Item = VT>) -> Result<Self> {
        let items = heads_iter.collect::<Vec<VT>>();
        let items_count = items.len();

        match C::try_from(items) {
            Ok(heads) => Ok(KUniformDirHyperedge { tail, heads }),
            _ => Err(StorageError::NotKElement(items_count, K).into()),
        }
    }
}

impl<VT, C, const K: usize> Direction<true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

impl<VT, C, const K: usize> EdgeDescriptor<VT, true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(std::iter::once(&self.tail))
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.heads.iterator())
    }

    fn is_source(&self, vt: &VT) -> bool {
        &self.tail == vt
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.heads.contains_value(vt)
    }

    fn sources_count(&self) -> usize {
        1
    }

    fn destinations_count(&self) -> usize {
        K
    }
}

impl<VT, C, const K: usize> FixedSizeMutEdgeDescriptor<VT, true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    fn replace_src(&mut self, _: &VT, vt: VT) {
        self.tail = vt
    }

    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.heads.replace(dst_vt, vt)
    }
}

impl<VT, C, const K: usize> CheckedFixedSizeMutEdgeDescriptor<VT, true>
    for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}
