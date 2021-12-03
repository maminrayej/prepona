use crate::common::DynIter;
use crate::storage::vertex::VertexDescriptor;

pub trait Vertices {
    type V: VertexDescriptor;

    fn vertex(&self, vt: usize) -> &Self::V;

    fn vertex_count(&self) -> usize;

    fn vertex_tokens(&self) -> DynIter<'_, usize>;

    fn vertices(&self) -> DynIter<'_, &Self::V>;

    fn neighbors(&self, vt: usize) -> DynIter<'_, usize>;

    fn has_vt(&self, vt: usize) -> bool;

    fn successors(&self, vt: usize) -> DynIter<'_, usize>;

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize>;
}

pub trait MutVertices: Vertices {
    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V;

    fn add_vertex(&mut self, vertex: Self::V) -> usize;

    fn remove_vertex(&mut self, vt: usize) -> Self::V;
}
