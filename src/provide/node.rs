use crate::error::{Error, Result};
use crate::provide::Storage;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

pub trait NodeRef: Storage {
    #[rustfmt::skip]
    type Nodes<'a>: Iterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;
    #[rustfmt::skip]
    type Succs<'a>: Iterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;
    #[rustfmt::skip]
    type Preds<'a>: Iterator<Item = (NodeId, &'a Self::Node)> where Self: 'a;

    fn contains_node(&self, nid: NodeId) -> bool;
    fn node_count(&self) -> usize;

    /* TODO: should this method return Option<&Self::Node> */
    fn node(&self, nid: NodeId) -> &Self::Node;
    fn nodes(&self) -> Self::Nodes<'_>;
    fn succs(&self, nid: NodeId) -> Self::Succs<'_>;
    fn preds(&self, nid: NodeId) -> Self::Preds<'_>;

    fn node_checked(&self, nid: NodeId) -> Result<&Self::Node> {
        if self.contains_node(nid) {
            Ok(self.node(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn succs_checked(&self, nid: NodeId) -> Result<Self::Succs<'_>> {
        if self.contains_node(nid) {
            Ok(self.succs(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }

    fn preds_checked(&self, nid: NodeId) -> Result<Self::Preds<'_>> {
        if self.contains_node(nid) {
            Ok(self.preds(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}

pub trait NodeAdd: NodeRef {
    fn add_node(&mut self, nid: NodeId, node: Self::Node);

    fn add_node_checked(&mut self, nid: NodeId, node: Self::Node) -> Result<()> {
        if !self.contains_node(nid) {
            Ok(self.add_node(nid, node))
        } else {
            Err(Error::NodeExists(nid))
        }
    }
}

pub trait NodeDel: NodeRef {
    fn node_del(&mut self, nid: NodeId);

    fn node_del_checked(&mut self, nid: NodeId) -> Result<()> {
        if self.contains_node(nid) {
            Ok(self.node_del(nid))
        } else {
            Err(Error::NodeNotFound(nid))
        }
    }
}
