use std::collections::VecDeque;

use crate::provide::{EdgeRef, Id, NodeId, Storage};

use super::{control, VisitFlow};

pub enum BfsEvent<'a, N, E> {
    Begin(&'a N),
    Discover(&'a N),
    Finish(&'a N),
    End,
    Edge { src: &'a N, dst: &'a N, edge: &'a E },
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

            control!(f(BfsEvent::Begin(start)));

            let start_idx = self.idmap[start_nid];
            self.discovered[start_idx] = true;

            control!(f(BfsEvent::Discover(start)));

            queue.push_back((start, self.storage.outgoing(start_nid)));

            while let Some((src, mut outgoing)) = queue.pop_front() {
                for (dst, edge) in outgoing.by_ref() {
                    let dst_nid = dst.id();
                    let dst = self.storage.node(dst_nid);
                    let dst_idx = self.idmap[dst_nid];

                    if self.discovered[dst_idx] {
                        continue;
                    }

                    self.discovered[dst_idx] = true;
                    control!(f(BfsEvent::Discover(dst)));

                    control!(f(BfsEvent::Edge { src, dst, edge }));

                    queue.push_back((dst, self.storage.outgoing(dst_nid)));
                }

                control!(f(BfsEvent::Finish(src)));
            }

            control!(f(BfsEvent::End));
        }

        VisitFlow::Break(())
    }
}
