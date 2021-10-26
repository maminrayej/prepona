use super::UnorderedSet;
use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, CheckedMutEdgeDescriptor, Direction, EdgeDescriptor,
    FixedSizeMutEdgeDescriptor, MutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use std::collections::HashSet;
use std::marker::PhantomData;

/// A [`DirHyperedge`] that uses a hashmap as its unordered set.
pub type HashedDirHyperedge<VT> = DirHyperedge<VT, HashSet<VT>>;

/// A directed hyperedge edge that can connect multiple sources to multiple destinations.
///
/// A `DirHyperedge` is an ordered pair of non-empty subset of vertices.
/// All vertices in the first subset are sources that connect to all vertices in the second subset.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
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
    /// # Arguments
    /// * `src_vt`: Token of the source vertex.
    /// * `dst_vt`: Token of the destination vertex.
    ///
    /// # Returns
    /// Constructed `DirHyperedge` connecting `src_vt` to `dst_vt`.
    pub fn init(src_vt: VT, dst_vt: VT) -> Self {
        DirHyperedge {
            source_set: Set::from_iter(std::iter::once(src_vt)),
            destination_set: Set::from_iter(std::iter::once(dst_vt)),
            phantom_vt: PhantomData,
        }
    }

    /// # Arguments
    /// * `srv_vts`: An iterator over tokens of source vertices.
    /// * `dst_vts`: An iterator over tokens of destination vertices.
    ///
    /// # Returns
    /// Constructed `DirHyperedge` connecting all vertices in `src_vts` to all vertices in `dst_vts`.
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
    /// # Complexity
    /// O([`UnorderedSet::iterator`])
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.source_set.iterator())
    }

    /// # Complexity
    /// O([`UnorderedSet::iterator`])
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.destination_set.iterator())
    }

    /// # Complexity
    /// O([`UnorderedSet::contains`])
    fn is_source(&self, vt: &VT) -> bool {
        self.source_set.contains(vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::contains`])
    fn is_destination(&self, vt: &VT) -> bool {
        self.destination_set.contains(vt)
    }

    /// # Complexity
    /// O([`UnorderedSet::len`])
    fn sources_count(&self) -> usize {
        self.source_set.len()
    }

    /// # Complexity
    /// O([`UnorderedSet::len`])
    fn destinations_count(&self) -> usize {
        self.destination_set.len()
    }
}

impl<VT, Set> FixedSizeMutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
    /// # Complexity
    /// O([`UnorderedSet::replace`])
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.source_set.replace(src_vt, vt);
    }

    /// # Complexity
    /// O([`UnorderedSet::replace`])
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
    /// # Complexity
    /// O([`Extend::extend`] on [`UnorderedSet`])
    fn add(&mut self, src_vt: VT, dst_vt: VT) {
        self.source_set.extend(std::iter::once(src_vt));
        self.destination_set.extend(std::iter::once(dst_vt));
    }

    fn add_src(&mut self, src_vt: VT) {
        self.source_set.insert(src_vt);
    }

    fn add_dst(&mut self, dst_vt: VT) {
        self.destination_set.insert(dst_vt);
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
