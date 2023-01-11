use crate::provide::*;

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

    pub fn exec(&mut self, start: NodeID, cost_of: impl Fn(NodeID, NodeID) -> isize) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut costs = vec![INF; node_count];
        let mut used_edges = vec![None; node_count];

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
                }
            }
        }

        // TODO: Do negative cycle detection

        // TODO: Construct the shortest path tree
    }
}
