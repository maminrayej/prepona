use std::iter::Chain;

use crate::provide::{Directed, EdgeId, EdgeRef, NodeId, NodeRef, Storage, Undirected};

pub struct UndirectedView<'a, S> {
    storage: &'a S,
}

impl<'a, S> UndirectedView<'a, S>
where
    S: Storage<Dir = Directed>,
{
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }
}

impl<'a, S> Storage for UndirectedView<'a, S>
where
    S: Storage<Dir = Directed>,
{
    type Node = S::Node;
    type Edge = S::Edge;
    type Dir = Undirected;
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        self.storage.idmap()
    }
}

impl<'a, S> NodeRef for UndirectedView<'a, S>
where
    S: NodeRef<Dir = Directed>,
{
    type Nodes<'b> = S::Nodes<'b> where Self: 'b;
    type Succs<'b> = Chain<S::Succs<'b>, S::Preds<'b>> where Self: 'b;
    type Preds<'b> = Chain<S::Succs<'b>, S::Preds<'b>> where Self: 'b;

    fn has_node(&self, nid: NodeId) -> bool {
        self.storage.has_node(nid)
    }

    fn node_count(&self) -> usize {
        self.storage.node_count()
    }

    fn node(&self, nid: NodeId) -> &Self::Node {
        self.storage.node(nid)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.storage.nodes()
    }

    fn succs(&self, nid: NodeId) -> Self::Succs<'_> {
        self.storage.succs(nid).chain(self.storage.preds(nid))
    }

    fn preds(&self, nid: NodeId) -> Self::Preds<'_> {
        self.storage.succs(nid).chain(self.storage.preds(nid))
    }
}

impl<'a, S> EdgeRef for UndirectedView<'a, S>
where
    S: EdgeRef<Dir = Directed>,
{
    type AllEdges<'b>
    where
        Self: 'b;

    type Incoming<'b>
    where
        Self: 'b;

    type Outgoing<'b>
    where
        Self: 'b;

    fn has_edge(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool {
        self.storage.has_edge(src, dst, eid) || self.storage.has_edge(dst, src, eid)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        todo!()
    }

    fn incoming(&self, nid: NodeId) -> Self::Incoming<'_> {
        todo!()
    }

    fn outgoing(&self, nid: NodeId) -> Self::Outgoing<'_> {
        todo!()
    }
}
