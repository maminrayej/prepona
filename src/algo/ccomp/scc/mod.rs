use std::cmp::min;
use std::collections::HashSet;

use crate::algo::visit::{Continue, Dfs, DfsEvent, VisitFlow};
use crate::provide::{EdgeId, EdgeRef, Id, NodeId, NodeRef, Storage};
use crate::view::View;

pub struct Tarjan<'a, S>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,

    id: usize,
    id_of: Vec<usize>,
    ll_of: Vec<usize>,
    stack: Vec<(NodeId, usize, Option<EdgeId>)>,
    on_stack: Vec<bool>,
    parent_of: Vec<usize>,
    last_used_edge: Option<EdgeId>,
}

impl<'a, S> Tarjan<'a, S>
where
    S: NodeRef,
{
    pub fn new(storage: &'a S) -> Self {
        let node_count = storage.node_count();

        Self {
            storage,
            idmap: storage.idmap(),
            id: 0,
            id_of: vec![usize::MAX; node_count],
            ll_of: vec![usize::MAX; node_count],
            stack: vec![],
            on_stack: vec![false; node_count],
            parent_of: vec![usize::MAX; node_count],
            last_used_edge: None,
        }
    }
}

impl<'a, S> Tarjan<'a, S>
where
    S: EdgeRef,
{
    pub fn exec(mut self) -> Vec<View<'a, S, HashSet<NodeId>, HashSet<EdgeId>>> {
        let mut sccs = vec![];

        Dfs::new(self.storage, None.into_iter()).exec(|event| {
            match event {
                DfsEvent::Discover(node) => {
                    let node_idx = self.idmap[node.id()];

                    self.id_of[node_idx] = self.id;
                    self.ll_of[node_idx] = self.id;

                    self.id += 1;

                    self.stack
                        .push((node.id(), node_idx, self.last_used_edge.take()));

                    self.on_stack[node_idx] = true;
                }
                DfsEvent::Finish(node) => {
                    let node_idx = self.idmap[node.id()];

                    if self.id_of[node_idx] == self.ll_of[node_idx] {
                        let mut node_set = HashSet::new();
                        let mut edge_set = HashSet::new();

                        while let Some((nid, idx, eid)) = self.stack.pop() {
                            node_set.insert(nid);

                            if node_idx == idx {
                                break;
                            }

                            edge_set.insert(eid.unwrap());
                        }

                        sccs.push(View::new(self.storage, node_set, edge_set));
                    } else {
                        let parent_idx = self.parent_of[node_idx];

                        self.ll_of[parent_idx] = min(self.ll_of[parent_idx], self.ll_of[node_idx]);
                    }
                }
                DfsEvent::Edge { src, dst, edge, .. } => {
                    let src_idx = self.idmap[src.id()];
                    let dst_idx = self.idmap[dst.id()];

                    if self.id_of[dst_idx] == usize::MAX {
                        self.parent_of[dst_idx] = src_idx;
                        self.last_used_edge = Some(edge.id());
                    } else if self.on_stack[dst_idx] {
                        self.ll_of[src_idx] = min(self.ll_of[src_idx], self.id_of[dst_idx]);
                    }
                }
                _ => {}
            }

            VisitFlow::Continue(Continue::Noop)
        });

        sccs
    }
}
