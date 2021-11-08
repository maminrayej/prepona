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
#[derive(Debug, PartialEq, Eq)]
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
mod test {
    use super::*;
    use crate::storage::edge::test_utils;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl<VT, Set> Clone for Hyperedge<VT, Set>
    where
        VT: VertexToken + Clone,
        Set: UnorderedSet<VT> + Clone,
    {
        fn clone(&self) -> Self {
            Self {
                vertex_set: self.vertex_set.clone(),
                phantom_vt: self.phantom_vt.clone(),
            }
        }
    }

    impl<VT, Set> Arbitrary for Hyperedge<VT, Set>
    where
        VT: VertexToken + Arbitrary,
        Set: UnorderedSet<VT> + Clone + 'static,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vec: Vec<VT> = Arbitrary::arbitrary(g);

            Hyperedge {
                vertex_set: Set::from_iter(vec),
                phantom_vt: PhantomData,
            }
        }
    }

    #[quickcheck]
    fn prop_edge_description(edge: HashedHyperedge<usize>) {
        test_utils::prop_edge_description(edge);
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_src(edge: HashedHyperedge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_dst(edge: HashedHyperedge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_src(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_dst(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_src(edge: HashedHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add_src(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_dst(edge: HashedHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add_dst(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add(edge: HashedHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_remove(edge: HashedHyperedge<usize>) {
        test_utils::prop_mut_descriptor_remove(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_src(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add_src(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_dst(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add_dst(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_remove(edge: HashedHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_remove(edge);
    }
}
