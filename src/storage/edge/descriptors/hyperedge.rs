use super::{CheckedMutEdgeDescriptor, EdgeDescriptor, MutEdgeDescriptor};
use crate::storage::edge::direction::UndirectedEdge;
use crate::storage::vertex::VertexToken;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub trait UnorderedSet<T>: PartialEq + Eq + FromIterator<T> + Extend<T> {
    fn contains(&self, item: &T) -> bool;

    fn insert(&mut self, item: T);

    fn remove(&mut self, item: &T);

    fn len(&self) -> usize;

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}

impl<T> UnorderedSet<T> for HashSet<T>
where
    T: Hash + Eq,
{
    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.iter())
    }

    fn insert(&mut self, item: T) {
        self.insert(item);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, item: &T) {
        self.remove(item);
    }
}

pub type HashHyperedge<VT> = Hyperedge<VT, HashSet<VT>>;

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

impl<VT, Set> PartialEq for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn eq(&self, other: &Self) -> bool {
        self.vertex_set == other.vertex_set && self.phantom_vt == other.phantom_vt
    }
}

impl<VT, Set> Eq for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

impl<VT, Set> EdgeDescriptor<UndirectedEdge, VT> for Hyperedge<VT, Set>
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

    fn is_source(&self, vertex_token: &VT) -> bool {
        self.vertex_set.contains(vertex_token)
    }

    fn is_destination(&self, vertex_token: &VT) -> bool {
        self.vertex_set.contains(vertex_token)
    }

    fn sources_count(&self) -> usize {
        self.vertex_set.len()
    }

    fn destinations_count(&self) -> usize {
        self.vertex_set.len()
    }
}

impl<VT, Set> MutEdgeDescriptor<UndirectedEdge, VT> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    fn add_source_destination(&mut self, source_vertex_token: VT, destination_vertex_token: VT) {
        self.vertex_set
            .extend(std::iter::once(source_vertex_token).chain(Some(destination_vertex_token)));
    }

    fn remove_vertex(&mut self, vertex_token: VT) {
        self.vertex_set.remove(&vertex_token)
    }
}

impl<VT, Set> CheckedMutEdgeDescriptor<UndirectedEdge, VT> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}
