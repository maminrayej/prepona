use crate::algo::visit::macros::on_event;
use crate::algo::visit::ControlFlow;
use crate::provide::*;

const INF: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Forward,
    Tree,
    Back,
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DfsEvent {
    Begin(NodeID),
    Discover(NodeID),
    Finish(NodeID),
    End(NodeID),
    VisitEdge(NodeID, NodeID, EdgeType),
}

#[derive(Debug)]
pub struct Dfs<'a, S: Storage> {
    storage: &'a S,
    idmap: S::Map,
    starters: Vec<NodeID>,

    time: usize,
    discover: Vec<usize>,
    finished: Vec<usize>,
    parent: Vec<usize>,
}

impl<'a, S> Dfs<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self::with_starters(storage, vec![])
    }

    pub fn with_starters(storage: &'a S, starters: Vec<NodeID>) -> Self {
        let node_count = storage.node_count();

        Self {
            storage,
            idmap: storage.idmap(),
            starters,

            time: 0,
            discover: vec![INF; node_count],
            finished: vec![INF; node_count],
            parent: vec![INF; node_count],
        }
    }

    #[rustfmt::skip]
    fn next_start(&mut self) -> Option<NodeID> {
        if !self.starters.is_empty() {
            Some(self.starters.swap_remove(0))
        } else {
            self.discover.iter().position(|t| *t == INF).map(|i| self.idmap[i])
        }
    }

    fn type_of(&self, src: usize, dst: usize) -> EdgeType {
        if !self.is_discovered(dst) {
            EdgeType::Tree
        } else if self.is_discovered(dst) && !self.is_finished(dst) {
            EdgeType::Back
        } else if self.discover[src] < self.discover[dst] {
            EdgeType::Forward
        } else {
            EdgeType::Cross
        }
    }

    fn is_discovered(&self, id: usize) -> bool {
        self.discover[id] != INF
    }

    fn is_finished(&self, id: usize) -> bool {
        self.finished[id] != INF
    }

    pub fn execute(&mut self, callback: impl FnMut(DfsEvent) -> ControlFlow) {
        self._execute(callback);
    }

    fn _execute(&mut self, mut callback: impl FnMut(DfsEvent) -> ControlFlow) -> ControlFlow {
        while let Some(start) = self.next_start() {
            let start_vid = self.idmap[start];

            self.time += 1;
            self.discover[start_vid] = self.time;

            on_event!(callback(DfsEvent::Discover(start)));

            let mut stack = vec![(start, start_vid, 0, self.storage.succs(start))];

            while let Some((src, src_vid, depth, mut succs)) = stack.pop() {
                if let Some(dst) = succs.next() {
                    let dst_vid = self.idmap[dst];

                    #[rustfmt::skip]
                    if S::Dir::is_undirected() && (self.parent[src_vid] == dst_vid || self.is_finished(dst_vid)) {
                        stack.push((src, src_vid, depth, succs));
                        continue;
                    };

                    let edge_type = self.type_of(src_vid, dst_vid);

                    match edge_type {
                        EdgeType::Tree => {
                            self.parent[dst_vid] = src_vid;
                            self.time += 1;
                            self.discover[dst_vid] = self.time;

                            on_event!(callback(DfsEvent::Discover(dst)));

                            stack.push((src, src_vid, depth, succs));
                            stack.push((dst, dst_vid, depth + 1, self.storage.succs(dst)));
                        }
                        _ => {
                            stack.push((src, src_vid, depth, succs));
                        }
                    }
                } else {
                    self.time += 1;
                    self.finished[src_vid] = self.time;

                    on_event!(callback(DfsEvent::Finish(src)));
                }
            }
            on_event!(callback(DfsEvent::End(start)));
        }
        ControlFlow::Break(())
    }
}
