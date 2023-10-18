use std::collections::HashSet;

use crate::algo::visit::{Continue, Dfs, DfsEvent, VisitFlow};
use crate::provide::{Directed, EdgeId, EdgeRef, Id, NodeId};
use crate::view::{AcceptAll, Reverse, View};

pub struct Kosaraju<'a, S> {
    storage: &'a S,
}

impl<'a, S> Kosaraju<'a, S>
where
    S: EdgeRef<Dir = Directed>,
{
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(self) -> Vec<View<'a, S, HashSet<NodeId>, HashSet<EdgeId>>> {
        let mut scc = Vec::new();
        let mut finished = Vec::new();

        Dfs::new(self.storage, None.into_iter()).exec(|event| {
            if let DfsEvent::Finish(node) = event {
                finished.push(node.id());
            }

            VisitFlow::Continue(Continue::Noop)
        });

        let reversed = Reverse::new(self.storage, AcceptAll, AcceptAll);
        let mut node_set = HashSet::new();
        let mut edge_set = HashSet::new();
        Dfs::new(&reversed, finished.into_iter()).exec(|event| {
            match event {
                DfsEvent::Discover(node) => {
                    node_set.insert(node.id());
                }
                DfsEvent::Edge { edge, .. } => {
                    edge_set.insert(edge.id());
                }
                DfsEvent::End => scc.push(View::new(
                    self.storage,
                    std::mem::take(&mut node_set),
                    std::mem::take(&mut edge_set),
                )),
                _ => {}
            }

            VisitFlow::Continue(Continue::Noop)
        });

        scc
    }
}
