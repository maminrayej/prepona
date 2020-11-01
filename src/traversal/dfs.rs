use crate::provide;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Gray,
    Black,
}

pub struct Dfs {
    stack: Vec<usize>,
    colors: Vec<Color>,
    discovered: Vec<usize>,
    finished: Vec<usize>,
    time: usize,
    id_map: provide::IdMap,
}

impl Dfs {
    pub fn init<G>(graph: &G) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        let vertex_count = graph.vertex_count();

        Dfs {
            stack: vec![],
            colors: vec![Color::White; vertex_count],
            discovered: vec![],
            finished: vec![],
            time: 0,
            id_map: graph.continuos_id_map(),
        }
    }

    pub fn execute<G>(
        &mut self,
        graph: &G,
        on_start: &dyn Fn(usize),
        on_white: &dyn Fn(usize),
        on_gray: &dyn Fn(usize),
        on_black: &dyn Fn(usize),
    ) where
        G: provide::Vertices + provide::Neighbors,
    {
        while let Some((virt_start_id, _)) = self
            .colors
            .iter()
            .enumerate()
            .find(|(_, color)| **color == Color::White)
        {
            // On start.
            self.stack.push(virt_start_id);
            on_start(virt_start_id);

            while let Some(virt_id) = self.stack.pop() {
                match self.colors[virt_id] {
                    Color::White => {
                        self.time += 1;

                        // On white.
                        self.discovered[virt_id] = self.time;
                        on_white(virt_id);

                        self.colors[virt_id] = Color::Gray;

                        let real_id = self.id_map.get_virt_to_real(virt_id).unwrap();

                        let mut undiscovered_neighbors = graph
                            .neighbors(real_id)
                            .into_iter()
                            .filter(|n_id| self.colors[*n_id] == Color::White)
                            .map(|real_id| self.id_map.get_real_to_virt(real_id).unwrap())
                            .collect::<Vec<usize>>();

                        self.stack.push(virt_id);
                        self.stack.append(&mut undiscovered_neighbors);
                    }
                    Color::Gray => {
                        // On gray.
                        self.colors[virt_id] = Color::Black;
                        on_gray(virt_id);
                    }
                    Color::Black => {
                        // On black.
                        self.time += 1;
                        self.finished[virt_id] = self.time;
                        on_black(virt_id);
                    }
                };
            }
        }
    }

    pub fn get_stack(&self) -> &Vec<usize> {
        &self.stack
    }

    pub fn get_colors(&self) -> &Vec<Color> {
        &self.colors
    }

    pub fn get_discovered(&self) -> &Vec<usize> {
        &self.discovered
    }

    pub fn get_finished(&self) -> &Vec<usize> {
        &self.finished
    }

    pub fn get_id_map(&self) -> &provide::IdMap {
        &self.id_map
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // use crate::graph::structs::SimpleGraph;
//     use crate::graph::MatGraph;
//     use crate::provide::*;
//     use crate::storage::Mat;

//     #[test]
//     fn one_vertex_directed_graph() {
//         // Given: directed graph with single vertex.
//         let mut graph = MatGraph::<usize>::init(Mat::init(true));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(false));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(true));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(false));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(true));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(false));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(true));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(false));
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
//         let mut graph = MatGraph::<usize>::init(Mat::init(true));
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
