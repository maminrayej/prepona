use crate::error::{Error, Result};
use crate::provide::{Id, NodeId, NodeRef};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeId(usize);

pub trait EdgeRef: NodeRef {
    #[rustfmt::skip]
    type AllEdges<'a>: Iterator<Item = (&'a Self::Node, &'a Self::Node, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type Incoming<'a>: Iterator<Item = (&'a Self::Node, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type Outgoing<'a>: Iterator<Item = (&'a Self::Node, &'a Self::Edge)> where Self: 'a;

    fn has_edge(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool;

    fn edges(&self) -> Self::AllEdges<'_>;
    fn incoming(&self, nid: NodeId) -> Self::Incoming<'_>;
    fn outgoing(&self, nid: NodeId) -> Self::Outgoing<'_>;

    fn incoming_checked(&self, nid: NodeId) -> Result<Self::Incoming<'_>> {
        if self.has_node(nid) {
            Ok(self.incoming(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn outgoing_checked(&self, nid: NodeId) -> Result<Self::Outgoing<'_>> {
        if self.has_node(nid) {
            Ok(self.outgoing(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}

pub trait EdgeAdd: EdgeRef {
    fn add_edge(&mut self, src: NodeId, dst: NodeId, edge: Self::Edge);

    fn add_edge_checked(&mut self, src: NodeId, dst: NodeId, edge: Self::Edge) -> Result<()> {
        if !self.has_edge(src, dst, edge.id()) {
            Ok(self.add_edge(src, dst, edge))
        } else {
            Err(Error::EdgeExists(src, dst, edge.id()))
        }
    }
}

pub trait EdgeDel: EdgeRef {
    fn del_edge(&mut self, src: NodeId, dst: NodeId, eid: EdgeId) -> Self::Edge;

    fn del_edge_checked(&mut self, src: NodeId, dst: NodeId, eid: EdgeId) -> Result<Self::Edge> {
        if self.has_edge(src, dst, eid) {
            Ok(self.del_edge(src, dst, eid))
        } else {
            Err(Error::EdgeNotFound(src, dst, eid))
        }
    }
}
