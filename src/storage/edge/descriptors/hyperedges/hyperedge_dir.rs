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
    /// * `srv_vts`: An iterator over tokens of source vertices.
    /// * `dst_vts`: An iterator over tokens of destination vertices.
    ///
    /// # Returns
    /// Constructed `DirHyperedge` connecting all vertices in `src_vts` to all vertices in `dst_vts`.
    pub fn init(
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

    /// # Complexity
    /// O([`UnorderedSet::insert`])
    fn add_src(&mut self, src_vt: VT) {
        self.source_set.insert(src_vt);
    }

    /// # Complexity
    /// O([`UnorderedSet::insert`])
    fn add_dst(&mut self, dst_vt: VT) {
        self.destination_set.insert(dst_vt);
    }

    /// # Complexity
    /// O([`UnorderedSet::remove`])
    fn remove(&mut self, vt: &VT) {
        self.source_set.remove(vt);
        self.destination_set.remove(vt)
    }
}

impl<VT, Set> CheckedMutEdgeDescriptor<VT, true> for DirHyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_directed_edge_description<VT: VertexToken, S: UnorderedSet<VT>>(
        edge: &DirHyperedge<VT, S>,
        src_vts_iter: impl IntoIterator<Item = VT>,
        dst_vts_iter: impl IntoIterator<Item = VT>,
    ) {
        let src_vts: Vec<VT> = src_vts_iter.into_iter().collect();
        let dst_vts: Vec<VT> = dst_vts_iter.into_iter().collect();

        // It must return only 0 as its source.
        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return only 1 as its destination.
        assert_eq!(
            HashSet::<_>::from_iter(dst_vts.iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is only a source.
        for src_vt in src_vts.iter() {
            assert!(edge.is_source(src_vt));
            assert!(edge.contains(src_vt));
        }

        for dst_vt in dst_vts.iter() {
            assert!(edge.is_destination(dst_vt));
            assert!(edge.contains(dst_vt));
        }

        // It contain only one source and one destination.
        assert_eq!(edge.sources_count(), src_vts.len());
        assert_eq!(edge.destinations_count(), dst_vts.len());
    }

    #[test]
    fn edge_descriptor() {
        assert_directed_edge_description(
            &HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]),
            [0, 1, 2],
            [0, 4, 5],
        );
    }

    #[test]
    fn fixed_descriptor_replace_src() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (target_vt, vt) in [0, 1, 2].iter().zip([6, 7, 8]) {
            hyperedge.replace_src(target_vt, vt);
        }

        assert_directed_edge_description(&hyperedge, [6, 7, 8], [0, 4, 5]);
    }

    #[test]
    fn fixed_descriptor_replace_dst() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (target_vt, vt) in [0, 4, 5].iter().zip([6, 7, 8]) {
            hyperedge.replace_dst(target_vt, vt);
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2], [6, 7, 8]);
    }

    #[test]
    fn checked_fixed_descriptor_replace_src() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (target_vt, vt) in [10, 11, 12].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_src_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [0, 1, 2].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_src_checked(target_vt, vt).is_ok())
        }

        assert_directed_edge_description(&hyperedge, [6, 7, 8], [0, 4, 5]);
    }

    #[test]
    fn checked_fixed_descriptor_replace_dst() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (target_vt, vt) in [10, 11, 12].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [0, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_ok())
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2], [6, 7, 8]);
    }

    #[test]
    fn mut_descriptor_add_src() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for src_vt in [4, 6, 7] {
            hyperedge.add_src(src_vt);
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2, 4, 6, 7], [0, 4, 5]);
    }

    #[test]
    fn mut_descriptor_add_dst() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for dst_vt in [6, 7, 8] {
            hyperedge.add_dst(dst_vt);
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2], [0, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn mut_descriptor_add() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (src_vt, dst_vt) in [4, 6, 7].into_iter().zip([10, 11, 12]) {
            hyperedge.add(src_vt, dst_vt);
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2, 4, 6, 7], [0, 4, 5, 10, 11, 12]);
    }

    #[test]
    fn mut_descriptor_remove() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for vt in [0, 1, 4].iter() {
            hyperedge.remove(vt);
        }

        // TODO: investigate what happens if there is a source and no destination.
        assert_directed_edge_description(&hyperedge, [2], [5]);
    }

    #[test]
    fn checked_mut_descriptor_add_src() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for src_vt in [0, 1, 2] {
            assert!(hyperedge.add_src_checked(src_vt).is_err());
        }

        for src_vt in [4, 6, 7] {
            assert!(hyperedge.add_src_checked(src_vt).is_ok())
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2, 4, 6, 7], [0, 4, 5]);
    }

    #[test]
    fn checked_mut_descriptor_add_dst() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for dst_vt in [0, 4, 5] {
            assert!(hyperedge.add_dst_checked(dst_vt).is_err());
        }

        for dst_vt in [6, 7, 8] {
            assert!(hyperedge.add_dst_checked(dst_vt).is_ok());
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2], [0, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn checked_mut_descriptor_add() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for (src_vt, dst_vt) in [0, 1, 2].into_iter().zip([5, 4, 0]) {
            assert!(hyperedge.add_checked(src_vt, dst_vt).is_err());
        }

        for (src_vt, dst_vt) in [4, 6, 7].into_iter().zip([10, 11, 12]) {
            assert!(hyperedge.add_checked(src_vt, dst_vt).is_ok());
        }

        assert_directed_edge_description(&hyperedge, [0, 1, 2, 4, 6, 7], [0, 4, 5, 10, 11, 12]);
    }

    #[test]
    fn checked_mut_descriptor_remove() {
        let mut hyperedge = HashedDirHyperedge::init([0, 1, 2], [0, 4, 5]);

        for vt in [10, 11, 14].iter() {
            assert!(hyperedge.remove_checked(vt).is_err());
        }

        for vt in [0, 1, 4].iter() {
            assert!(hyperedge.remove_checked(vt).is_ok())
        }

        assert_directed_edge_description(&hyperedge, [2], [5]);
    }
}
