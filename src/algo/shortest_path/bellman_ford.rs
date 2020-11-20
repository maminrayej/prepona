use magnitude::Magnitude;
use num_traits::Zero;
use std::any::Any;
use std::collections::HashMap;

use crate::graph::Edge;
use crate::provide;

pub struct BellmanFord<W> {
    distance: Vec<Magnitude<W>>,
    prev: Vec<Magnitude<usize>>,
}

impl<W: Clone + Any + Zero + Ord + std::fmt::Debug> BellmanFord<W> {
    pub fn init<G>(graph: &G) -> Self
    where
        G: provide::Vertices,
    {
        let vertex_count = graph.vertex_count();

        BellmanFord {
            distance: vec![Magnitude::PosInfinite; vertex_count],
            prev: vec![Magnitude::PosInfinite; vertex_count],
        }
    }

    pub fn execute<G, E: Edge<W>>(
        mut self,
        graph: &G,
        src_id: usize,
    ) -> Result<HashMap<(usize, usize), Magnitude<W>>, String>
    where
        G: provide::Vertices + provide::Edges<W, E>,
    {
        let vertex_count = graph.vertex_count();

        let id_map = graph.continuos_id_map();

        let src_virt_id = id_map
            .get_real_to_virt(src_id)
            .expect(&format!("{} is not valid", src_id));

        self.distance[src_virt_id] = W::zero().into();

        let edges = graph.edges();

        for _ in 0..vertex_count - 1 {
            for (u_real_id, v_real_id, edge) in &edges {
                let u_virt_id = id_map.get_real_to_virt(*u_real_id).unwrap();
                let v_virt_id = id_map.get_real_to_virt(*v_real_id).unwrap();

                let alt = self.distance[u_virt_id].clone() + edge.get_weight().clone();
                if alt < self.distance[v_virt_id] {
                    self.distance[v_virt_id] = alt;
                    self.prev[v_virt_id] = u_virt_id.into();
                }
            }
        }

        for (u_real_id, v_real_id, edge) in &edges {
            // let (u_real_id, v_real_id) = (edge.get_src_id(), edge.get_dst_id());

            let u_virt_id = id_map.get_real_to_virt(*u_real_id).unwrap();
            let v_virt_id = id_map.get_real_to_virt(*v_real_id).unwrap();

            let alt = self.distance[u_virt_id].clone() + edge.get_weight().clone();
            if alt < self.distance[v_virt_id] {
                return Err("Cycle detected".to_string());
            }
        }

        let mut distance_map = HashMap::new();
        for virt_id in 0..graph.vertex_count() {
            let real_id = id_map.get_virt_to_real(virt_id).unwrap();
            distance_map.insert((src_id, real_id), self.distance[virt_id].clone());
        }

        Ok(distance_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::{DiMat, Mat};

    #[test]
    #[should_panic(expected = "0 is not valid")]
    fn empty_undirected_graph() {
        let graph = MatGraph::init(Mat::<usize>::init());

        let _ = BellmanFord::init(&graph).execute(&graph, 0);
    }

    #[test]
    #[should_panic(expected = "0 is not valid")]
    fn empty_directed_graph() {
        let graph = MatGraph::init(Mat::<usize>::init());

        let _ = BellmanFord::init(&graph).execute(&graph, 0);
    }

    #[test]
    fn one_vertex_undirected_graph() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();

        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        assert!(shortest_paths.is_ok());
        let shortest_paths = shortest_paths.unwrap();
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

        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        assert!(shortest_paths.is_ok());
        let shortest_paths = shortest_paths.unwrap();
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

        // When: Performing BellmanFord algorithm.
        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        // Then:
        assert!(shortest_paths.is_ok());
        let shortest_paths = shortest_paths.unwrap();
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

        // When: Performing BellmanFord algorithm.
        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        // Then:
        assert!(shortest_paths.is_ok());
        let shortest_paths = shortest_paths.unwrap();
        assert_eq!(shortest_paths.keys().len(), 5);
        assert_eq!(*shortest_paths.get(&(a, a)).unwrap(), 0.into());
        assert_eq!(*shortest_paths.get(&(a, b)).unwrap(), 4.into());
        assert_eq!(*shortest_paths.get(&(a, c)).unwrap(), 3.into());
        assert_eq!(*shortest_paths.get(&(a, d)).unwrap(), 1.into());
        assert_eq!(*shortest_paths.get(&(a, e)).unwrap(), 2.into());
    }

    #[test]
    fn undirected_graph_with_negative_cycle() {
        // Given: Graph
        //          1
        //      a ----- b
        //      |       | 2
        //      '------ c
        //          -5
        //
        let mut graph = MatGraph::init(Mat::<isize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 2.into());
        graph.add_edge(c, a, (-5).into());


        // When: Performing BellmanFord algorithm.
        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        assert!(shortest_paths.is_err());
    }

    #[test]
    fn directed_graph_with_negative_cycle() {
        // Given: Graph
        //          1
        //      a ----> b
        //      ^       | 2
        //      |       v
        //      '------ c
        //          -5
        //
        let mut graph = MatGraph::init(DiMat::<isize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 2.into());
        graph.add_edge(c, a, (-5).into());


        // When: Performing BellmanFord algorithm.
        let shortest_paths = BellmanFord::init(&graph).execute(&graph, a);

        assert!(shortest_paths.is_err());
    }
}