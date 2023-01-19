mod iter;
pub use iter::*;

mod reverse;
pub use reverse::*;

mod complement;
pub use complement::*;

mod selector;
pub use selector::*;

use crate::provide::*;
use crate::view::selector::Selector;

pub struct View<'a, S, NS, ES> {
    storage: &'a S,
    nselect: NS,
    eselect: ES,
}

impl<'a, S, NS, ES> View<'a, S, NS, ES> {
    pub fn new(storage: &'a S, nselect: NS, eselect: ES) -> Self {
        Self {
            storage,
            nselect,
            eselect,
        }
    }
}

impl<'a, S, NS, ES> Storage for View<'a, S, NS, ES>
where
    S: Storage,
{
    type Dir = S::Dir;

    // FIXME: This should be a custom map
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        todo!()
    }
}

impl<'b, S, NS, ES> Node for View<'b, S, NS, ES>
where
    S: Node,
    NS: Selector<Storage = S, Element = NodeID>,
{
    type Nodes<'a> = NodeSelector<'a, S, NS, S::Nodes<'a>> where Self: 'a;

    type Succs<'a> = NodeSelector<'a, S, NS, S::Succs<'a>> where Self: 'a;

    type Preds<'a> = NodeSelector<'a, S, NS, S::Preds<'a>> where Self: 'a;

    fn contains_node(&self, node: NodeID) -> bool {
        self.storage.contains_node(node) && self.nselect.select(self.storage, &node)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        NodeSelector {
            storage: self.storage,
            inner: self.storage.nodes(),
            nselect: &self.nselect,
        }
    }

    fn succs(&self, node: NodeID) -> Self::Succs<'_> {
        NodeSelector {
            storage: self.storage,
            inner: self.storage.succs(node),
            nselect: &self.nselect,
        }
    }

    fn preds(&self, node: NodeID) -> Self::Preds<'_> {
        NodeSelector {
            storage: self.storage,
            inner: self.storage.preds(node),
            nselect: &self.nselect,
        }
    }
}

impl<'b, S, NS, ES> Edge for View<'b, S, NS, ES>
where
    S: Edge,
    NS: Selector<Storage = S, Element = NodeID>,
    ES: Selector<Storage = S, Element = (NodeID, NodeID)>,
{
    type AllEdges<'a> = AllEdgeSelector<'a, S, NS, ES, S::AllEdges<'a>> where Self: 'a;

    type Incoming<'a> = EdgeSelector<'a, S, NS, ES, S::Incoming<'a>> where Self: 'a;

    type Outgoing<'a> = EdgeSelector<'a, S, NS, ES, S::Outgoing<'a>> where Self: 'a;

    fn contains_edge(&self, src: NodeID, dst: NodeID) -> bool {
        self.storage.contains_edge(src, dst)
            && self.nselect.select(self.storage, &src)
            && self.nselect.select(self.storage, &dst)
            && self.eselect.select(self.storage, &(src, dst))
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        AllEdgeSelector {
            storage: self.storage,
            inner: self.storage.edges(),
            nselect: &self.nselect,
            eselect: &self.eselect,
        }
    }

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_> {
        EdgeSelector {
            src: node,
            inner: self.storage.incoming(node),
            storage: self.storage,
            nselect: &self.nselect,
            eselect: &self.eselect,
        }
    }

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_> {
        EdgeSelector {
            src: node,
            inner: self.storage.outgoing(node),
            storage: self.storage,
            nselect: &self.nselect,
            eselect: &self.eselect,
        }
    }
}
