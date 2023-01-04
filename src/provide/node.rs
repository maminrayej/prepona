use crate::provide::{Error, Storage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeID(pub(crate) usize);

pub trait Node: Storage {
    #[rustfmt::skip]
    type Nodes<'a>: Iterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type Succs<'a>: Iterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type Preds<'a>: Iterator<Item = NodeID> where Self: 'a;

    // >>> Required Functions <<< //
    fn contains_node(&self, node: NodeID) -> bool;

    fn nodes(&self) -> Self::Nodes<'_>;

    fn succs(&self, node: NodeID) -> Self::Succs<'_>;

    fn preds(&self, node: NodeID) -> Self::Preds<'_>;

    // >>> Provided Functions <<< //
    fn node_conut(&self) -> usize {
        self.nodes().count()
    }

    fn is_succ_of(&self, node: NodeID, succ: NodeID) -> bool {
        self.succs(node).any(|s| s == succ)
    }

    fn is_pred_of(&self, node: NodeID, pred: NodeID) -> bool {
        self.preds(node).any(|s| s == pred)
    }

    // >>> Checked Functions <<< //
    fn succs_checked(&self, node: NodeID) -> Result<Self::Succs<'_>, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.succs(node))
    }

    fn preds_checked(&self, node: NodeID) -> Result<Self::Preds<'_>, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.preds(node))
    }

    fn is_succ_of_checked(&self, node: NodeID, succ: NodeID) -> Result<bool, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        } else if !self.contains_node(succ) {
            return Err(Error::NodeNotFound(succ));
        }

        Ok(self.is_succ_of(node, succ))
    }

    fn is_pred_of_checked(&self, node: NodeID, pred: NodeID) -> Result<bool, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        } else if !self.contains_node(pred) {
            return Err(Error::NodeNotFound(pred));
        }

        Ok(self.is_pred_of(node, pred))
    }
}

pub trait AddNode: Node {
    fn add_node(&mut self, node: NodeID);
    fn add_node_checked(&mut self, node: NodeID) -> Result<(), Error> {
        if self.contains_node(node) {
            return Err(Error::NodeExists(node));
        }

        #[allow(clippy::unit_arg)]
        Ok(self.add_node(node))
    }
}

pub trait DelNode: Node {
    #[rustfmt::skip]
    type DeletedEdges: Iterator<Item = (NodeID, NodeID)>;

    fn del_node(&mut self, node: NodeID) -> Self::DeletedEdges;
    fn del_node_checked(&mut self, node: NodeID) -> Result<Self::DeletedEdges, Error> {
        if self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.del_node(node))
    }
}
