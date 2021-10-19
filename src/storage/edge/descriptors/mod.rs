mod edge;
mod hyperedge;
mod hyperedge_dir;
mod hyperedge_k_uniform;

use super::Direction;
use crate::storage::{vertex::VertexToken, StorageError};
use anyhow::Result;

pub use edge::{DirectedEdge, Edge, UndirectedEdge};
pub use hyperedge::{HashHyperedge, Hyperedge, UnorderedSet};
pub use hyperedge_dir::DirHyperedge;
pub use hyperedge_k_uniform::KUniformHyperedge;

pub trait EdgeDescriptor<VT: VertexToken, const DIR: bool>:
    PartialEq + Eq + Direction<DIR>
{
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_>;
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    fn is_source(&self, vt: &VT) -> bool {
        self.get_sources().any(|src_vt| src_vt == vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        self.get_destinations().any(|dst_vt| dst_vt == vt)
    }

    fn contains(&self, vt: &VT) -> bool {
        self.is_source(vt) || self.is_destination(vt)
    }

    fn sources_count(&self) -> usize {
        self.get_sources().count()
    }

    fn destinations_count(&self) -> usize {
        self.get_destinations().count()
    }
}

// TODO: Add checked version
pub trait FixedSizeMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    EdgeDescriptor<VT, DIR>
{
    fn replace_src(&mut self, src_vt: &VT, vt: VT);

    fn replace_dst(&mut self, dst_vt: &VT, vt: VT);

    fn replace_src_dst(&mut self, src_vt: &VT, new_src_vt: VT, dst_vt: &VT, new_dst_vt: VT) {
        if self.is_source(src_vt) && self.is_destination(dst_vt) {
            self.replace_src(src_vt, new_src_vt);
            self.replace_dst(dst_vt, new_dst_vt);
        }
    }
}

pub trait MutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    FixedSizeMutEdgeDescriptor<VT, DIR>
{
    fn add(&mut self, src_vt: VT, dst_vt: VT);

    fn remove(&mut self, vertex_token: VT);
}

pub trait CheckedMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    MutEdgeDescriptor<VT, DIR>
{
    fn add_checked(&mut self, src_vt: VT, dst_vt: VT) -> Result<()> {
        self.add(src_vt, dst_vt);

        Ok(())
    }

    fn remove_checked(&mut self, vt: VT) -> Result<()> {
        if !self.contains(&vt) {
            return Err(StorageError::VertexNotFound(vt.to_string()).into());
        }

        self.remove(vt);

        Ok(())
    }
}
