use crate::provide::*;

pub trait EdgeProp: Edge {
    type Prop;

    fn edge_prop(&self, src: NodeID, dst: NodeID) -> &Self::Prop;

    fn edge_prop_checked(&self, src: NodeID, dst: NodeID) -> Result<&Self::Prop, Error> {
        if !self.contains_edge(src, dst) {
            return Err(Error::EdgeNotFound(src, dst));
        }

        Ok(self.edge_prop(src, dst))
    }
}

pub trait EdgePropMut: EdgeProp {
    fn edge_prop_mut(&mut self, src: NodeID, dst: NodeID) -> &mut Self::Prop;

    fn edge_prop_mut_checked(
        &mut self,
        src: NodeID,
        dst: NodeID,
    ) -> Result<&mut Self::Prop, Error> {
        if !self.contains_edge(src, dst) {
            return Err(Error::EdgeNotFound(src, dst));
        }

        Ok(self.edge_prop_mut(src, dst))
    }
}

pub trait AddEdgeProp: EdgeProp {
    fn insert_edge_prop(&mut self, src: NodeID, dst: NodeID, prop: Self::Prop) -> bool;

    fn insert_edge_prop_checked(
        &mut self,
        src: NodeID,
        dst: NodeID,
        prop: Self::Prop,
    ) -> Result<bool, Error> {
        if !self.contains_edge(src, dst) {
            return Err(Error::EdgeNotFound(src, dst));
        }

        Ok(self.insert_edge_prop(src, dst, prop))
    }
}

pub trait DelEdgeProp: EdgeProp {
    fn delete_edge_prop(&mut self, src: NodeID, dst: NodeID) -> Self::Prop;

    fn delete_edge_prop_checked(&mut self, src: NodeID, dst: NodeID) -> Result<Self::Prop, Error> {
        if !self.contains_edge(src, dst) {
            return Err(Error::EdgeNotFound(src, dst));
        }

        Ok(self.delete_edge_prop(src, dst))
    }
}
