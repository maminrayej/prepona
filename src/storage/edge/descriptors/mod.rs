mod edge;
mod hyperedge;

use anyhow::Result;
pub use edge::Edge;
pub use hyperedge::{HashHyperedge, Hyperedge, UnorderedSet};

use super::direction::EdgeDirection;
use crate::storage::{vertex::VertexToken, StorageError};

pub trait EdgeDescriptor<Dir: EdgeDirection, VT: VertexToken>: PartialEq + Eq {
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_>;
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    fn is_directed() -> bool {
        Dir::is_directed()
    }

    fn is_source(&self, vertex_token: &VT) -> bool {
        self.get_sources()
            .find(|source_vertex_token| *source_vertex_token == vertex_token)
            .is_some()
    }

    fn is_destination(&self, vertex_token: &VT) -> bool {
        self.get_destinations()
            .find(|destination_vertex_token| *destination_vertex_token == vertex_token)
            .is_some()
    }

    fn contains(&self, vertex_token: &VT) -> bool {
        self.is_source(vertex_token) || self.is_destination(vertex_token)
    }

    fn sources_count(&self) -> usize {
        self.get_sources().count()
    }

    fn destinations_count(&self) -> usize {
        self.get_destinations().count()
    }
}

pub trait MutEdgeDescriptor<Dir: EdgeDirection, VT: VertexToken>: EdgeDescriptor<Dir, VT> {
    fn add_source_destination(&mut self, source_vertex_token: VT, destination_vertex_token: VT);

    fn remove_vertex(&mut self, vertex_token: VT);
}

pub trait CheckedMutEdgeDescriptor<Dir: EdgeDirection, VT: VertexToken>:
    MutEdgeDescriptor<Dir, VT>
{
    fn add_source_destination_checked(
        &mut self,
        source_vertex_token: VT,
        destination_vertex_token: VT,
    ) -> Result<()> {
        Ok(self.add_source_destination(source_vertex_token, destination_vertex_token))
    }

    fn remove_vertex_checked(&mut self, vertex_token: VT) -> Result<()> {
        if !self.contains(&vertex_token) {
            Err(StorageError::VertexNotFound(vertex_token.to_string()))?
        }

        Ok(self.remove_vertex(vertex_token))
    }
}
