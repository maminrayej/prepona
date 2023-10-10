use rayon::iter::ParallelIterator;

use crate::error::{Error, Result};
use crate::provide::{EdgeId, NodeId, NodeRefPar};

pub trait EdgeRefPar: NodeRefPar {
    #[rustfmt::skip]
    type AllEdgesPar<'a>: ParallelIterator<Item = (&'a Self::Node, &'a Self::Node, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type IncomingPar<'a>: ParallelIterator<Item = (&'a Self::Node, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type OutgoingPar<'a>: ParallelIterator<Item = (&'a Self::Node, &'a Self::Edge)> where Self: 'a;

    fn has_edge_par(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool;

    fn edges_par(&self) -> Self::AllEdgesPar<'_>;
    fn incoming_par(&self, nid: NodeId) -> Self::IncomingPar<'_>;
    fn outgoing_par(&self, nid: NodeId) -> Self::OutgoingPar<'_>;

    fn incoming_checked_par(&self, nid: NodeId) -> Result<Self::IncomingPar<'_>> {
        if self.has_node_par(nid) {
            Ok(self.incoming_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn outgoing_checked_par(&self, nid: NodeId) -> Result<Self::OutgoingPar<'_>> {
        if self.has_node_par(nid) {
            Ok(self.outgoing_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}
