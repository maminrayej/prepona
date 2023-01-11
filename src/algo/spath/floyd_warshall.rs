use crate::provide::*;

const INF: isize = isize::MAX;

pub struct FloydWarshall<'a, S> {
    storage: &'a S,
}

impl<'a, S> FloydWarshall<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        FloydWarshall { storage }
    }

    pub fn exec(&self, cost_of: impl Fn(NodeID, NodeID) -> isize) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut dist = vec![vec![INF; node_count]; node_count];

        for i in 0..dist.len() {
            dist[i][i] = 0;
        }

        for src in self.storage.nodes() {
            let src_vid = idmap[src];

            for dst in self.storage.outgoing(src) {
                let dst_vid = idmap[dst];

                dist[src_vid][dst_vid] = cost_of(src, dst);
            }
        }

        for k in 0..node_count {
            for src in self.storage.nodes() {
                let src_vid = idmap[src];

                for dst in self.storage.nodes() {
                    let dst_vid = idmap[dst];

                    let cost_through_k = dist[src_vid][k] + dist[k][dst_vid];
                    let direct_cost = dist[src_vid][dst_vid];

                    if cost_through_k < direct_cost {
                        dist[src_vid][dst_vid] = cost_through_k;
                    }
                }
            }

            for i in 0..dist.len() {
                if dist[i][i] < 0 {
                    panic!("Graph contains negative cycle");
                }
            }
        }
    }
}
