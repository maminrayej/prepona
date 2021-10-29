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
#[derive(PartialEq, Eq)]
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
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn assert_undirected_edge_description<VT: VertexToken, const K: usize>(
        edge: &ArrKUniformHyperedge<VT, K>,
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
        assert_undirected_edge_description(
            &ArrKUniformHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap(),
            [0, 1, 2],
            [0, 1, 2],
        );
    }

    #[test]
    fn fixed_size_descriptor_replace_src() {
        let mut hyperedge =
            ArrKUniformHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            hyperedge.replace_src(target_vt, vt);
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn fixed_size_descriptor_replace_dst() {
        let mut hyperedge =
            ArrKUniformHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            hyperedge.replace_dst(target_vt, vt);
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }

    #[test]
    fn checked_fixed_size_descriptor_replace_src() {
        let mut hyperedge =
            ArrKUniformHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

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
        let mut hyperedge =
            ArrKUniformHyperedge::<usize, 3>::try_init([0, 1, 2].into_iter()).unwrap();

        for (target_vt, vt) in [3, 4, 5].iter().zip([6, 7, 8]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_err());
        }

        for (target_vt, vt) in [0, 1, 2].iter().zip([3, 4, 5]) {
            assert!(hyperedge.replace_dst_checked(target_vt, vt).is_ok());
        }

        assert_undirected_edge_description(&hyperedge, [3, 4, 5], [3, 4, 5]);
    }
}
