use std::collections::HashMap;

use crate::give::{Edge, Error, Node, NodeID, Storage};
use crate::view::{
    AllEdges, EdgeSetSelector, Incoming, NodeSelector, NodeSetSelector, Outgoing, View,
};

pub struct DijkstraSPT<'a, S> {
    pub(super) view: View<'a, S, NodeSetSelector<S>, EdgeSetSelector<S>>,
    pub(super) cost: HashMap<NodeID, usize>,
}

impl<'a, S> DijkstraSPT<'a, S> {
    pub fn cost_of(&self, node: NodeID) -> usize {
        self.cost[&node]
    }
}

impl<'a, S> DijkstraSPT<'a, S>
where
    S: Node,
{
    pub fn cost_of_checked(&self, node: NodeID) -> Result<usize, Error> {
        if !self.view.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.cost_of(node))
    }
}

impl<'a, S> Storage for DijkstraSPT<'a, S>
where
    S: Storage,
{
    type Dir = S::Dir;

    // FIXME: This should be custom map
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        todo!()
    }
}

impl<'b, S> Node for DijkstraSPT<'b, S>
where
    S: Node,
{
    type Nodes<'a> = NodeSelector<'a, S, NodeSetSelector<S>, S::Nodes<'a>> where Self: 'a;

    type Succs<'a> = NodeSelector<'a, S, NodeSetSelector<S>, S::Succs<'a>> where Self: 'a;

    type Preds<'a> = NodeSelector<'a, S, NodeSetSelector<S>, S::Preds<'a>> where Self: 'a;

    fn has_node(&self, node: NodeID) -> bool {
        self.view.has_node(node)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.view.nodes()
    }

    fn succs(&self, node: NodeID) -> Self::Succs<'_> {
        self.view.succs(node)
    }

    fn preds(&self, node: NodeID) -> Self::Preds<'_> {
        self.view.preds(node)
    }
}

impl<'b, S> Edge for DijkstraSPT<'b, S>
where
    S: Edge,
{
    type AllEdges<'a> = AllEdges<'a, S, NodeSetSelector<S>, EdgeSetSelector<S>, S::AllEdges<'a>> where Self: 'a;

    type Incoming<'a> = Incoming<'a, S, NodeSetSelector<S>, EdgeSetSelector<S>, S::Incoming<'a>> where Self: 'a;

    type Outgoing<'a> = Outgoing<'a, S, NodeSetSelector<S>, EdgeSetSelector<S>, S::Outgoing<'a>> where Self: 'a;

    fn has_edge(&self, src: NodeID, dst: NodeID) -> bool {
        self.view.has_edge(src, dst)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        self.view.edges()
    }

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_> {
        self.view.incoming(node)
    }

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_> {
        self.view.outgoing(node)
    }
}
