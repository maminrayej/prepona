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
