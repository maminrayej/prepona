use std::marker::PhantomData;

use crate::storage::edge::{
    CheckedFixedSizeMutEdgeDescriptor, Direction, EdgeDescriptor, FixedSizeMutEdgeDescriptor,
};
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;

use super::KElementCollection;

/// A [`KUniformDirHyperedge`] that uses an array of size `K` as its [`KElementCollection`].
pub type ArrKUniformDirHyperedge<VT, const K: usize> = KUniformDirHyperedge<VT, [VT; K], K>;

/// A directed edge that can connect exactly `K` vertices.
///
/// A `KUniformDirHyperedge` is a non-empty subset of exactly `K` vertices. It consists of one `tail`(source) that is connected to `K-1` heads(destinations).
/// For more info checkout [`KUniformDirHyperedge::try_init`].
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `C`: A collection that contains `K` elements at all times.
/// * `K`: Number of connected vertices.
#[derive(Debug, PartialEq, Eq)]
pub struct KUniformDirHyperedge<VT, C, const K: usize>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    vertices: C,

    phantom_vt: PhantomData<VT>,
}

impl<VT, C, const K: usize> KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    /// First element in `vertex_tokens` iterator will be the tail(source) of the edge and the rest `K-1` of vertices will be the heads(destinations).
    /// So for example if `vertex_tokens` contains {1, 2, 3}, the connection between vertices will be as follows:
    ///         .---> 2
    ///         |
    ///     1 ---
    ///         |
    ///         .---> 3
    ///
    /// # Arguments
    /// `vertex_tokens`: An iterator containing exactly `K` elements.
    ///
    /// # Returns
    /// `Ok`: Containing the constructed `KUniformDirHyperedge` edge, if `vertex_tokens` contains exactly `K` elements.
    /// `Err`: Specifically [`StorageError::NotKElement`] error if `vertex_tokens` does not contain exactly `K` elements.
    pub fn try_init(vertex_tokens: impl Iterator<Item = VT>) -> Result<Self> {
        let vertex_tokens = vertex_tokens.collect::<Vec<VT>>();
        let tokens_count = vertex_tokens.len();

        match C::try_from(vertex_tokens) {
            Ok(vertices) => Ok(KUniformDirHyperedge {
                vertices,
                phantom_vt: PhantomData,
            }),
            _ => Err(StorageError::NotKElement(tokens_count, K).into()),
        }
    }
}

impl<VT, C, const K: usize> Direction<true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

impl<VT, C, const K: usize> EdgeDescriptor<VT, true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    /// # Complexity
    /// O([`std::ops::Index::index`] on [`KElementCollection`])
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(std::iter::once(&self.vertices[0]))
    }

    /// # Complexity
    /// O(1)
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertices.iterator().skip(1))
    }

    /// # Complexity
    /// O([`std::ops::Index::index`] on [`KElementCollection`])
    fn is_source(&self, vt: &VT) -> bool {
        &self.vertices[0] == vt
    }

    /// # Complexity
    /// O(`K`)
    fn is_destination(&self, vt: &VT) -> bool {
        self.vertices.iterator().skip(1).any(|dst_vt| dst_vt == vt)
    }

    /// # Complexity
    /// O(1)
    fn sources_count(&self) -> usize {
        1
    }

    /// # Complexity
    /// O(1)
    fn destinations_count(&self) -> usize {
        K - 1
    }
}

impl<VT, C, const K: usize> FixedSizeMutEdgeDescriptor<VT, true> for KUniformDirHyperedge<VT, C, K>
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

impl<VT, C, const K: usize> CheckedFixedSizeMutEdgeDescriptor<VT, true>
    for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use rand::prelude::IteratorRandom;
    use rand::Rng;
    use std::collections::HashSet;

    impl<C, const K: usize> Clone for KUniformDirHyperedge<usize, C, K>
    where
        C: KElementCollection<usize, K> + Clone,
    {
        fn clone(&self) -> Self {
            Self {
                vertices: self.vertices.clone(),
                phantom_vt: self.phantom_vt.clone(),
            }
        }
    }

    impl<C, const K: usize> Arbitrary for KUniformDirHyperedge<usize, C, K>
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
                KUniformDirHyperedge {
                    vertices,
                    phantom_vt: PhantomData,
                }
            } else {
                panic!("Creating collection failed.");
            }
        }
    }

    fn assert_directed_edge_description<VT: VertexToken, const K: usize>(
        edge: &ArrKUniformDirHyperedge<VT, K>,
        src_vts_iter: impl IntoIterator<Item = VT>,
        dst_vts_iter: impl IntoIterator<Item = VT>,
    ) {
        let src_vts: Vec<VT> = src_vts_iter.into_iter().collect();
        let dst_vts: Vec<VT> = dst_vts_iter.into_iter().collect();

        assert_eq!(
            HashSet::<_>::from_iter(src_vts.iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        assert_eq!(
            HashSet::<_>::from_iter(dst_vts.iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        for src_vt in src_vts.iter() {
            assert!(edge.is_source(src_vt));
            assert!(edge.contains(src_vt));
        }

        for dst_vt in dst_vts.iter() {
            assert!(edge.is_destination(dst_vt));
            assert!(edge.contains(dst_vt));
        }

        assert_eq!(edge.sources_count(), src_vts.len());
        assert_eq!(edge.destinations_count(), dst_vts.len());
    }

    #[quickcheck]
    fn prop_edge_descriptor(edge: ArrKUniformDirHyperedge<usize, 8>) {
        assert_directed_edge_description(
            &edge,
            edge.get_sources().copied(),
            edge.get_destinations().copied(),
        );
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_src(mut edge: ArrKUniformDirHyperedge<usize, 8>) {
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

            assert_directed_edge_description(&edge, new_src_vts, edge.get_destinations().copied());
        }
    }

    #[quickcheck]
    fn prop_fixed_size_descriptor_replace_dst(mut edge: ArrKUniformDirHyperedge<usize, 8>) {
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

            assert_directed_edge_description(&edge, edge.get_sources().copied(), new_dst_vts);
        }
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_src(mut edge: ArrKUniformDirHyperedge<usize, 8>) {
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

            assert_directed_edge_description(&edge, new_src_vts, edge.get_destinations().copied());
        }
    }

    #[quickcheck]
    fn prop_checked_fixed_size_descriptor_replace_dst(mut edge: ArrKUniformDirHyperedge<usize, 8>) {
        let dst_vts: Vec<usize> = edge.get_destinations().copied().collect();

        if !dst_vts.is_empty() {
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

            assert_directed_edge_description(&edge, edge.get_sources().copied(), new_dst_vts);
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
