// use std::collections::{HashSet, VecDeque};

// use crate::provide;
// pub struct Bfs {
//     queue: VecDeque<usize>,
//     discovered: HashSet<usize>,
// }

// impl Bfs {
//     pub fn init(src_index: usize) -> Self {
//         let mut queue = VecDeque::with_capacity(1);
//         queue.push_back(src_index);

//         Bfs {
//             queue,
//             discovered: HashSet::new(),
//         }
//     }

//     pub fn next<G, W>(&mut self, graph: &G) -> Option<usize>
//     where
//         G: provide::Graph<W> + provide::Neighbors,
//     {
//         if let Some(v_index) = self.queue.pop_front() {
//             let undiscovered_neighbors = graph
//                 .neighbors(v_index)
//                 .iter()
//                 .filter(|&neighbor_index| {
//                     !self.discovered.contains(neighbor_index)
//                         && !self.queue.contains(neighbor_index)
//                 })
//                 .copied()
//                 .collect::<Vec<usize>>();

//             self.queue.append(&mut undiscovered_neighbors.into());

//             self.discovered.insert(v_index);

//             Some(v_index)
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::graph::structs::SimpleGraph;
//     use crate::storage::Storage;
//     use crate::provide::*;

//     #[test]
//     fn dense_graph() {
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, true);
//         for _ in 0..5 {
//             graph.add_vertex();
//         }

//         for i in 0..5 {
//             for j in 0..5 {
//                 if i == j {
//                     continue;
//                 }
//                 graph.add_edge(i, j, 1.into());
//             }
//         }

//         for src_index in 0..5 {
//             let mut dfs = Bfs::init(src_index);
//             let mut count = 0usize;
//             while let Some(_) = dfs.next(&graph) {
//                 count += 1;
//             }

//             assert_eq!(count, 5);
//         }
//     }
// }
