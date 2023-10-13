mod dijkstra;
use std::collections::HashMap;

pub use dijkstra::*;

use crate::filter::{Filter, FilteredAllEdges, FilteredEdges, FilteredNodes, View};
use crate::provide::{EdgeId, EdgeRef, NodeId, NodeRef, Storage};

pub struct PathView<'a, S, C, NF, EF> {
    view: View<'a, S, NF, EF>,
    dist: HashMap<NodeId, C>,
}

impl<'a, S, C, NF, EF> PathView<'a, S, C, NF, EF> {
    pub fn new(storage: &'a S, nfilter: NF, efilter: EF, dist: HashMap<NodeId, C>) -> Self
    where
        S: NodeRef,
        NF: Filter<Item = NodeId>,
    {
        Self {
            view: View::new(storage, nfilter, efilter),
            dist,
        }
    }

    pub fn dist(&self, nid: NodeId) -> &C {
        &self.dist[&nid]
    }
}

impl<'a, S, C, NF, EF> Storage for PathView<'a, S, C, NF, EF>
where
    S: Storage,
{
    type Node = S::Node;
    type Edge = S::Edge;
    type Dir = S::Dir;
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        self.view.idmap()
    }
}

impl<'a, S, C, NF, EF> NodeRef for PathView<'a, S, C, NF, EF>
where
    S: NodeRef,
    NF: Filter<Item = NodeId>,
{
    type Nodes<'b> = FilteredNodes<'b, S, NF, S::Nodes<'b>> where Self: 'b;
    type Succs<'b> = FilteredNodes<'b, S, NF, S::Succs<'b>> where Self: 'b;
    type Preds<'b> = FilteredNodes<'b, S, NF, S::Preds<'b>> where Self: 'b;

    fn has_node(&self, nid: NodeId) -> bool {
        self.view.has_node(nid)
    }

    fn node_count(&self) -> usize {
        self.view.node_count()
    }

    fn node(&self, nid: NodeId) -> &Self::Node {
        self.view.node(nid)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.view.nodes()
    }

    fn succs(&self, nid: NodeId) -> Self::Succs<'_> {
        self.view.succs(nid)
    }

    fn preds(&self, nid: NodeId) -> Self::Preds<'_> {
        self.view.preds(nid)
    }
}

impl<'a, S, C, NF, EF> EdgeRef for PathView<'a, S, C, NF, EF>
where
    S: EdgeRef,
    NF: Filter<Item = NodeId>,
    EF: Filter<Item = EdgeId>,
{
    type AllEdges<'b> = FilteredAllEdges<'b, S, NF, EF, S::AllEdges<'b>> where Self: 'b;
    type Incoming<'b> = FilteredEdges<'b, S, NF, EF, S::Incoming<'b>> where Self: 'b;
    type Outgoing<'b> = FilteredEdges<'b, S, NF, EF, S::Outgoing<'b>> where Self: 'b;

    fn has_edge(&self, src: NodeId, dst: NodeId, eid: EdgeId) -> bool {
        self.view.has_edge(src, dst, eid)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        self.view.edges()
    }

    fn incoming(&self, nid: NodeId) -> Self::Incoming<'_> {
        self.view.incoming(nid)
    }

    fn outgoing(&self, nid: NodeId) -> Self::Outgoing<'_> {
        self.view.outgoing(nid)
    }
}
