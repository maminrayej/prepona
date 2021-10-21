use super::{
    CheckedFixedSizeMutEdgeDescriptor, CheckedMutEdgeDescriptor, EdgeDescriptor,
    FixedSizeMutEdgeDescriptor, MutEdgeDescriptor,
};
use crate::storage::edge::Direction;
use crate::storage::vertex::VertexToken;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub trait UnorderedSet<T>: PartialEq + Eq + FromIterator<T> + Extend<T> {
    fn contains(&self, value: &T) -> bool;

    fn insert(&mut self, value: T);

    fn remove(&mut self, value: &T);

    fn replace(&mut self, target: &T, value: T);

    fn len(&self) -> usize;

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

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }
}

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
