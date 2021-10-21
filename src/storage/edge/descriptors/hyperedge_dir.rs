use super::{
    CheckedFixedSizeMutEdgeDescriptor, CheckedMutEdgeDescriptor, EdgeDescriptor,
    FixedSizeMutEdgeDescriptor, MutEdgeDescriptor, UnorderedSet,
};
use crate::storage::{edge::Direction, vertex::VertexToken};
use std::marker::PhantomData;

#[derive(PartialEq, Eq)]
pub struct DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    source_set: Set,
    destination_set: Set,

    phantom_vt: PhantomData<VT>,
}

impl<VT, Set> DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    pub fn init(src_vt: VT, dst_vt: VT) -> Self {
        DirHyperedge {
            source_set: Set::from_iter(std::iter::once(src_vt)),
            destination_set: Set::from_iter(std::iter::once(dst_vt)),
            phantom_vt: PhantomData,
        }
    }

    pub fn init_multiple(
        src_vts: impl IntoIterator<Item = VT>,
        dst_vts: impl IntoIterator<Item = VT>,
    ) -> Self {
        DirHyperedge {
            source_set: Set::from_iter(src_vts),
            destination_set: Set::from_iter(dst_vts),
            phantom_vt: PhantomData,
        }
    }
}

impl<VT, Set> Direction<true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

impl<VT, Set> EdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.source_set.iterator())
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.destination_set.iterator())
    }

    fn is_source(&self, vt: &VT) -> bool {
        self.source_set.contains(vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.destination_set.contains(vt)
    }

    fn sources_count(&self) -> usize {
        self.source_set.len()
    }

    fn destinations_count(&self) -> usize {
        self.destination_set.len()
    }
}

impl<VT, Set> FixedSizeMutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.source_set.replace(src_vt, vt);
    }

    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.destination_set.replace(dst_vt, vt);
    }
}

impl<VT, Set> CheckedFixedSizeMutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

impl<VT, Set> MutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn add(&mut self, src_vt: VT, dst_vt: VT) {
        self.source_set.extend(std::iter::once(src_vt));
        self.destination_set.extend(std::iter::once(dst_vt));
    }

    fn remove(&mut self, vt: VT) {
        self.source_set.remove(&vt);
        self.destination_set.remove(&vt)
    }
}

impl<VT, Set> CheckedMutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}
