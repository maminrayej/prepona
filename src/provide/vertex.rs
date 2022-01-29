use anyhow::Result;

use super::Storage;
use crate::common::{DynIter, IdMap};
use crate::storage::vertex::VertexDescriptor;
use crate::storage::StorageError;

pub trait Vertices: Storage {
    type V: VertexDescriptor;

    fn has_vt(&self, vt: usize) -> bool;

    fn id_map(&self) -> IdMap {
        IdMap::init(self.vertex_tokens())
    }

    fn vertex(&self, vt: usize) -> &Self::V;
    fn vertex_checked(&self, vt: usize) -> Result<&Self::V> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.vertex(vt))
        }
    }

    fn vertex_count(&self) -> usize;
    fn vertex_count_checked(&self) -> Result<usize> {
        Ok(self.vertex_count())
    }

    fn vertex_tokens(&self) -> DynIter<'_, usize>;
    fn vertex_tokens_checked(&self) -> Result<DynIter<'_, usize>> {
        Ok(self.vertex_tokens())
    }

    fn vertices(&self) -> DynIter<'_, &Self::V>;
    fn vertices_checked(&self) -> Result<DynIter<'_, &Self::V>> {
        Ok(self.vertices())
    }

    fn neighbors(&self, vt: usize) -> DynIter<'_, usize>;
    fn neighbors_checked(&self, vt: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.neighbors(vt))
        }
    }

    fn successors(&self, vt: usize) -> DynIter<'_, usize>;
    fn successors_checked(&self, vt: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.successors(vt))
        }
    }

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize>;
    fn predecessors_checked(&self, vt: usize) -> Result<DynIter<'_, usize>> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.predecessors(vt))
        }
    }
}

pub trait MutVertices: Vertices {
    fn has_free_token(&mut self) -> bool;

    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V;
    fn vertex_mut_checked(&mut self, vt: usize) -> Result<&mut Self::V> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.vertex_mut(vt))
        }
    }

    fn add_vertex(&mut self, vertex: Self::V) -> usize;
    fn add_vertex_checked(&mut self, vertex: Self::V) -> Result<usize> {
        if !self.has_free_token() {
            Err(StorageError::NoMoreVertexToken.into())
        } else {
            Ok(self.add_vertex(vertex))
        }
    }

    fn remove_vertex(&mut self, vt: usize) -> Self::V;
    fn remove_vertex_checked(&mut self, vt: usize) -> Result<Self::V> {
        if !self.has_vt(vt) {
            Err(StorageError::InvalidVertexToken(vt).into())
        } else {
            Ok(self.remove_vertex(vt))
        }
    }
}
