use anyhow::Result;

use crate::common::DynIter;
use crate::storage::edge::EdgeDescriptor;
use crate::storage::StorageError;

use super::Vertices;

pub trait Edges: Vertices {
    type E: EdgeDescriptor;

    fn has_et(&self, et: usize) -> bool;

    fn edge(&self, et: usize) -> (usize, usize, &Self::E);
    fn edge_checked(&self, et: usize) -> Result<(usize, usize, &Self::E)> {
        if !self.has_et(et) {
            Err(StorageError::InvalidEdgeToken(et).into())
        } else {
            Ok(self.edge(et))
        }
    }

    fn edge_count(&self) -> usize;
    fn edge_count_checked(&self) -> Result<usize> {
        Ok(self.edge_count())
    }

    fn edge_tokens(&self) -> DynIter<'_, usize>;
    fn edge_tokens_checked(&self) -> Result<DynIter<'_, usize>> {
        Ok(self.edge_tokens())
    }

    fn edges(&self) -> DynIter<'_, (usize, usize, &Self::E)>;
    fn edges_checked(&self) -> Result<DynIter<'_, (usize, usize, &Self::E)>> {
        Ok(self.edges())
    }

    fn ingoing_edges(&self, vt: usize) -> DynIter<'_, usize>;
    fn ingoing_edges_checked(&self, vt: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.ingoing_edges(vt))
        }
    }

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize>;
    fn outgoing_edges_checked(&self, vt: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.outgoing_edges(vt))
        }
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> DynIter<'_, usize>;
    fn edges_between_checked(&self, src_id: usize, dst_id: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(src_id) {
            Err(StorageError::InvalidVertexToken(src_id).into())
        } else if !self.has_vt(dst_id) {
            Err(StorageError::InvalidVertexToken(dst_id).into())
        } else {
            Ok(self.edges_between(src_id, dst_id))
        }
    }
}

pub trait MutEdges: Edges {
    fn has_free_et(&mut self) -> bool;

    fn edge_mut(&mut self, et: usize) -> (usize, usize, &mut Self::E);
    fn edge_mut_checked(&mut self, et: usize) -> Result<(usize, usize, &mut Self::E)> {
        if !self.has_et(et) {
            Err(StorageError::InvalidEdgeToken(et).into())
        } else {
            Ok(self.edge_mut(et))
        }
    }

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize;
    fn add_edge_checked(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> Result<usize> {
        if !self.has_vt(src_vt) {
            Err(StorageError::InvalidVertexToken(src_vt).into())
        } else if !self.has_vt(dst_vt) {
            Err(StorageError::InvalidVertexToken(dst_vt).into())
        } else if !self.has_free_et() {
            Err(StorageError::NoMoreEdgeToken.into())
        } else {
            Ok(self.add_edge(src_vt, dst_vt, edge))
        }
    }

    fn remove_edge(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Self::E;
    fn remove_edge_checked(&mut self, src_vt: usize, dst_vt: usize, et: usize) -> Result<Self::E> {
        if !self.has_vt(src_vt) {
            Err(StorageError::InvalidVertexToken(src_vt).into())
        } else if !self.has_vt(dst_vt) {
            Err(StorageError::InvalidVertexToken(dst_vt).into())
        } else if !self.has_et(et) {
            Err(StorageError::InvalidEdgeToken(et).into())
        } else {
            Ok(self.remove_edge(src_vt, dst_vt, et))
        }
    }
}
