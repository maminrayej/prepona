use std::collections::HashSet;

use crate::algo::visit::{Continue, ControlFlow, Dfs, DfsEvent};
use crate::give::*;
use crate::view::{AllEdgeSelector, NodeSetSelector, View};

pub struct ConnectedComponents<'a, S> {
    storage: &'a S,
}

impl<'a, S> ConnectedComponents<'a, S>
where
    S: Edge,
{
    pub fn init(storage: &'a S) -> Self {
        ConnectedComponents { storage }
    }

    pub fn exec(&self) -> Vec<View<'a, S, NodeSetSelector<S>, AllEdgeSelector<S>>> {
        let mut ccs = vec![];

        let mut cc = HashSet::new();

        Dfs::init(self.storage).exec(|event| {
            match event {
                DfsEvent::Discover(node) => {
                    cc.insert(node);
                }
                DfsEvent::End(_) => {
                    ccs.push(cc.clone());
                    cc.clear();
                }
                _ => {}
            }

            ControlFlow::Continue(Continue::Noop)
        });

        ccs.into_iter()
            .map(|cc| {
                View::new(
                    self.storage,
                    NodeSetSelector::new(cc),
                    AllEdgeSelector::new(),
                )
            })
            .collect()
    }
}
