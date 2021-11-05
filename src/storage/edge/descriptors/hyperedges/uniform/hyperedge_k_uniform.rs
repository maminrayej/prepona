use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, Direction, EdgeDescriptor, FixedSizeMutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;
use std::marker::PhantomData;

use super::KElementCollection;

/// A [`KUniformHyperedge`] that uses an array of size `K` as its [`KElementCollection`].
pub type ArrKUniformHyperedge<VT, const K: usize> = KUniformHyperedge<VT, [VT; K], K>;

/// An edge that can connect exactly `K` vertices.
///
/// A `KUniformHyperedge` is a non-empty subset of exactly `K` vertices. All vertices participating in a hyperedge are connected together.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `C`: A collection that contains `K` elements at all times.
/// * `K`: Number of connected vertices.
#[derive(Debug, PartialEq, Eq)]
pub struct KUniformHyperedge<VT, C, const K: usize>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    vertices: C,

    phantom_vt: PhantomData<VT>,
}

impl<VT, C, const K: usize> KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    /// # Arguments
    /// `vertex_tokens`: An iterator that contains exactly `K` elements.
    ///
    /// # Returns
    /// `Ok`: Containing the constructed `KUniformHyperedge` if `vertex_tokens` contains exactly `K` elements.
    /// `Err`: Specifically [`StorageError::NotKElement`] error if `vertex_tokens` does not contain exactly `K` elements.
    pub fn try_init(vertex_tokens: impl Iterator<Item = VT>) -> Result<Self> {
        let vertex_tokens = vertex_tokens.collect::<Vec<VT>>();
        let tokens_count = vertex_tokens.len();

        match C::try_from(vertex_tokens) {
            Ok(vertices) => Ok(KUniformHyperedge {
                vertices,
                phantom_vt: PhantomData,
            }),
            _ => Err(StorageError::NotKElement(tokens_count, K).into()),
        }
    }
}

impl<VT, C, const K: usize> Direction<false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

impl<VT, C, const K: usize> EdgeDescriptor<VT, false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    /// # Complexity
    /// O(1)
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertices.iterator())
    }

    /// # Complexity
    /// O(1)
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertices.iterator())
    }

    /// # Complexity
    /// O([`KElementCollection::contains_value`])
    fn is_source(&self, vt: &VT) -> bool {
        self.vertices.contains_value(vt)
    }

    /// # Complexity
    /// O([`KElementCollection::contains_value`])
    fn is_destination(&self, vt: &VT) -> bool {
        self.vertices.contains_value(vt)
    }

    /// # Complexity
    /// O([`KElementCollection::contains_value`])
    fn contains(&self, vt: &VT) -> bool {
        self.vertices.contains_value(vt)
    }

    /// # Complexity
    /// O([`KElementCollection::len`])
    fn sources_count(&self) -> usize {
        self.vertices.len()
    }

    /// # Complexity
    /// O([`KElementCollection::len`])
    fn destinations_count(&self) -> usize {
        self.vertices.len()
    }
}

impl<VT, C, const K: usize> FixedSizeMutEdgeDescriptor<VT, false> for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    /// # Complexity
    /// O([`KElementCollection::replace`])
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.vertices.replace(src_vt, vt);
    }

    /// # Complexity
    /// O([`KElementCollection::replace`])
    fn replace_dst(&mut self, dst_vt: &VT, vt: VT) {
        self.vertices.replace(dst_vt, vt);
    }
}

impl<VT, C, const K: usize> CheckedFixedSizeMutEdgeDescriptor<VT, false>
    for KUniformHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use rand::prelude::IteratorRandom;
    use rand::Rng;
    use std::collections::HashSet;

    impl<VT, C, const K: usize> Clone for KUniformHyperedge<VT, C, K>
    where
        VT: VertexToken + Clone,
        C: KElementCollection<VT, K> + Clone,
    {
        fn clone(&self) -> Self {
            Self {
                vertices: self.vertices.clone(),
                phantom_vt: self.phantom_vt.clone(),
            }
        }
    }

    impl<C, const K: usize> Arbitrary for KUniformHyperedge<usize, C, K>
    where
        C: KElementCollection<usize, K> + Clone + 'static,
    {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let mut k_vertices = vec![];
            for _ in 0..K {
                let new_vertex_token = get_non_duplicate(k_vertices.iter().copied(), 1)[0];
                k_vertices.push(new_vertex_token);
            }

            if let Ok(vertices) = C::try_from(k_vertices) {
                KUniformHyperedge {
                    vertices,
                    phantom_vt: PhantomData,
                }
            } else {
                panic!("Creating collection failed.");
            }
        }
    }

    fn assert_undirected_edge_description<VT: VertexToken, const K: usize>(
        edge: &ArrKUniformHyperedge<VT, K>,
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
    fn prop_edge_descriptor(edge: ArrKUniformHyperedge<usize, 8>) {
        assert_undirected_edge_description(
            &edge,
            edge.get_sources().copied(),
            edge.get_destinations().copied(),
        );
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_src(mut edge: ArrKUniformHyperedge<usize, 8>) {
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
    fn prop_fixed_size_descriptor_replace_dst(mut edge: ArrKUniformHyperedge<usize, 8>) {
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
    fn prop_checked_fixed_size_descriptor_replace_src(mut edge: ArrKUniformHyperedge<usize, 8>) {
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
    fn prop_checked_fixed_size_descriptor_replace_dst(mut edge: ArrKUniformHyperedge<usize, 8>) {
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
