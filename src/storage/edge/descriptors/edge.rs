use super::{EdgeDescriptor, FixedSizeMutEdgeDescriptor};
use crate::storage::{edge::Direction, vertex::VertexToken};

pub type DirectedEdge<VT> = Edge<VT, true>;
pub type UndirectedEdge<VT> = Edge<VT, false>;

#[derive(PartialEq, Eq)]
pub struct Edge<VT: VertexToken, const DIR: bool> {
    src_vt: VT,
    dst_vt: VT,
}

impl<VT: VertexToken, const DIR: bool> Edge<VT, DIR> {
    pub fn init(src_vt: VT, dst_vt: VT) -> Self {
        Edge { src_vt, dst_vt }
    }
}

impl<VT: VertexToken, const DIR: bool> Direction<DIR> for Edge<VT, DIR> {}

impl<VT: VertexToken, const DIR: bool> EdgeDescriptor<VT, DIR> for Edge<VT, DIR> {
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.src_vt))
        } else {
            Box::new(std::iter::once(&self.src_vt).chain(Some(&self.dst_vt)))
        }
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.dst_vt))
        } else {
            Box::new(std::iter::once(&self.dst_vt).chain(Some(&self.src_vt)))
        }
    }

    fn is_source(&self, vt: &VT) -> bool {
        &self.src_vt == vt || (Self::is_undirected() && &self.dst_vt == vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        &self.dst_vt == vt || (Self::is_undirected() && &self.src_vt == vt)
    }

    fn contains(&self, vt: &VT) -> bool {
        &self.src_vt == vt || &self.dst_vt == vt
    }

    fn sources_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }

    fn destinations_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }
}

impl<VT: VertexToken, const DIR: bool> FixedSizeMutEdgeDescriptor<VT, DIR> for Edge<VT, DIR> {
    fn replace_src(&mut self, _: &VT, vt: VT) {
        self.src_vt = vt
    }

    fn replace_dst(&mut self, _: &VT, vt: VT) {
        self.dst_vt = vt
    }
}
