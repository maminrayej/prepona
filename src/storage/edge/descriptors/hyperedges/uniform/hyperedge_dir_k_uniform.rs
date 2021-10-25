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
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
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
    /// First element in `vertices` iterator will be the tail(source) of the edge and the rest `K-1` of vertices will be the heads(destinations).
    /// So if `vertices` contains {1, 2, 3}, the connection between vertices will be as follows:
    ///         .---> 2
    ///         |
    ///     1 ---
    ///         |
    ///         .---> 3
    ///
    /// # Arguments
    /// `vertices`: An iterator containing exactly `K` elements.
    ///
    /// # Returns
    /// `Ok`: Containing the constructed `KUniformDirHyperedge` edge, if `vertices` contains exactly `K` elements.
    /// `Err`: Specifically [`StorageError::NotKElement`] error if `vertices` does not contain exactly `K` elements.
    pub fn try_init(vertices: impl Iterator<Item = VT>) -> Result<Self> {
        let items = vertices.collect::<Vec<VT>>();
        let items_count = items.len();

        match C::try_from(items) {
            Ok(vertices) => Ok(KUniformDirHyperedge {
                vertices,
                phantom_v: PhantomData,
            }),
            _ => Err(StorageError::NotKElement(items_count, K).into()),
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
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(std::iter::once(&self.vertices[0]))
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        Box::new(self.vertices.iterator().skip(1))
    }

    fn is_source(&self, vt: &VT) -> bool {
        &self.vertices[0] == vt
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.vertices
            .iterator()
            .skip(1)
            .find(|dst_vt| *dst_vt == vt)
            .is_some()
    }

    fn sources_count(&self) -> usize {
        1
    }

    fn destinations_count(&self) -> usize {
        K - 1
    }
}

impl<VT, C, const K: usize> FixedSizeMutEdgeDescriptor<VT, true> for KUniformDirHyperedge<VT, C, K>
where
    VT: VertexToken,
    C: KElementCollection<VT, K>,
{
    fn replace_src(&mut self, src_vt: &VT, vt: VT) {
        self.vertices.replace(src_vt, vt);
    }

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
