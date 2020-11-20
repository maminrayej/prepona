use crate::algo::{Dfs, DfsListener};
use crate::provide;

use magnitude::Magnitude;

pub struct HasCycle<'a, G> {
    parent_of: Vec<Magnitude<usize>>,
    stack: Vec<usize>,
    cycle: Vec<usize>,
    graph: &'a G,
}

impl<'a, G: provide::Neighbors + provide::Vertices + provide::Direction> DfsListener
    for HasCycle<'a, G>
{
    fn on_white(&mut self, dfs: &Dfs<Self>, virt_id: usize) {
        println!("Calling on white with vertex: {}", virt_id);

        if let Some(parent_id) = self.stack.last() {
            self.parent_of[virt_id] = (*parent_id).into();
        }

        self.stack.push(virt_id);

        // detect cycle
        let real_id = dfs.get_id_map().get_virt_to_real(virt_id).unwrap();

        if let Some(start_virt_id) = self
            .graph
            .neighbors(real_id)
            .into_iter()
            .map(|real_id| dfs.get_id_map().get_real_to_virt(real_id).unwrap())
            .find(|&n_id| {
                self.has_back_edge_to_neighbor(
                    n_id,
                    dfs.get_discovered()[n_id],
                    dfs.get_discovered()[virt_id],
                    dfs.get_finished()[n_id],
                    self.parent_of[virt_id],
                )
            })
        {
            // trace back to vertex with start_virt_id
            let mut parent = virt_id;
            while parent != start_virt_id {
                self.cycle.push(parent);
                if self.parent_of[parent].is_pos_infinite() {
                    break;
                }
                parent = self.parent_of[parent].unwrap();
            }

            self.cycle.push(parent);

            self.cycle.reverse()
        }
    }

    fn on_black(&mut self, _: &Dfs<Self>, _: usize) {
        self.stack.pop();
    }
}

impl<'a, G: provide::Neighbors + provide::Vertices + provide::Direction> HasCycle<'a, G> {
    pub fn init(graph: &'a G) -> Self {
        HasCycle {
            parent_of: vec![Magnitude::PosInfinite; graph.vertex_count()],
            stack: vec![],
            cycle: vec![],
            graph,
        }
    }

    fn has_back_edge_to_neighbor(
        &self,
        n_id: usize,
        n_disc: Magnitude<usize>,
        v_disc: Magnitude<usize>,
        n_finish: Magnitude<usize>,
        v_parent: Magnitude<usize>,
    ) -> bool {
        if G::is_directed() {
            n_disc < v_disc && n_finish.is_pos_infinite()
        } else {
            n_disc < v_disc && n_finish.is_pos_infinite() && v_parent != n_id.into()
        }
    }

    pub fn execute(mut self, graph: &G) -> Vec<usize> {
        println!("Executing ...");
        let dfs = Dfs::init(graph, &mut self);
        println!("Before executing dfs...");
        dfs.execute(graph);
        println!("After executing dfs...");

        let id_map = dfs.id_map();

        let a = self
            .cycle
            .iter()
            .map(|virt_id| id_map.get_virt_to_real(*virt_id).unwrap())
            .collect();

        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::{DiMat, Mat};

    #[test]
    fn empty_directed_graph() {
        // Given: An empty directed graph.
        let graph = MatGraph::init(DiMat::<usize>::init());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn empty_undirected_graph() {
        // Given: An empty undirected graph.
        let graph = MatGraph::init(Mat::<usize>::init());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn two_vertex_undirected_graph() {
        // Given: Graph
        //
        //      a  ---  b
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge((a, b, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn two_vertex_directed_graph() {
        // Given: Graph
        //
        //      a  -->  b
        //      ^       |
        //      |_______|
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, a, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 2);
        assert!(vec![a, b].iter().all(|v_id| cycle.contains(v_id)));
    }

    #[test]
    fn trivial_directed_graph_without_cycle() {
        // Given: Graph
        //
        //      a  -->  b  -->  c
        //      |       |
        //      v       v
        //      d  -->  e
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((a, d, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((b, e, 1).into());
        graph.add_edge((d, e, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn trivial_undirected_graph_without_cycle() {
        // Given: Graph
        //
        //      a  ---  b  ---  c
        //              |
        //              |
        //      d  ---  e
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((b, e, 1).into());
        graph.add_edge((d, e, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert_eq!(cycle.len(), 0);
    }

    #[test]
    fn trivial_directed_graph_with_cycle() {
        // Given: Graph
        //
        //      a  -->  b  -->  c  -->  d
        //      ^        \              ^
        //      |         \             |
        //      |          -->  e  -----'
        //      |               |
        //      '_______________'
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((c, d, 1).into());
        graph.add_edge((b, e, 1).into());
        graph.add_edge((e, d, 1).into());
        graph.add_edge((e, a, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        assert_eq!(cycle.len(), 3);
        assert!(vec![a, b, e].iter().all(|v_id| cycle.contains(v_id)));
    }

    #[test]
    fn trivial_undirected_graph() {
        // Given: Graph
        //
        //      a  ---  b  ---  c  ---  d
        //      |        \
        //      |         \
        //      |          ---  e
        //      |               |
        //      '_______________'
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        graph.add_edge((a, b, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((c, d, 1).into());
        graph.add_edge((b, e, 1).into());
        graph.add_edge((e, a, 1).into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        assert_eq!(cycle.len(), 3);
        assert!(vec![a, b, e].iter().all(|v_id| cycle.contains(v_id)));
    }
}

// Wishing you'd be here ferris pier?
