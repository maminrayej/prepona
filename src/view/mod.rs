mod errors;
mod generic_view;
mod reverse_view;
mod undirected_view;

pub use errors::ViewError;
pub use generic_view::*;
pub use reverse_view::*;
pub use undirected_view::*;

use crate::provide::{Edges, Vertices};
use anyhow::Result;

pub trait FrozenView<G>: Vertices + Edges
where
    G: Vertices + Edges,
{
    fn inner(&self) -> &G;
}

pub trait SubgraphView<G>: FrozenView<G>
where
    G: Vertices + Edges,
{
    fn add_vertex_from_inner(&mut self, vid: usize);
    fn add_vertex_from_inner_checked(&mut self, vid: usize) -> Result<()> {
        if !self.inner().has_vt(vid) {
            return Err(ViewError::InnerVertexNotFound(vid).into());
        }

        Ok(self.add_vertex_from_inner(vid))
    }

    fn remove_vertex_from_view(&mut self, vid: usize);
    fn remove_vertex_from_view_checked(&mut self, vid: usize) -> Result<()> {
        if !self.has_vt(vid) {
            return Err(ViewError::VertexNotFound(vid).into());
        }

        Ok(self.remove_vertex_from_view(vid))
    }

    fn add_edge_from_inner(&mut self, eid: usize);
    fn add_edge_from_inner_checked(&mut self, eid: usize) -> Result<()> {
        if !self.inner().has_et(eid) {
            return Err(ViewError::InnerEdgeNotFound(eid).into());
        }

        Ok(self.add_edge_from_inner(eid))
    }

    fn remove_edge_from_view(&mut self, eid: usize);
    fn remove_edge_from_view_checked(&mut self, eid: usize) -> Result<()> {
        if !self.has_et(eid) {
            return Err(ViewError::EdgeNotFound(eid).into());
        }

        Ok(self.remove_edge_from_view(eid))
    }
}
