use rayon::iter::ParallelIterator;

use crate::give::{Error, Node, NodeID};

#[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
pub trait NodePar: Node {
    #[rustfmt::skip]
    type NodesPar<'a>: ParallelIterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type SuccsPar<'a>: ParallelIterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type PredsPar<'a>: ParallelIterator<Item = NodeID> where Self: 'a;

    // >>> Required functions <<< //
    fn nodes_par(&self) -> Self::NodesPar<'_>;

    fn succs_par(&self, node: NodeID) -> Self::SuccsPar<'_>;

    fn preds_par(&self, node: NodeID) -> Self::PredsPar<'_>;

    // >>> Provided functions <<< //
    fn is_succ_of_par(&self, node: NodeID, succ: NodeID) -> bool {
        self.succs_par(node).any(|s| s == succ)
    }

    fn is_pred_of_par(&self, node: NodeID, pred: NodeID) -> bool {
        self.preds_par(node).any(|p| p == pred)
    }

    // >>> Checked functions <<< //
    fn succs_par_checked(&self, node: NodeID) -> Result<Self::SuccsPar<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.succs_par(node))
    }

    fn preds_par_checked(&self, node: NodeID) -> Result<Self::PredsPar<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.preds_par(node))
    }

    fn is_succ_of_par_checked(&self, node: NodeID, succ: NodeID) -> Result<bool, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        } else if !self.has_node(succ) {
            return Err(Error::NodeNotFound(succ));
        }

        Ok(self.is_succ_of_par(node, succ))
    }

    fn is_pred_of_par_checked(&self, node: NodeID, pred: NodeID) -> Result<bool, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        } else if !self.has_node(pred) {
            return Err(Error::NodeNotFound(pred));
        }

        Ok(self.is_pred_of_par(node, pred))
    }
}
