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
#[derive(PartialEq, Eq)]
pub struct KUniformDirHyperedge<VT, C, const K: usize>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    vertices: C,

    phantom_v: PhantomData<VT>,
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
                phantom_v: PhantomData,
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
    use std::collections::HashSet;

    use super::*;

    fn assert_directed_edge_description<VT: VertexToken, const K: usize>(
        edge: &ArrKUniformDirHyperedge<VT, K>,
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
            &ArrKUniformDirHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap(),
            [0],
            [1, 2],
        );
    }

    #[test]
    fn fixed_size_descriptor_replace_src() {
        let mut hyperedge =
            ArrKUniformDirHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        hyperedge.replace_src(&0, 3);

        assert_directed_edge_description(&hyperedge, [3], [1, 2]);
    }

    #[test]
    fn fixed_size_descriptor_replace_dst() {
        let mut hyperedge =
            ArrKUniformDirHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [1, 2].iter().zip([3, 4]) {
            hyperedge.replace_dst(target_vt, vt);
        }

        assert_directed_edge_description(&hyperedge, [0], [3, 4]);
    }

    #[test]
    fn checked_fixed_size_descriptor_replace_src() {
        let mut hyperedge =
            ArrKUniformDirHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [3, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_src_checked(target_vt, vt).is_err());
        }

        assert!(hyperedge.replace_src_checked(&0, 3).is_ok());

        assert_directed_edge_description(&hyperedge, [3], [1, 2]);
    }

    #[test]
    fn checked_fixed_size_descriptor_replace_dst() {
        let mut hyperedge =
            ArrKUniformDirHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [3, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [1, 2].iter().zip([3, 4]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_ok());
        }

        assert_directed_edge_description(&hyperedge, [0], [3, 4]);
    }
}
