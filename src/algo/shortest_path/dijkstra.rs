use magnitude::Magnitude;
use num_traits::{Unsigned, Zero};
use std::any::Any;
use std::collections::HashMap;

use crate::graph::Edge;
use crate::provide;

pub struct Dijkstra<W> {
    visited: Vec<bool>,
    dist: Vec<Magnitude<W>>,
    prev: Vec<Magnitude<usize>>,
}

impl<W: Clone + Ord + Zero + Any + Unsigned> Dijkstra<W> {
    pub fn init<G, E: Edge<W>>(graph: &G) -> Self
    where
        G: provide::Edges<W, E> + provide::Vertices,
    {
        let vertex_count = graph.vertex_count();

        Dijkstra {
            visited: vec![false; vertex_count],
            dist: vec![Magnitude::PosInfinite; vertex_count],
            prev: vec![Magnitude::PosInfinite; vertex_count],
        }
    }

    fn next_id(&self) -> Option<usize> {
        self.dist
            .iter()
            .enumerate()
            .filter(|(virt_id, dist)| dist.is_finite() && self.visited[*virt_id] == false)
            .min_by(|dist1, dist2| dist1.1.cmp(dist2.1))
            .map(|(v_id, _)| v_id)
    }

    pub fn execute<G, E: Edge<W>>(
        mut self,
        graph: &G,
        src_id: usize,
    ) -> HashMap<(usize, usize), Magnitude<W>>
    where
        G: provide::Edges<W, E> + provide::Vertices,
    {
        let id_map = graph.continuos_id_map();

        let src_virt_id = id_map.virt_id_of(src_id);

        self.dist[src_virt_id] = W::zero().into();

        while let Some(virt_id) = self.next_id() {
            self.visited[virt_id] = true;

            let real_id = id_map.real_id_of(virt_id);

            for (n_id, edge) in graph.edges_from(real_id) {
                let n_virt_id = id_map.virt_id_of(n_id);

                let alt = self.dist[virt_id].clone() + edge.get_weight().clone();
                if alt < self.dist[n_virt_id] {
                    self.dist[n_virt_id] = alt;
                    self.prev[n_virt_id] = virt_id.into();
                }
            }
        }

        let mut distance_map = HashMap::new();
        for virt_id in 0..graph.vertex_count() {
            let real_id = id_map.real_id_of(virt_id);
            distance_map.insert((src_id, real_id), self.dist[virt_id].clone());
        }

        distance_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::{DiMat, Mat};
    
    #[test]
    fn one_vertex_undirected_graph() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();

        let shortest_paths = Dijkstra::init(&graph).execute(&graph, a);

        assert_eq!(shortest_paths.keys().len(), 1);
        assert_eq!(*shortest_paths.get(&(a, a)).unwrap(), 0.into());
    }

    #[test]
    fn one_vertex_directed_graph() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();

        let shortest_paths = Dijkstra::init(&graph).execute(&graph, a);

        assert_eq!(shortest_paths.keys().len(), 1);
        assert_eq!(*shortest_paths.get(&(a, a)).unwrap(), 0.into());
    }

    #[test]
    fn trivial_undirected_graph() {
        // Given: Graph
        //          6       5
        //      a  ---  b  ---  c
        //    1 |       |       | 5
        //      |  2 /`````\ 2  |
        //      |````       ````|
        //      d  -----------  e
        //              1
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        graph.add_edge(a, b, 6.into());
        graph.add_edge(a, d, 1.into());
        graph.add_edge(b, d, 2.into());
        graph.add_edge(b, c, 5.into());
        graph.add_edge(b, e, 2.into());
        graph.add_edge(c, e, 5.into());
        graph.add_edge(d, e, 1.into());

        // When: Performing Dijkstra algorithm.
        let shortest_paths = Dijkstra::init(&graph).execute(&graph, a);

        // Then:
        assert_eq!(shortest_paths.keys().len(), 5);
        assert_eq!(*shortest_paths.get(&(a, a)).unwrap(), 0.into());
        assert_eq!(*shortest_paths.get(&(a, b)).unwrap(), 3.into());
        assert_eq!(*shortest_paths.get(&(a, c)).unwrap(), 7.into());
        assert_eq!(*shortest_paths.get(&(a, d)).unwrap(), 1.into());
        assert_eq!(*shortest_paths.get(&(a, e)).unwrap(), 2.into());
    }

    #[test]
    fn trivial_directed_graph() {
        // Given: Graph
        //          6       1
        //      a  -->  b  <--  c ---
        //    1 |       |           |
        //      |  2 /`````\ 2      |
        //      |````       ````|   |
        //      v               v   | 1
        //      d  ---------->  e --'
        //              1
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex(); // 0
        let b = graph.add_vertex(); // 1
        let c = graph.add_vertex(); // 2
        let d = graph.add_vertex(); // 3
        let e = graph.add_vertex(); // 4

        graph.add_edge(a, b, 6.into());
        graph.add_edge(a, d, 1.into());
        graph.add_edge(b, d, 2.into());
        graph.add_edge(b, e, 2.into());
        graph.add_edge(c, b, 1.into());
        graph.add_edge(e, c, 1.into());
        graph.add_edge(d, e, 1.into());

        // When: Performing Dijkstra algorithm.
        let shortest_paths = Dijkstra::init(&graph).execute(&graph, a);

        // Then:
        assert_eq!(shortest_paths.keys().len(), 5);
        assert_eq!(*shortest_paths.get(&(a, a)).unwrap(), 0.into());
        assert_eq!(*shortest_paths.get(&(a, b)).unwrap(), 4.into());
        assert_eq!(*shortest_paths.get(&(a, c)).unwrap(), 3.into());
        assert_eq!(*shortest_paths.get(&(a, d)).unwrap(), 1.into());
        assert_eq!(*shortest_paths.get(&(a, e)).unwrap(), 2.into());
    }
}
