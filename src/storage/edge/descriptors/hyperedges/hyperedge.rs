use super::UnorderedSet;
use crate::storage::edge::descriptors::FixedSizeMutEdgeDescriptor;
use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, CheckedMutEdgeDescriptor, Direction, EdgeDescriptor,
    MutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use std::collections::HashSet;
use std::marker::PhantomData;

pub type HashHyperedge<VT> = Hyperedge<VT, HashSet<VT>>;

#[derive(PartialEq, Eq)]
pub struct Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    vertex_set: Set,

    phantom_vt: PhantomData<VT>,
}

impl<VT, Set> Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    pub fn init(vertex_token: VT) -> Self {
        Hyperedge {
            vertex_set: Set::from_iter(std::iter::once(vertex_token)),
            phantom_vt: PhantomData,
        }
    }

    pub fn init_multiple(vertex_tokens: impl IntoIterator<Item = VT>) -> Self {
        Hyperedge {
            vertex_set: Set::from_iter(vertex_tokens),
            phantom_vt: PhantomData,
        }
    }
}

impl<VT, Set> Direction<false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

impl<VT, Set> EdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertex_set.iterator())
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertex_set.iterator())
    }

    fn is_source(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    fn contains(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    fn sources_count(&self) -> usize {
        self.vertex_set.len()
    }

    fn destinations_count(&self) -> usize {
        self.vertex_set.len()
    }
}

impl<VT, Set> FixedSizeMutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.vertex_set.replace(src_vt, vt);
    }

    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.replace_src(dst_vt, vt);
    }
}

impl<VT, Set> CheckedFixedSizeMutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

impl<VT, Set> MutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn add(&mut self, src_vt: VT, dst_vt: VT) {
        self.vertex_set
            .extend(std::iter::once(src_vt).chain(Some(dst_vt)));
    }

    fn remove(&mut self, vt: VT) {
        self.vertex_set.remove(&vt)
    }
}

impl<VT, Set> CheckedMutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}
