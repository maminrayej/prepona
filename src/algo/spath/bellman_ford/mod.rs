mod view;
pub use view::BellmanFordSPT;

use std::collections::HashSet;

use crate::give::*;
use crate::view::{EdgeSetSelector, NodeSetSelector, View};

const INF: isize = isize::MAX;

pub struct BellmanFord<'a, S> {
    storage: &'a S,
}

impl<'a, S> BellmanFord<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        BellmanFord { storage }
    }

    pub fn exec(
        &mut self,
        start: NodeID,
        cost_of: impl Fn(NodeID, NodeID) -> isize,
    ) -> BellmanFordSPT<'a, S> {
        let node_count = self.storage.node_count();

        let idmap = self.storage.idmap();

        let mut costs = vec![INF; node_count];
        let mut used_edges = vec![None; node_count];

        let mut changed_nodes = HashSet::new();

        let start_vid = idmap[start];

        costs[start_vid] = 0;

        for _ in 1..node_count {
            for (src, dst) in self.storage.edges() {
                let src_vid = idmap[src];
                let dst_vid = idmap[dst];

                let src_cost = costs[src_vid];
                let dst_cost = costs[dst_vid];

                let new_cost = src_cost + cost_of(src, dst);

                if new_cost < dst_cost {
                    costs[dst_vid] = new_cost;
                    used_edges[dst_vid] = Some((src, dst));

                    changed_nodes.insert(src);
                    changed_nodes.insert(dst);
                }
            }
        }

        for (src, dst) in self.storage.edges() {
            let src_vid = idmap[src];
            let dst_vid = idmap[dst];

            let src_cost = costs[src_vid];
            let dst_cost = costs[dst_vid];

            let new_cost = src_cost + cost_of(src, dst);

            if new_cost < dst_cost {
                panic!("Graph contains negative cycles");
            }
        }

        let edge_set = used_edges.into_iter().filter_map(|e| e).collect();

        let node_set = changed_nodes;

        let cost_map = costs
            .into_iter()
            .enumerate()
            .map(|(i, cost)| (idmap[i], cost))
            .collect();

        BellmanFordSPT {
            view: View::new(
                self.storage,
                NodeSetSelector::new(node_set),
                EdgeSetSelector::new(edge_set),
            ),
            cost: cost_map,
        }
    }
}
