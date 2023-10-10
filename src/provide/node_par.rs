use rayon::iter::ParallelIterator;

use crate::error::{Error, Result};
use crate::provide::{NodeId, Storage};

pub trait NodeRefPar: Storage {
    #[rustfmt::skip]
    type NodesPar<'a>: ParallelIterator<Item = &'a Self::Node> where Self: 'a;
    #[rustfmt::skip]
    type SuccsPar<'a>: ParallelIterator<Item = &'a Self::Node> where Self: 'a;
    #[rustfmt::skip]
    type PredsPar<'a>: ParallelIterator<Item = &'a Self::Node> where Self: 'a;

    fn has_node_par(&self, nid: NodeId) -> bool;

    fn node_count_par(&self) -> usize;

    /* TODO: should this method return Option<&Self::Node> */
    fn node_par(&self, nid: NodeId) -> &Self::Node;
    fn nodes_par(&self) -> Self::NodesPar<'_>;
    fn succs_par(&self, nid: NodeId) -> Self::SuccsPar<'_>;
    fn preds_par(&self, nid: NodeId) -> Self::PredsPar<'_>;

    fn node_checked_par(&self, nid: NodeId) -> Result<&Self::Node> {
        if self.has_node_par(nid) {
            Ok(self.node_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn succs_checked_par(&self, nid: NodeId) -> Result<Self::SuccsPar<'_>> {
        if self.has_node_par(nid) {
            Ok(self.succs_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn preds_checked_par(&self, nid: NodeId) -> Result<Self::PredsPar<'_>> {
        if self.has_node_par(nid) {
            Ok(self.preds_par(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}
