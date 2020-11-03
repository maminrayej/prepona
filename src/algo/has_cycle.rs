use crate::provide;
use crate::traversal::{Color, Dfs};

use magnitude::Magnitude;
use std::cell::RefCell;

pub struct HasCycle {
    parents: RefCell<Vec<Magnitude<usize>>>,
    stack: RefCell<Vec<usize>>,
}

impl HasCycle {
    pub fn init<G>(graph: &G) -> Self
    where
        G: provide::Neighbors + provide::Vertices,
    {
        HasCycle {
            parents: RefCell::new(vec![Magnitude::PosInfinite; graph.vertex_count()]),
            stack: RefCell::new(vec![]),
        }
    }

    pub fn detect<G>(&self, graph: &G) -> Vec<usize>
    where
        G: provide::Neighbors + provide::Vertices,
    {
        let mut cycle = Vec::new();

        let dfs = Dfs::init(graph);

        dfs.execute(
            graph,
            |virt_id| {
                self.stack.borrow_mut().push(virt_id);
            },
            |virt_id| {
                self.parents.borrow_mut()[virt_id] = (*self.stack.borrow().last().unwrap()).into();

                if self.parents.borrow()[virt_id] != virt_id.into() {
                    self.stack.borrow_mut().push(virt_id);
                }

                // detect cycle
                let real_id = dfs.get_id_map().get_virt_to_real(virt_id).unwrap();

                if let Some(start_virt_id) = graph
                    .neighbors(real_id)
                    .into_iter()
                    .map(|real_id| dfs.get_id_map().get_real_to_virt(real_id).unwrap())
                    .find(|&n_id| {
                        dfs.get_colors()[n_id] != Color::White
                            && self.parents.borrow()[virt_id] != n_id.into()
                    })
                {
                    // trace back to vertex with start_virt_id
                    let mut parent = virt_id;
                    while parent != start_virt_id {
                        cycle.push(parent);
                        parent = self.parents.borrow()[parent].unwrap();
                    }

                    cycle.push(parent);

                    cycle.reverse()
                }
            },
            |_| (),
            |_| {
                self.stack.borrow_mut().pop();
            },
            || ()
        );

        cycle
            .into_iter()
            .map(|virt_id| dfs.get_id_map().get_virt_to_real(virt_id).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;
    #[test]
    fn directed_simple_cycle() {
        // Give: A directed graph:
        //      a --> b --> c
        //      ^           |
        //      '-----------'
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        println!("{:?}", cycle);
    }

    #[test]
    fn undirected_simple_cycle() {
        // Give: An undirected graph:
        //      a -- b -- c
        //      |         |
        //      '---------'
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        println!("{:?}", cycle);
    }

    #[test]
    fn undirected_single_edge() {
        // Give: An undirected graph:
        //      a -- b
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, 1.into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn directed_single_edge() {
        // Give: An undirected graph:
        //      a --> b
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, 1.into());

        let has_cycle = HasCycle::init(&graph);

        let cycle = has_cycle.detect(&graph);

        assert_eq!(cycle.len(), 0);
    }
}
