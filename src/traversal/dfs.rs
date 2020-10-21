use std::collections::HashSet;

use crate::provide;

pub struct Dfs {
    stack: Vec<usize>,
    discovered: HashSet<usize>,
}

impl Dfs {
    pub fn init(src_index: usize) -> Self {
        Dfs {
            stack: vec![src_index],
            discovered: HashSet::new(),
        }
    }

    pub fn next<G>(&mut self, graph: G) -> Option<usize>
    where
        G: provide::Graph + provide::Neighbors,
    {
        if let Some(v_index) = self.stack.pop() {
            let mut undiscovered_neighbors = graph
                .neighbors(v_index)
                .iter()
                .filter(|&&neighbor_index| !self.discovered.contains(&neighbor_index))
                .copied()
                .collect::<Vec<usize>>();

            self.stack.append(&mut undiscovered_neighbors);

            self.discovered.insert(v_index);

            Some(v_index)
        } else {
            None
        }
    }
}
