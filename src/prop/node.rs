use crate::give::*;

pub trait NodeProp: Node {
    type Prop;

    #[rustfmt::skip]
    type NodeProps<'a>: Iterator<Item = (NodeID, &'a Self::Prop)> where Self: 'a;

    fn node_props(&self) -> Self::NodeProps<'_>;

    fn node_prop(&self, node: NodeID) -> &Self::Prop;

    fn node_prop_checked(&self, node: NodeID) -> Result<&Self::Prop, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.node_prop(node))
    }
}

pub trait NodePropMut: NodeProp {
    #[rustfmt::skip]
    type NodePropsMut<'a>: Iterator<Item = (NodeID, &'a mut Self::Prop)> where Self: 'a;

    fn node_props_mut(&mut self, node: NodeID) -> Self::NodePropsMut<'_>;

    fn node_prop_mut(&mut self, node: NodeID) -> &mut Self::Prop;

    fn node_prop_mut_checked(&mut self, node: NodeID) -> Result<&mut Self::Prop, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.node_prop_mut(node))
    }
}

pub trait AddNodeProp: NodeProp {
    fn insert_node_prop(&mut self, node: NodeID, prop: Self::Prop) -> bool;

    fn insert_node_prop_checked(&mut self, node: NodeID, prop: Self::Prop) -> Result<bool, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.insert_node_prop(node, prop))
    }
}

pub trait DelNodeProp: NodeProp {
    fn delete_node_prop(&mut self, node: NodeID) -> Self::Prop;

    fn delete_node_prop_checked(&mut self, node: NodeID) -> Result<Self::Prop, Error> {
        if !self.has_node(node) {
            return Err(Error::NodeNotFound(node));
        }

        Ok(self.delete_node_prop(node))
    }
}
