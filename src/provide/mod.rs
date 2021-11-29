use crate::{
    common::DynIter,
    storage::{edge::EdgeDescriptor, vertex::VertexDescriptor},
};

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

pub trait MutVertices {
    type V: VertexDescriptor;

    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V;

    fn add_vertex(&mut self, vertex: Self::V) -> usize;

    fn remove_vertex(&mut self, vt: usize) -> Self::V;
}

pub trait Edges<const DIR: bool> {
    type E: EdgeDescriptor<DIR>;

    fn edge(&self, et: usize) -> &Self::E;

    fn edge_count(&self) -> usize;

    fn edge_tokens(&self) -> DynIter<'_, usize>;

    fn edges(&self) -> DynIter<'_, &Self::E>;

    fn ingoing_edges(&self, vt: usize) -> DynIter<'_, usize>;

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize>;

    fn has_edge(&self, et: usize) -> bool;
}

pub trait MutEdges<const DIR: bool> {
    type E: EdgeDescriptor<DIR>;

    fn edge_mut(&mut self, et: usize) -> &mut Self::E;

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize;

    fn remove_edge(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Self::E;
}
