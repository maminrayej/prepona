use std::marker::PhantomData;

use crate::give::NodeID;
use crate::view::Selector;

pub struct AllNodeSelector<S> {
    phantom_s: PhantomData<S>,
}

impl<S> AllNodeSelector<S> {
    pub fn new() -> Self {
        Self {
            phantom_s: PhantomData,
        }
    }
}

impl<S> Selector for AllNodeSelector<S> {
    type Storage = S;

    type Element = NodeID;

    fn select(&self, _storage: &Self::Storage, _element: &Self::Element) -> bool {
        true
    }
}

pub struct AllEdgeSelector<S> {
    phantom_s: PhantomData<S>,
}

impl<S> AllEdgeSelector<S> {
    pub fn new() -> Self {
        Self {
            phantom_s: PhantomData,
        }
    }
}

impl<S> Selector for AllEdgeSelector<S> {
    type Storage = S;

    type Element = (NodeID, NodeID);

    fn select(&self, _storage: &Self::Storage, _element: &Self::Element) -> bool {
        true
    }
}
