use std::collections::HashSet;

use crate::algo::visit::{Continue, Dfs, DfsEvent, VisitFlow};
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, Storage, Undirected};
use crate::view::View;

pub struct ConnectedComponents<'a, S>
where
    S: Storage<Dir = Undirected>,
{
    storage: &'a S,
}

impl<'a, S> ConnectedComponents<'a, S>
where
    S: Storage<Dir = Undirected>,
{
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }
}

impl<'a, S> ConnectedComponents<'a, S>
where
    S: EdgeRef<Dir = Undirected>,
{
    pub fn exec(self) -> Vec<View<'a, S, HashSet<NodeId>, HashSet<EdgeId>>> {
        let mut node_set = HashSet::new();
        let mut edge_set = HashSet::new();
        let mut ccs = Vec::new();

        Dfs::new(self.storage, None.into_iter()).exec(|event| {
            match event {
                DfsEvent::Discover(node) => {
                    node_set.insert(node.id());
                }
                DfsEvent::Edge { edge, .. } => {
                    edge_set.insert(edge.id());
                }
                DfsEvent::End => {
                    ccs.push(View::new(
                        self.storage,
                        std::mem::take(&mut node_set),
                        std::mem::take(&mut edge_set),
                    ));
                }
                _ => {}
            };

            VisitFlow::Continue(Continue::Noop)
        });

        ccs
    }
}
