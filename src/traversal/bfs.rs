use std::collections::{HashSet, VecDeque};

use crate::graph::Edge;
use crate::provide;

pub struct Bfs {
    queue: VecDeque<usize>,
    discovered: HashSet<usize>,
}

impl Bfs {
    pub fn init(src_index: usize) -> Self {
        let mut queue = VecDeque::with_capacity(1);
        queue.push_back(src_index);

        Bfs {
            queue,
            discovered: HashSet::new(),
        }
    }

    // pub fn next<G, W, E: Edge<W>>(&mut self, graph: &G) -> Option<usize>
    // where
    //     G: provide::Graph<W, E> + provide::Neighbors,
    // {
    //     if let Some(v_index) = self.queue.pop_front() {
    //         let undiscovered_neighbors = graph
    //             .neighbors(v_index)
    //             .iter()
    //             .filter(|&neighbor_index| {
    //                 !self.discovered.contains(neighbor_index)
    //                     && !self.queue.contains(neighbor_index)
    //             })
    //             .copied()
    //             .collect::<Vec<usize>>();

    //         self.queue.append(&mut undiscovered_neighbors.into());

    //         self.discovered.insert(v_index);

    //         Some(v_index)
    //     } else {
    //         None
    //     }
    // }
}
