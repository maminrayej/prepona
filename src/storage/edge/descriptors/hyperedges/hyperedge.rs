use super::UnorderedSet;
use crate::storage::edge::descriptors::FixedSizeMutEdgeDescriptor;
use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, CheckedMutEdgeDescriptor, Direction, EdgeDescriptor,
    MutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use std::collections::HashSet;
use std::marker::PhantomData;

/// A [`Hyperedge`] that uses a hashmap as its unordered set.
pub type HashedHyperedge<VT> = Hyperedge<VT, HashSet<VT>>;

/// An edge that can connect multiple vertices together.
///
/// A `Hyperedge` is a non-empty subset of vertices. All vertices participating in a hyperedge are connected together(for further reading see [here]).
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
/// * `Set`: The unordered set to be used as backing storage of vertex tokens.
///
/// [here]: https://en.wikipedia.org/wiki/Hypergraph
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
    /// # Arguments
    /// `vertex_token`: Token of the vertex to be added as the only vertex participating in the hyperedge.
    ///
    /// # Returns
    /// A hyperedge containing `vertex_token` as its only participating vertex.
    pub fn init(vertex_token: VT) -> Self {
        Hyperedge {
            vertex_set: Set::from_iter(std::iter::once(vertex_token)),
            phantom_vt: PhantomData,
        }
    }

    /// # Arguments
    /// `vertex_tokens`: An iterator over tokens of vertices to be added as participating vertices in the hyperedge.
    ///
    /// # Returns
    /// A hyperedge containing vertex tokens returned by `vertex_tokens`.
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

    fn add_src(&mut self, src_vt: VT) {
        self.vertex_set.insert(src_vt);
    }

    fn add_dst(&mut self, dst_vt: VT) {
        self.vertex_set.insert(dst_vt)
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
