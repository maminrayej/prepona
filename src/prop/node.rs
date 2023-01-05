use crate::provide::*;

pub trait NodeProp: Node {
    type Prop;

    fn node_prop(&self, node: NodeID) -> &Self::Prop;

    fn node_prop_checked(&self, node: NodeID) -> Result<&Self::Prop, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.node_prop(node))
    }
}

pub trait NodePropMut: NodeProp {
    fn node_prop_mut(&mut self, node: NodeID) -> &mut Self::Prop;

    fn node_prop_mut_checked(&mut self, node: NodeID) -> Result<&mut Self::Prop, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.node_prop_mut(node))
    }
}

pub trait AddNodeProp: NodeProp {
    fn insert_node_prop(&mut self, node: NodeID, prop: Self::Prop) -> bool;

    fn insert_node_prop_checked(&mut self, node: NodeID, prop: Self::Prop) -> Result<bool, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.insert_node_prop(node, prop))
    }
}

pub trait DelNodeProp: NodeProp {
    fn delete_node_prop(&mut self, node: NodeID) -> Self::Prop;

    fn delete_node_prop_checked(&mut self, node: NodeID) -> Result<Self::Prop, Error> {
        if !self.contains_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.delete_node_prop(node))
    }
}