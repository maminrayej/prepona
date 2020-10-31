// use std::collections::HashSet;

// use crate::provide;

// pub struct Dfs {
//     stack: Vec<usize>,
//     visited: HashSet<usize>,
// }

// impl Dfs {
//     pub fn init(src_index: usize) -> Self {
//         Dfs {
//             stack: vec![src_index],
//             visited: HashSet::new(),
//         }
//     }

//     pub fn next<G, W>(&mut self, graph: &G) -> Option<usize>
//     where
//         G: provide::Graph<W> + provide::Neighbors,
//     {
//         if let Some(v_index) = self.stack.pop() {
//             let mut undiscovered_neighbors = graph
//                 .neighbors(v_index)
//                 .into_iter()
//                 .filter(|neighbor_id| self.is_discovered(neighbor_id))
//                 .collect::<Vec<usize>>();

//             self.stack.append(&mut undiscovered_neighbors);

//             self.visited.insert(v_index);

//             Some(v_index)
//         } else {
//             None
//         }
//     }

//     pub fn get_stack(&self) -> &Vec<usize> {
//         &self.stack
//     }

//     pub fn get_visited(&self) -> &HashSet<usize> {
//         &self.visited
//     }

//     pub fn is_discovered(&self, vertex_id: &usize) -> bool {
//         !self.visited.contains(vertex_id) && !self.stack.contains(vertex_id)
//     }

//     pub fn execute<G, W>(mut self, graph: &G) -> HashSet<usize>
//     where
//         G: provide::Graph<W> + provide::Neighbors,
//     {
//         while let Some(_) = &mut self.next(graph) {}

//         self.visited
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::graph::structs::SimpleGraph;
//     use crate::provide::*;
//     use crate::storage::Storage;

//     #[test]
//     fn one_vertex_directed_graph() {
//         // Given: directed graph with single vertex.
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, true);
//         graph.add_vertex();

//         // When: traversing graph using dfs algorithm.
//         let dfs = Dfs::init(0);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain only one item.
//         assert_eq!(visited.len(), 1);
//         // Visited set mut contain only the vertex itself.
//         assert!(visited.contains(&0));
//     }

//     #[test]
//     fn one_vertex_undirected_graph() {
//         // Given: undirected graph with single vertex.
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, false);
//         graph.add_vertex();

//         // When: traversing graph using dfs algorithm.
//         let dfs = Dfs::init(0);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain only one item.
//         assert_eq!(visited.len(), 1);
//         // Visited set mut contain only the vertex itself.
//         assert!(visited.contains(&0));
//     }

//     #[test]
//     fn acyclic_directed_graph() {
//         // Given: directed graph: a -> b.
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, true);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());

//         // When: traversing graph using dfs algorithm from vertex a.
//         let dfs = Dfs::init(a);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must contain both a and b.
//         assert!(vec![a, b].into_iter().all(|v_id| visited.contains(&v_id)));

//         // When: traversing graph using dfs algorithm from vertex b.
//         let dfs = Dfs::init(b);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain only one element.
//         assert_eq!(visited.len(), 1);
//         // Visited set must only contain b.
//         assert!(visited.contains(&b));
//     }

//     #[test]
//     fn acyclic_undirected_graph() {
//         // Given: undirected graph: a -- b.
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, false);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());

//         // When: traversing graph using dfs algorithm from vertex a.
//         let dfs = Dfs::init(a);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must contain both a and b.
//         assert!(vec![a, b].into_iter().all(|v_id| visited.contains(&v_id)));

//         // When: traversing graph using dfs algorithm from vertex b.
//         let dfs = Dfs::init(b);
//         let visited = dfs.execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must contain both a and b.
//         assert!(vec![a, b].into_iter().all(|v_id| visited.contains(&v_id)));
//     }

//     #[test]
//     fn directed_graph_with_cycle() {
//         // Given: directed graph:   a --> b
//         //                          ^     |
//         //                          |     |
//         //                          c <----
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, true);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         let c = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());
//         graph.add_edge(b, c, 1.into());
//         graph.add_edge(c, a, 1.into());

//         // When traversing graph from either a or b or c.
//         for vertex_id in 0..3 {
//             let visited = Dfs::init(vertex_id).execute(&graph);

//             // Then:
//             // Visited set must contain 3 elements.
//             assert_eq!(visited.len(), 3);
//             // Visited set must contain a and b and c.
//             assert!(vec![a, b, c]
//                 .into_iter()
//                 .all(|v_id| visited.contains(&v_id)));
//         }
//     }

//     #[test]
//     fn undirected_graph_with_cycle() {
//         // Given: undirected graph:    a --- b
//         //                             |     |
//         //                             c ----
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, false);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         let c = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());
//         graph.add_edge(b, c, 1.into());
//         graph.add_edge(c, a, 1.into());

//         // When traversing graph from either a or b or c.
//         for vertex_id in 0..3 {
//             let visited = Dfs::init(vertex_id).execute(&graph);

//             // Then:
//             // Visited set must contain 3 elements.
//             assert_eq!(visited.len(), 3);
//             // Visited set must contain a and b and c.
//             assert!(vec![a, b, c]
//                 .into_iter()
//                 .all(|v_id| visited.contains(&v_id)));
//         }
//     }

//     #[test]
//     fn disconnected_directed_graph() {
//         // Given: directed graph:   a ---> b   c ---> d
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, true);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         let c = graph.add_vertex();
//         let d = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());
//         graph.add_edge(c, d, 1.into());

//         // When: traversing graph with dfs algorithm from a.
//         let visited = Dfs::init(a).execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must only contain a and b.
//         assert!(vec![a, b].into_iter().all(|v_id| visited.contains(&v_id)));

//         // When: traversing graph with dfs algorithm from c.
//         let visited = Dfs::init(c).execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must contain c and d.
//         assert!(vec![c, d].into_iter().all(|v_id| visited.contains(&v_id)));
//     }

//     #[test]
//     fn disconnected_undirected_graph() {
//         // Given: undirected graph:   a --- b   c --- d
//         let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, false);
//         let a = graph.add_vertex();
//         let b = graph.add_vertex();
//         let c = graph.add_vertex();
//         let d = graph.add_vertex();
//         graph.add_edge(a, b, 1.into());
//         graph.add_edge(c, d, 1.into());

//         // When: traversing graph with dfs algorithm from a.
//         let visited = Dfs::init(a).execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must only contain a and b.
//         assert!(vec![a, b].into_iter().all(|v_id| visited.contains(&v_id)));

//         // When: traversing graph with dfs algorithm from c.
//         let visited = Dfs::init(c).execute(&graph);

//         // Then:
//         // Visited set must contain two elements.
//         assert_eq!(visited.len(), 2);
//         // Visited set must contain c and d.
//         assert!(vec![c, d].into_iter().all(|v_id| visited.contains(&v_id)));
//     }

//     #[test]
//     fn fully_connected_directed_graph() {
//         // Given: A fully connected graph with 5 vertices.
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

//         // When: traversing graph with dfs algorithm from each vertex.
//         for src_index in 0..5 {
//             let visited = Dfs::init(src_index).execute(&graph);

//             // Then:
//             // Visited set must contain 5 elements;
//             assert_eq!(visited.len(), 5);
//             // Visited set must contain all vertices.
//             assert!(vec![0, 1, 2, 3, 4]
//                 .into_iter()
//                 .all(|v_id| visited.contains(&v_id)));
//         }
//     }
// }
