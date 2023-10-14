use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::provide::{EdgeRef, Id, NodeId, Storage};

pub struct FloydWarshall<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
}

impl<'a, S> FloydWarshall<'a, S>
where
    S: EdgeRef,
{
    pub fn new(storage: &'a S) -> Self {
        Self {
            storage,
            idmap: storage.idmap(),
        }
    }

    pub fn exec<F>(self, cost_fn: F) -> Result<HashMap<(NodeId, NodeId), isize>>
    where
        F: Fn(&'a S::Node, &'a S::Node, &'a S::Edge) -> isize,
    {
        let node_count = self.storage.node_count();

        let mut dist = vec![vec![isize::MAX; node_count]; node_count];

        for idx in 0..node_count {
            dist[idx][idx] = 0;
        }

        for (src, dst, edge) in self.storage.edges() {
            let src_idx = self.idmap[src.id()];
            let dst_idx = self.idmap[dst.id()];

            dist[src_idx][dst_idx] = cost_fn(src, dst, edge);
        }

        for k in 0..node_count {
            for src_idx in 0..node_count {
                for dst_idx in 0..node_count {
                    let cost_through_k = dist[src_idx][k].saturating_add(dist[k][dst_idx]);

                    if cost_through_k < dist[src_idx][dst_idx] {
                        dist[src_idx][dst_idx] = cost_through_k;
                    }
                }
            }
        }

        for idx in 0..node_count {
            if dist[idx][idx] < 0 {
                return Err(Error::NegativeCycle);
            }
        }

        let mut dist_map = HashMap::new();

        for src_idx in 0..node_count {
            let src = self.idmap[src_idx];

            for dst_idx in 0..node_count {
                let dst = self.idmap[dst_idx];

                if dist[src_idx][dst_idx] != isize::MAX {
                    dist_map.insert((src, dst), dist[src_idx][dst_idx]);
                }
            }
        }

        Ok(dist_map)
    }
}
