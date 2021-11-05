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
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use rand::prelude::{IteratorRandom, SliceRandom};
    use rand::Rng;

    use super::*;

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

    fn assert_undirected_edge_description<VT: VertexToken, S: UnorderedSet<VT>>(
        edge: &Hyperedge<VT, S>,
        src_vts_iter: impl IntoIterator<Item = VT>,
        dst_vts_iter: impl IntoIterator<Item = VT>,
    ) {
        let src_vts: Vec<VT> = src_vts_iter.into_iter().collect();
        let dst_vts: Vec<VT> = dst_vts_iter.into_iter().collect();

        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter().chain(dst_vts.iter())),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter().chain(dst_vts.iter())),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

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

        assert_eq!(edge.sources_count(), src_vts.len());
        assert_eq!(edge.destinations_count(), dst_vts.len());
    }

    #[quickcheck]
    fn prop_edge_descriptor(edge: HashedHyperedge<usize>) {
        assert_undirected_edge_description(
            &edge,
            edge.get_sources().copied(),
            edge.get_destinations().copied(),
        );
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_src(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap();

            let new_src_vt = get_non_duplicate(src_vts.iter().copied(), 1)[0];

            edge.replace_src(&src_vt, new_src_vt);

            let new_src_vts = src_vts
                .iter()
                .copied()
                .filter(|vt| *vt != src_vt)
                .chain(std::iter::once(new_src_vt));

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_dst(mut edge: HashedHyperedge<usize>) {
        let dst_vts: Vec<usize> = edge.get_destinations().copied().collect();

        if !dst_vts.is_empty() {
            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap();

            let new_dst_vt = get_non_duplicate(dst_vts.iter().copied(), 1)[0];

            edge.replace_dst(&dst_vt, new_dst_vt);

            let new_dst_vts = dst_vts
                .iter()
                .copied()
                .filter(|vt| *vt != dst_vt)
                .chain(std::iter::once(new_dst_vt));

            assert_undirected_edge_description(&edge, new_dst_vts.clone(), new_dst_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_src(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap();

            let new_vts = get_non_duplicate(src_vts.iter().copied(), 2);
            let invalid_vt = new_vts[0];
            let new_src_vt = new_vts[1];

            assert!(edge.replace_src_checked(&invalid_vt, new_src_vt).is_err());

            assert!(edge.replace_src_checked(&src_vt, new_src_vt).is_ok());

            let new_src_vts = src_vts
                .iter()
                .copied()
                .filter(|vt| *vt != src_vt)
                .chain(std::iter::once(new_src_vt));

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_dst(mut edge: HashedHyperedge<usize>) {
        let dst_vts: Vec<usize> = edge.get_destinations().copied().collect();

        if !dst_vts.is_empty() {
            // Choose one destination vertex randomly.
            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .copied()
                .unwrap();

            let new_vts = get_non_duplicate(dst_vts.iter().copied(), 2);
            let invalid_vt = new_vts[0];
            let new_dst_vt = new_vts[1];

            assert!(edge.replace_dst_checked(&invalid_vt, new_dst_vt).is_err());

            assert!(edge.replace_dst_checked(&dst_vt, new_dst_vt).is_ok());

            let new_dst_vts = dst_vts
                .iter()
                .copied()
                .filter(|vt| *vt != dst_vt)
                .chain(std::iter::once(new_dst_vt));

            assert_undirected_edge_description(&edge, new_dst_vts.clone(), new_dst_vts);
        }
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_src(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        let new_src_vt = get_non_duplicate(src_vts.iter().copied(), 1)[0];

        edge.add_src(new_src_vt);

        let new_src_vts = src_vts.iter().copied().chain(std::iter::once(new_src_vt));

        assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add_dst(mut edge: HashedHyperedge<usize>) {
        let dst_vts: Vec<usize> = edge.get_destinations().copied().collect();

        let new_dst_vt = get_non_duplicate(dst_vts.iter().copied(), 1)[0];

        edge.add_dst(new_dst_vt);

        let new_dst_vts = dst_vts.iter().copied().chain(std::iter::once(new_dst_vt));

        assert_undirected_edge_description(&edge, edge.get_sources().copied(), new_dst_vts);
    }

    #[quickcheck]
    fn prop_mut_descriptor_add(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        let new_vts = get_non_duplicate(src_vts.iter().copied(), 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        edge.add(new_src_vt, new_dst_vt);

        let new_src_vts = src_vts.iter().copied().chain([new_src_vt, new_dst_vt]);

        assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
    }

    #[quickcheck]
    fn prop_mut_descriptor_remove(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.choose(&mut rand::thread_rng()).unwrap();

            edge.remove(src_vt);

            let new_src_vts = src_vts.iter().copied().filter(|vt| vt != src_vt);

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_src(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.choose(&mut rand::thread_rng()).copied().unwrap();

            let new_src_vt = get_non_duplicate(src_vts.iter().copied(), 1)[0];

            assert!(edge.add_src_checked(src_vt).is_err());
            assert!(edge.add_src_checked(new_src_vt).is_ok());

            let new_src_vts = src_vts.iter().copied().chain(std::iter::once(new_src_vt));

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add_dst(mut edge: HashedHyperedge<usize>) {
        let dst_vts: Vec<usize> = edge.get_destinations().copied().collect();

        if !dst_vts.is_empty() {
            let dst_vt = dst_vts.choose(&mut rand::thread_rng()).copied().unwrap();

            let new_dst_vt = get_non_duplicate(dst_vts.iter().copied(), 1)[0];

            assert!(edge.add_dst_checked(dst_vt).is_err());
            assert!(edge.add_dst_checked(new_dst_vt).is_ok());

            let new_dst_vts = dst_vts.iter().copied().chain(std::iter::once(new_dst_vt));

            assert_undirected_edge_description(&edge, new_dst_vts.clone(), new_dst_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_add(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.choose(&mut rand::thread_rng()).copied().unwrap();
            let dst_vt = src_vts.choose(&mut rand::thread_rng()).copied().unwrap();

            let new_vts = get_non_duplicate(src_vts.iter().copied(), 2);
            let new_src_vt = new_vts[0];
            let new_dst_vt = new_vts[1];

            assert!(edge.add_checked(src_vt, dst_vt).is_err());
            assert!(edge.add_checked(new_src_vt, new_dst_vt).is_ok());

            let new_src_vts = src_vts.iter().copied().chain([new_src_vt, new_dst_vt]);

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_mut_descriptor_remove(mut edge: HashedHyperedge<usize>) {
        let src_vts: Vec<usize> = edge.get_sources().copied().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.choose(&mut rand::thread_rng()).unwrap();

            let invalid_vt = get_non_duplicate(src_vts.iter().copied(), 1)[0];

            assert!(edge.remove_checked(&invalid_vt).is_err());
            assert!(edge.remove_checked(src_vt).is_ok());

            let new_src_vts = src_vts.iter().copied().filter(|vt| vt != src_vt);

            assert_undirected_edge_description(&edge, new_src_vts.clone(), new_src_vts);
        }
    }

    fn get_non_duplicate(set_iter: impl IntoIterator<Item = usize>, count: usize) -> Vec<usize> {
        let mut set = HashSet::<_>::from_iter(set_iter);

        let mut rng = rand::thread_rng();

        let mut values = vec![0; count];

        for index in 0..count {
            let mut value: usize = rng.gen();
            while set.contains(&value) {
                value = rng.gen();
            }
            values[index] = value;
            set.insert(value);
        }

        values
    }
}
