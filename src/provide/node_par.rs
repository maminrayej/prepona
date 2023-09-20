use rayon::iter::ParallelIterator;

use crate::error::{Error, Result};
use crate::provide::{NodeId, Storage};

pub trait NodeRefPar: Storage {
    #[rustfmt::skip]
    type Nodes<'a>: ParallelIterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;
    #[rustfmt::skip]
    type Succs<'a>: ParallelIterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;
    #[rustfmt::skip]
    type Preds<'a>: ParallelIterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;

    fn contains_node_par(&self, nid: NodeId) -> bool;

    fn nodes_par(&self) -> Self::Nodes<'_>;
    fn succs_par(&self, nid: NodeId) -> Self::Succs<'_>;
    fn preds_par(&self, nid: NodeId) -> Self::Preds<'_>;

    fn succs_par_checked(&self, nid: NodeId) -> Result<Self::Succs<'_>> {
        if self.contains_node_par(nid) {
            Ok(self.succs_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
    fn preds_par_checked(&self, nid: NodeId) -> Result<Self::Preds<'_>> {
        if self.contains_node_par(nid) {
            Ok(self.preds_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}
