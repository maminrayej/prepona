use rayon::prelude::ParallelIterator;

use crate::prop::NodeProp;
use crate::provide::*;

#[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
pub trait NodePropPar: NodeProp {
    #[rustfmt::skip]
    type NodePropsPar<'a>: ParallelIterator<Item = (NodeID, &'a Self::Prop)> where Self: 'a;

    fn node_props_par(&self) -> Self::NodePropsPar<'_>;
}

#[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
pub trait NodePropMutPar: NodeProp {
    #[rustfmt::skip]
    type NodePropsMutPar<'a>: ParallelIterator<Item = (NodeID, &'a mut Self::Prop)> where Self: 'a;

    fn node_props_mut_par(&mut self) -> Self::NodePropsMutPar<'_>;
}
