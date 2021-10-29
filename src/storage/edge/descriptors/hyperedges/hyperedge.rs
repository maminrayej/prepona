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
    /// `vts`: An iterator over tokens of vertices to be added as participating vertices in the hyperedge.
    ///
    /// # Returns
    /// A hyperedge containing vertex tokens returned by `vts`.
    pub fn init(vts: impl IntoIterator<Item = VT>) -> Self {
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
    fn remove(&mut self, vt: &VT) {
        self.vertex_set.remove(vt)
    }
}

impl<VT, Set> CheckedMutEdgeDescriptor<VT, false> for Hyperedge<VT, Set>
where
    VT: VertexToken,
    Set: UnorderedSet<VT>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_undirected_edge_description<VT: VertexToken, S: UnorderedSet<VT>>(
        edge: &Hyperedge<VT, S>,
        src_vts_iter: impl IntoIterator<Item = VT>,
        dst_vts_iter: impl IntoIterator<Item = VT>,
    ) {
        let src_vts: Vec<VT> = src_vts_iter.into_iter().collect();
        let dst_vts: Vec<VT> = dst_vts_iter.into_iter().collect();

        // It must return both 0 and 1 as sources.
        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter().chain(dst_vts.iter())),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return both 0 and 1 as destinations.
        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter().chain(dst_vts.iter())),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is both a source and a destination.
        for src_vt in src_vts.iter() {
            assert!(edge.is_source(src_vt));
            assert!(edge.is_destination(src_vt));
            assert!(edge.contains(src_vt));
        }

        for dst_vt in dst_vts.iter() {
            assert!(edge.is_source(dst_vt));
            assert!(edge.is_destination(dst_vt));
            assert!(edge.contains(dst_vt));
        }

        // It must contain 2 sources and 2 destinations.
        assert_eq!(edge.sources_count(), src_vts.len());
        assert_eq!(edge.destinations_count(), dst_vts.len());
    }

    #[test]
    fn edge_descriptor() {
        assert_undirected_edge_description(&HashedHyperedge::init([0, 1, 2]), [0, 1, 2], [0, 1, 2]);
    }

    #[test]
    fn fixed_size_descriptor_replace_src() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            hyperedge.replace_src(target_vt, vt);
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn fixed_size_descriptor_replace_dst() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            hyperedge.replace_dst(target_vt, vt);
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn checked_fixed_size_descriptor_replace_src() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (target_vt, vt) in [3, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_src_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            assert!(hyperedge.replace_src_checked(target_vt, vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn checked_fixed_size_descriptor_replace_dst() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (target_vt, vt) in [3, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn mut_descriptor_add_src() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [3, 4, 5].into_iter() {
            hyperedge.add_src(vt);
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn mut_descriptor_add_dst() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [3, 4, 5].into_iter() {
            hyperedge.add_dst(vt);
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn mut_descriptor_add() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (src_vt, dst_vt) in [0, 1, 2].into_iter().zip([3, 4, 5].into_iter()) {
            hyperedge.add(src_vt, dst_vt);
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn mut_descriptor_remove() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [0, 1, 2].iter() {
            hyperedge.remove(vt);
        }

        assert_undirected_edge_description(&hyperedge, [], []);
    }

    #[test]
    fn checked_mut_descriptor_add_src() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [0, 1, 2].into_iter() {
            assert!(hyperedge.add_src_checked(vt).is_err());
        }

        for vt in [3, 4, 5].into_iter() {
            assert!(hyperedge.add_src_checked(vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn checked_mut_descriptor_add_dst() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [0, 1, 2].into_iter() {
            assert!(hyperedge.add_dst_checked(vt).is_err());
        }

        for vt in [3, 4, 5].into_iter() {
            assert!(hyperedge.add_dst_checked(vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn checked_mut_descriptor_add() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for (src_vt, dst_vt) in [0, 1, 2].into_iter().zip([2, 1, 0].into_iter()) {
            assert!(hyperedge.add_checked(src_vt, dst_vt).is_err());
        }

        for (src_vt, dst_vt) in [0, 1, 2].into_iter().zip([3, 4, 5].into_iter()) {
            assert!(hyperedge.add_checked(src_vt, dst_vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [0, 1, 2, 3, 4, 5], [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn checked_mut_descriptor_remove() {
        let mut hyperedge = HashedHyperedge::init([0, 1, 2]);

        for vt in [3, 4, 5].iter() {
            assert!(hyperedge.remove_checked(vt).is_err());
        }

        for vt in [0, 1, 2].iter() {
            assert!(hyperedge.remove_checked(vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [], []);
    }
}
