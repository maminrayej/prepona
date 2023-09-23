use std::collections::VecDeque;

use crate::provide::{EdgeId, EdgeRef, NodeId, Storage};

use super::{control, VisitFlow};

pub enum BfsEvent<'a, N, E> {
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
    },
}

pub struct Bfs<'a, S, I>
where
    S: Storage,
{
    storage: &'a S,
    idmap: S::Map,
    starts: I,

    discovered: Vec<bool>,
}

impl<'a, S, I> Bfs<'a, S, I>
where
    S: EdgeRef,
    I: Iterator<Item = NodeId>,
{
    pub fn new(storage: &'a S, starts: I) -> Self {
        let node_count = storage.node_count();

        Self {
            storage,
            starts,
            idmap: storage.idmap(),

            discovered: vec![false; node_count],
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
            self.discovered
                .iter()
                .position(|discovered| !discovered)
                .map(|idx| self.idmap[idx])
        })
    }

    pub fn exec<F>(self, f: F)
    where
        F: FnMut(BfsEvent<'a, S::Node, S::Edge>) -> VisitFlow,
    {
        self.internal_exec(f);
    }

    fn internal_exec<F>(mut self, mut f: F) -> VisitFlow
    where
        F: FnMut(BfsEvent<'a, S::Node, S::Edge>) -> VisitFlow,
    {
        /* TODO: guess a better initial capacity */
        let mut queue = VecDeque::with_capacity(0);

        while let Some(start_nid) = self.next_node() {
            let start = self.storage.node(start_nid);

            control!(f(BfsEvent::Begin(start_nid, start)));

            let start_idx = self.idmap[start_nid];
            self.discovered[start_idx] = true;

            control!(f(BfsEvent::Discover(start_nid, start)));

            queue.push_back((start, start_nid, self.storage.outgoing(start_nid)));

            while let Some((src, src_nid, mut outgoing)) = queue.pop_front() {
                for (dst_nid, eid, edge) in outgoing.by_ref() {
                    let dst = self.storage.node(dst_nid);
                    let dst_idx = self.idmap[dst_nid];

                    if self.discovered[dst_idx] {
                        continue;
                    }

                    self.discovered[dst_idx] = true;
                    control!(f(BfsEvent::Discover(dst_nid, dst)));

                    control!(f(BfsEvent::Edge {
                        src_nid,
                        src,
                        dst_nid,
                        dst,
                        eid,
                        edge,
                    }));

                    queue.push_back((dst, dst_nid, self.storage.outgoing(dst_nid)));
                }

                control!(f(BfsEvent::Finish(src_nid, src)));
            }

            control!(f(BfsEvent::End));
        }

        VisitFlow::Break(())
    }
}
