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
#[derive(Debug, PartialEq, Eq)]
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
pub mod test {
    use super::*;
    use quickcheck::Arbitrary;

    impl<VT, Set> Clone for DirHyperedge<VT, Set>
    where
        VT: VertexToken + Clone,
        Set: UnorderedSet<VT> + Clone,
    {
        fn clone(&self) -> Self {
            Self {
                source_set: self.source_set.clone(),
                destination_set: self.destination_set.clone(),
                phantom_vt: self.phantom_vt.clone(),
            }
        }
    }

    impl<VT, Set> Arbitrary for DirHyperedge<VT, Set>
    where
        VT: VertexToken + Arbitrary,
        Set: UnorderedSet<VT> + Clone + 'static,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let src_vts: Vec<VT> = Arbitrary::arbitrary(g);
            let dst_vts: Vec<VT> = Arbitrary::arbitrary(g);

            DirHyperedge {
                source_set: Set::from_iter(src_vts),
                destination_set: Set::from_iter(dst_vts),
                phantom_vt: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::edge::test_utils;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_edge_description(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_edge_description(edge);
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_src(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_dst(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_src(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_dst(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_src(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add_src(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_dst(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add_dst(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_mut_descriptor_add(edge);
    }

    #[quickcheck]
    fn prop_mut_descriptor_remove(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_mut_descriptor_remove(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_src(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add_src(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_dst(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add_dst(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_add(edge);
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_remove(edge: HashedDirHyperedge<usize>) {
        test_utils::prop_checked_mut_descriptor_remove(edge);
    }
}
