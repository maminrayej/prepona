use crate::give::{Error, Node, NodeID};

pub trait Edge: Node {
    #[rustfmt::skip]
    type AllEdges<'a>: Iterator<Item = (NodeID, NodeID)> where Self: 'a;
    #[rustfmt::skip]
    type Incoming<'a>: Iterator<Item = NodeID> where Self: 'a;
    #[rustfmt::skip]
    type Outgoing<'a>: Iterator<Item = NodeID> where Self: 'a;

    // >>> Required Functions <<< //
    fn has_edge(&self, src: NodeID, dst: NodeID) -> bool;

    fn edges(&self) -> Self::AllEdges<'_>;

    fn incoming(&self, node: NodeID) -> Self::Incoming<'_>;

    fn outgoing(&self, node: NodeID) -> Self::Outgoing<'_>;

    // >>> Provided Functions <<< //
    fn out_deg(&self, node: NodeID) -> usize {
        self.outgoing(node).count()
    }

    fn in_deg(&self, node: NodeID) -> usize {
        self.incoming(node).count()
    }

    // >>> Checked Functions <<< //
    fn has_edge_checked(&self, src: NodeID, dst: NodeID) -> Result<bool, Error> {
        if !self.has_node(src) {
            return Err(Error::NodeNotFound(src));
        } else if !self.has_node(dst) {
            return Err(Error::NodeNotFound(dst));
        }

        Ok(self.has_edge(src, dst))
    }

    fn incoming_checked(&self, node: NodeID) -> Result<Self::Incoming<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.incoming(node))
    }

    fn outgoing_checked(&self, node: NodeID) -> Result<Self::Outgoing<'_>, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.outgoing(node))
    }

    fn out_deg_checked(&self, node: NodeID) -> Result<usize, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.out_deg(node))
    }

    fn in_deg_checked(&self, node: NodeID) -> Result<usize, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.in_deg(node))
    }
}

pub trait AddEdge: Edge {
    fn add_edge(&mut self, src: NodeID, dst: NodeID);

    fn add_edge_checked(&mut self, src: NodeID, dst: NodeID) -> Result<(), Error> {
        if !self.has_node(src) {
            return Err(Error::NodeNotFound(src));
        } else if !self.has_node(dst) {
            return Err(Error::NodeNotFound(dst));
        }

        #[allow(clippy::unit_arg)]
        Ok(self.add_edge(src, dst))
    }
}

pub trait DelEdge: Edge {
    fn del_edge(&mut self, src: NodeID, dst: NodeID);

    fn del_edge_checked(&mut self, src: NodeID, dst: NodeID) -> Result<(), Error> {
        if !self.has_edge_checked(src, dst)? {
            return Err(Error::EdgeNotFound(src, dst));
        }

        #[allow(clippy::unit_arg)]
        Ok(self.del_edge(src, dst))
    }
}
