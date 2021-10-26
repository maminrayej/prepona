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
/// A `Hyperedge` is a non-empty subset of vertices. All vertices participating in a hyperedge are connected together.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
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
    /// `vt`: Token of the vertex to be added as the only vertex participating in the hyperedge.
    ///
    /// # Returns
    /// A hyperedge containing `vt` as its only participating vertex.
    pub fn init(vt: VT) -> Self {
        Hyperedge {
            vertex_set: Set::from_iter(std::iter::once(vt)),
            phantom_vt: PhantomData,
        }
    }

    /// # Arguments
    /// `vts`: An iterator over tokens of vertices to be added as participating vertices in the hyperedge.
    ///
    /// # Returns
    /// A hyperedge containing vertex tokens returned by `vts`.
    pub fn init_multiple(vts: impl IntoIterator<Item = VT>) -> Self {
        Hyperedge {
            vertex_set: Set::from_iter(vts),
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
    /// # Complexity
    /// O([`UnorderedSet::iterator`])
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertex_set.iterator())
    }

    /// # Complexity
    /// O([`UnorderedSet::iterator`])
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertex_set.iterator())
    }

    /// # Complexity
    /// O([`UnorderedSet::iterator`])
    fn is_source(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::contains`])
    fn is_destination(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::contains`])
    fn contains(&self, vt: &VT) -> bool {
        self.vertex_set.contains(vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::len`])
    fn sources_count(&self) -> usize {
        self.vertex_set.len()
    }

    /// # Complexity
    /// O([`UnorderedSet::len`])
    fn destinations_count(&self) -> usize {
        self.vertex_set.len()
    }
}

impl<VT, Set> FixedSizeMutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    /// # Complexity
    /// O([`UnorderedSet::replace`])
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.vertex_set.replace(src_vt, vt);
    }

    /// # Complexity
    /// O([`UnorderedSet::replace`])
    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.vertex_set.replace(dst_vt, vt);
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
    /// # Complexity
    /// O([`Extend::extend`] on [`UnorderedSet`])
    fn add(&mut self, src_vt: VT, dst_vt: VT) {
        self.vertex_set
            .extend(std::iter::once(src_vt).chain(Some(dst_vt)));
    }

    /// # Complexity
    /// O([`UnorderedSet::insert`])
    fn add_src(&mut self, src_vt: VT) {
        self.vertex_set.insert(src_vt);
    }

    /// # Complexity
    /// O([`UnorderedSet::insert`])
    fn add_dst(&mut self, dst_vt: VT) {
        self.vertex_set.insert(dst_vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::remove`])
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
