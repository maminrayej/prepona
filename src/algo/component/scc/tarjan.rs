use std::cmp;

use crate::algo::visit::{Continue, ControlFlow, Dfs, DfsEvent, EdgeType};
use crate::provide::*;

const INF: usize = usize::MAX;

pub struct TarjanSCC<'a, S> {
    storage: &'a S,
}

impl<'a, S> TarjanSCC<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(&self) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut next_id = 1;
        let mut id_of = vec![0; node_count];
        let mut low_link = vec![0; node_count];
        let mut parent = vec![INF; node_count];
        let mut stack = vec![];
        let mut sccs = vec![];

        Dfs::init(self.storage).execute(|event| {
            match event {
                DfsEvent::Discover(node) => {
                    let node_vid = idmap[node];

                    id_of[node_vid] = next_id;
                    low_link[node_vid] = next_id;

                    next_id += 1;
                    stack.push(node);
                }
                DfsEvent::Finish(node) => {
                    let node_vid = idmap[node];
                    let node_low_link = low_link[node_vid];
                    let node_id = id_of[node_vid];

                    if node_id == node_low_link {
                        let idx = stack.iter().position(|n| *n == node).unwrap();

                        let cc: Vec<NodeID> = stack.drain(idx..).collect();

                        sccs.push(cc);
                    } else {
                        let p_vid = parent[node_vid];
                        let p_low_link = low_link[p_vid];

                        if p_vid != INF {
                            low_link[p_vid] = cmp::min(node_low_link, p_low_link);
                        }
                    }
                }
                DfsEvent::VisitEdge(src, dst, edge_type) => match edge_type {
                    EdgeType::Tree => {
                        let src_vid = idmap[src];
                        let dst_vid = idmap[dst];

                        parent[dst_vid] = src_vid;
                    }
                    EdgeType::Back => {
                        let src_vid = idmap[src];
                        let dst_vid = idmap[dst];

                        let s_low_link = low_link[src_vid];
                        let d_low_link = low_link[dst_vid];

                        low_link[src_vid] = cmp::min(s_low_link, d_low_link);
                    }
                    _ => {}
                },
                _ => {}
            }

            ControlFlow::Continue(Continue::Noop)
        });
    }
}
