mod iter;
pub use iter::*;

use crate::give::*;

pub struct ReverseView<'a, S> {
    storage: &'a S,
}

impl<'a, S> ReverseView<'a, S>
where
    S: Storage<Dir = Directed>,
{
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }
}

impl<'a, S> Storage for ReverseView<'a, S>
where
    S: Storage<Dir = Directed>,
{
    type Dir = Directed;

    // TODO: Fix this map type
    type Map = S::Map;

    fn idmap(&self) -> Self::Map {
        todo!()
    }
}

impl<'b, S> Node for ReverseView<'b, S>
where
    S: Node<Dir = Directed>,
{
    #[rustfmt::skip]
    type Nodes<'a> = S::Nodes<'a> where Self: 'a;

    #[rustfmt::skip]
    type Succs<'a> = S::Preds<'a> where Self: 'a;

    #[rustfmt::skip]
    type Preds<'a> = S::Succs<'a> where Self: 'a;

    fn has_node(&self, node: NodeID) -> bool {
        self.storage.has_node(node)
    }

    fn nodes(&self) -> Self::Nodes<'_> {
        self.storage.nodes()
    }

    fn succs(&self, node: NodeID) -> Self::Succs<'_> {
        self.storage.preds(node)
    }

    fn preds(&self, node: NodeID) -> Self::Preds<'_> {
        self.storage.succs(node)
    }
}

impl<'b, S> Edge for ReverseView<'b, S>
where
    S: Edge<Dir = Directed>,
{
    #[rustfmt::skip]
    type AllEdges<'a> = Edges<'a, S> where Self: 'a;

    #[rustfmt::skip]
    type Incoming<'a> = S::Outgoing<'a> where Self: 'a;

    #[rustfmt::skip]
    type Outgoing<'a> = S::Incoming<'a> where Self: 'a;

    fn has_edge(&self, src: NodeID, dst: NodeID) -> bool {
        self.storage.has_edge(dst, src)
    }

    fn edges(&self) -> Self::AllEdges<'_> {
        Edges {
            in_iter: self.storage.edges(),
        }
    }

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_> {
        self.storage.outgoing(node)
    }

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_> {
        self.storage.incoming(node)
    }
}
