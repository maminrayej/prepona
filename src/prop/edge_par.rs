use rayon::prelude::ParallelIterator;

use crate::prop::EdgeProp;
use crate::provide::*;

pub trait EdgePropPar: EdgeProp {
    #[rustfmt::skip]
    type EdgePropsPar<'a>: ParallelIterator<Item = (NodeID, &'a Self::Prop)> where Self: 'a;

    fn edge_props_par(&self) -> Self::EdgePropsPar<'_>;
}

pub trait EdgePropMutPar: EdgeProp {
    #[rustfmt::skip]
    type EdgePropsMutPar<'a>: ParallelIterator<Item = (NodeID, &'a mut Self::Prop)> where Self: 'a;

    fn edge_props_mut_par(&mut self) -> Self::EdgePropsMutPar<'_>;
}
