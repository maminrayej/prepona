use std::collections::HashSet;

use crate::provide;

pub struct Dfs {
    stack: Vec<usize>,
    visited: HashSet<usize>,
}

impl Dfs {
    pub fn init(src_index: usize) -> Self {
        Dfs {
            stack: vec![src_index],
            visited: HashSet::new(),
        }
    }

    pub fn next<G, W>(&mut self, graph: &G) -> Option<usize>
    where
        G: provide::Graph<W> + provide::Neighbors,
    {
        if let Some(v_index) = self.stack.pop() {
            let mut undiscovered_neighbors = graph
                .neighbors(v_index)
                .iter()
                .filter(|&neighbor_index| {
                    !self.visited.contains(neighbor_index) && !self.stack.contains(neighbor_index)
                })
                .copied()
                .collect::<Vec<usize>>();

            self.stack.append(&mut undiscovered_neighbors);

            self.visited.insert(v_index);

            Some(v_index)
        } else {
            None
        }
    }

    pub fn get_stack(&self) -> &Vec<usize> {
        &self.stack
    }

    pub fn get_visited(&self) -> &HashSet<usize> {
        &self.visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::structs::SimpleGraph;
    use crate::graph::EdgeType;
    use crate::provide::*;
    use crate::storage::Storage;

    #[test]
    fn dense_graph() {
        let mut graph = SimpleGraph::<usize>::init(Storage::AdjMatrix, EdgeType::Directed);
        for _ in 0..5 {
            graph.add_vertex();
        }

        for i in 0..5 {
            for j in 0..5 {
                if i == j {
                    continue;
                }
                graph.add_edge(i, j, 1.into());
            }
        }

        for src_index in 0..5 {
            let mut dfs = Dfs::init(src_index);
            let mut count = 0usize;
            while let Some(_) = dfs.next(&graph) {
                count += 1;
            }

            assert_eq!(count, 5);
        }
    }
}
