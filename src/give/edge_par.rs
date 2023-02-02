use rayon::iter::ParallelIterator;

use crate::give::{Edge, Error, NodeID};

#[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
pub trait EdgePar: Edge {
    #[rustfmt::skip]
    type AllEdgesPar<'a>: ParallelIterator<Item = (NodeID, NodeID)> where Self: 'a;
    #[rustfmt::skip]
    type IncomingPar<'a>: ParallelIterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type OutgoingPar<'a>: ParallelIterator<Item = NodeID> where Self: 'a;

    // >>> Required Functions <<< //
    fn edges_par(&self) -> Self::AllEdges<'_>;

    fn incoming_par(&self, node: NodeID) -> Self::IncomingPar<'_>;

    fn outgoing_par(&self, node: NodeID) -> Self::OutgoingPar<'_>;

    // >>> Checked Functions <<< //
    fn incoming_par_checked(&self, node: NodeID) -> Result<Self::IncomingPar<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.incoming_par(node))
    }

    fn outgoing_par_checked(&self, node: NodeID) -> Result<Self::OutgoingPar<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.outgoing_par(node))
    }
}
