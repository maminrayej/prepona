use std::collections::BinaryHeap;

use crate::provide::*;

struct Element(NodeID, usize);

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Prim<'a, S> {
    storage: &'a S,
}

impl<'a, S> Prim<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec(&self, weight: impl Fn(NodeID, NodeID) -> usize) {
        let node_count = self.storage.node_count();
        let idmap = self.storage.idmap();

        let mut in_set = vec![false; node_count];
        let mut heap = BinaryHeap::new();

        heap.push(Element(NodeID(0), 0));

        while let Some(Element(node, _w)) = heap.pop() {
            let node_vid = idmap[node];

            if in_set[node_vid] {
                continue;
            }

            for dst in self.storage.outgoing(node) {
                if in_set[node_vid] {
                    continue;
                }

                heap.push(Element(dst, weight(node, dst)));
            }

            in_set[node_vid] = true;
        }
    }
}
