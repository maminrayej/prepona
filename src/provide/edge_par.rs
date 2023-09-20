use rayon::iter::ParallelIterator;

use crate::error::{Error, Result};
use crate::provide::{EdgeId, NodeId, NodeRefPar};

pub trait EdgeRefPar: NodeRefPar {
    #[rustfmt::skip]
    type AllEdges<'a>: ParallelIterator<Item = (NodeId, NodeId, EdgeId, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type Incoming<'a>: ParallelIterator<Item = (NodeId, EdgeId, &'a Self::Edge)> where Self: 'a;
    #[rustfmt::skip]
    type Outgoing<'a>: ParallelIterator<Item = (NodeId, EdgeId, &'a Self::Edge)> where Self: 'a;

    fn contains_edge_par(&self, eid: EdgeId) -> bool;

    fn edges_par(&self) -> Self::AllEdges<'_>;
    fn incoming_par(&self, nid: NodeId) -> Self::Incoming<'_>;
    fn outgoing_par(&self, nid: NodeId) -> Self::Outgoing<'_>;

    fn incoming_par_checked(&self, nid: NodeId) -> Result<Self::Incoming<'_>> {
        if self.contains_node_par(nid) {
            Ok(self.incoming_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn outgoing_checked(&self, nid: NodeId) -> Result<Self::Outgoing<'_>> {
        if self.contains_node_par(nid) {
            Ok(self.outgoing_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}
