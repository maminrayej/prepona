use crate::provide::{Direction, EdgeId, EdgeRef, NodeId, NodeRef, Storage};

use super::{control, VisitFlow, UNKNOWN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Forward,
    Tree,
    Back,
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DfsEvent<'a, N, E> {
    Begin(NodeId, &'a N),
    Discover(NodeId, &'a N),
    Finish(NodeId, &'a N),
    End,
    Edge {
        src_nid: NodeId,
        src: &'a N,
        dst_nid: NodeId,
        dst: &'a N,
        eid: EdgeId,
        edge: &'a E,
        ety: EdgeType,
    },
}

#[derive(Debug)]
pub struct Dfs<'a, S, I>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
    starts: I,

    time: usize,
    discover: Vec<usize>,
    finished: Vec<usize>,
    parent: Vec<usize>,
}

impl<'a, S, I> Dfs<'a, S, I>
where
    S: NodeRef,
    I: Iterator<Item = NodeId>,
{
    /* TODO: accept IntoIterator instead of Iterator */
    pub fn new(storage: &'a S, starts: I) -> Self {
        let node_count = storage.node_count();

        Self {
            storage,
            starts,
            idmap: storage.idmap(),

            time: 0,
            discover: vec![UNKNOWN; node_count],
            finished: vec![UNKNOWN; node_count],
            parent: vec![UNKNOWN; node_count],
        }
    }

    fn next_node(&mut self) -> Option<NodeId> {
        /*
            TODO: iterating over all nodes everytime we're looking
            for a start node may not be efficient. on one hand, if
            the graph is connected, this method will be called once,
            and will complete in O(1). on the other hand, if the graph
            is disconnected (no node is connected to any other node),
            it will run in O(n^2) overall. a more careful analysis of
            the runtime maybe required.
        */
        self.starts.next().or_else(|| {
            self.discover
                .iter()
                .position(|t| *t == UNKNOWN)
                .map(|idx| self.idmap[idx])
        })
    }

    fn tick(&mut self) {
        self.time += 1;
    }

    fn is_discovered(&self, node_idx: usize) -> bool {
        self.discover[node_idx] != UNKNOWN
    }
    fn is_finished(&self, node_idx: usize) -> bool {
        self.finished[node_idx] != UNKNOWN
    }

    fn edge_type(&self, src_idx: usize, dst_idx: usize) -> EdgeType {
        if !self.is_discovered(dst_idx) {
            EdgeType::Tree
        } else if !self.is_finished(dst_idx) {
            EdgeType::Back
        } else if self.discover[src_idx] < self.discover[dst_idx] {
            EdgeType::Forward
        } else {
            EdgeType::Cross
        }
    }
}

impl<'a, S, I> Dfs<'a, S, I>
where
    S: EdgeRef,
    I: Iterator<Item = NodeId>,
{
    pub fn exec<F>(self, f: F)
    where
        F: FnMut(DfsEvent<'a, S::Node, S::Edge>) -> VisitFlow,
    {
        self.internal_exec(f);
    }

    fn internal_exec<F>(mut self, mut f: F) -> VisitFlow
    where
        F: FnMut(DfsEvent<'a, S::Node, S::Edge>) -> VisitFlow,
    {
        /*
            TODO: if user specifies a max_depth,
            we can run the algorithm using a single
            allocation by calling with_capacity(self.max_depth)
        */
        let mut stack = Vec::with_capacity(0);

        while let Some(start_nid) = self.next_node() {
            let start = self.storage.node(start_nid);

            control!(f(DfsEvent::Begin(start_nid, start)));

            self.tick();
            let start_idx = self.idmap[start_nid];
            self.discover[start_idx] = self.time;

            control!(f(DfsEvent::Discover(start_nid, start)));

            stack.push((
                start,
                start_nid,
                start_idx,
                0,
                self.storage.outgoing(start_nid),
            ));

            /* start the DFS algorithm from the `start` node */
            while let Some((src, src_nid, src_idx, depth, mut outgoing)) = stack.pop() {
                if let Some((dst_nid, eid, edge)) = outgoing.next() {
                    let dst = self.storage.node(dst_nid);
                    let dst_idx = self.idmap[dst_nid];

                    /* TODO: see if you can get rid of this condition */
                    if S::Dir::is_undirected() && self.parent[src_idx] == dst_idx {
                        stack.push((src, src_nid, src_idx, depth, outgoing));
                        continue;
                    }

                    let ety = self.edge_type(src_idx, dst_idx);
                    if let EdgeType::Tree = ety {
                        self.tick();
                        self.parent[dst_idx] = src_idx;
                        self.discover[dst_idx] = self.time;

                        control!(f(DfsEvent::Discover(dst_nid, dst)));

                        stack.push((src, src_nid, src_idx, depth, outgoing));
                        stack.push((
                            dst,
                            dst_nid,
                            dst_idx,
                            depth + 1,
                            self.storage.outgoing(dst_nid),
                        ));
                    } else {
                        stack.push((src, src_nid, src_idx, depth, outgoing));
                    }

                    control!(f(DfsEvent::Edge {
                        src_nid,
                        src,
                        dst_nid,
                        dst,
                        eid,
                        edge,
                        ety,
                    }));
                } else {
                    self.tick();
                    self.finished[src_idx] = self.time;

                    control!(f(DfsEvent::Finish(src_nid, src)));
                }
            }

            control!(f(DfsEvent::End));
        }

        VisitFlow::Break(())
    }
}
