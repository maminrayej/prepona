use crate::common::DynIter;
use crate::storage::edge::EdgeDescriptor;

pub trait Edges {
    type E: EdgeDescriptor;

    fn edge(&self, et: usize) -> (usize, usize, &Self::E);

    fn edge_count(&self) -> usize;

    fn edge_tokens(&self) -> DynIter<'_, usize>;

    fn edges(&self) -> DynIter<'_, (usize, usize, &Self::E)>;

    fn ingoing_edges(&self, vt: usize) -> DynIter<'_, usize>;

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize>;

    fn has_et(&self, et: usize) -> bool;
}

pub trait MutEdges: Edges {
    fn edge_mut(&mut self, et: usize) -> (usize, usize, &mut Self::E);

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize;

    fn remove_edge(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Self::E;
}
