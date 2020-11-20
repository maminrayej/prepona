use magnitude::Magnitude;
use num_traits::Zero;
use std::any::Any;
use std::collections::HashMap;

use crate::graph::Edge;
use crate::provide;

pub struct FloydWarshall {}

impl FloydWarshall {
    pub fn init() -> Self {
        FloydWarshall {}
    }

    pub fn execute<G, W: Copy + Zero + Any + Ord + std::fmt::Debug, E: Edge<W>>(
        self,
        graph: &G,
    ) -> Result<HashMap<(usize, usize), Magnitude<W>>, String>
    where
        G: provide::Edges<W, E> + provide::Vertices,
    {
        let vertices = graph.vertices();
        let vertex_count = vertices.len();

        let id_map = graph.continuos_id_map();

        let mut dist = vec![vec![Magnitude::PosInfinite; vertex_count]; vertex_count];

        for &u_real_id in &vertices {
            let u_virt_id = id_map.get_real_to_virt(u_real_id).unwrap();
            dist[u_virt_id][u_virt_id] = W::zero().into();
        }

        for &u_real_id in &vertices {
            let u_virt_id = id_map.get_real_to_virt(u_real_id).unwrap();

            for (v_real_id, edge) in graph.edges_from(u_real_id) {
                let v_virt_id = id_map.get_real_to_virt(v_real_id).unwrap();
                dist[u_virt_id][v_virt_id] = edge.get_weight().clone();
            }
        }

        for k in 0..vertex_count {
            for &i in &vertices {
                let i_virt_id = id_map.get_real_to_virt(i).unwrap();
                for &j in &vertices {
                    let j_virt_id = id_map.get_real_to_virt(j).unwrap();

                    if (dist[i_virt_id][k] + dist[k][j_virt_id]).is_finite()
                        && dist[i_virt_id][j_virt_id] > dist[i_virt_id][k] + dist[k][j_virt_id]
                    {
                        dist[i_virt_id][j_virt_id] = dist[i_virt_id][k] + dist[k][j_virt_id]
                    }
                }

                // check for negative cycle
                for v_id in &vertices {
                    let v_virt_id = id_map.get_real_to_virt(*v_id).unwrap();
                    if dist[v_virt_id][v_virt_id] < W::zero().into() {
                        return Err("Graph contains negative cycle".to_string());
                    }
                }
            }
        }

        let mut distance_map = HashMap::new();
        for i in 0..vertex_count {
            let i_real_id = id_map.get_virt_to_real(i).unwrap();
            for j in 0..vertex_count {
                let j_real_id = id_map.get_virt_to_real(j).unwrap();

                distance_map.insert((i_real_id, j_real_id), dist[i][j].clone());
            }
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
    fn empty_directed_graph() {
        // Given:
        let graph = MatGraph::init(DiMat::<usize>::init());

        let distance_map = FloydWarshall::init().execute(&graph);

        assert_eq!(distance_map.unwrap().keys().len(), 0);
    }

    #[test]
    fn empty_undirected_graph() {
        // Given:
        let graph = MatGraph::init(Mat::<usize>::init());

        let distance_map = FloydWarshall::init().execute(&graph);

        assert_eq!(distance_map.unwrap().keys().len(), 0);
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

        // When: Performing FloydWarshall algorithm.
        let distance_map = FloydWarshall::init().execute(&graph);

        // Then:
        assert!(distance_map.is_ok());
        let distance_map = distance_map.unwrap();

        let expected: HashMap<(usize, usize), Magnitude<usize>> = [
            ((a, a), 0.into()),
            ((a, b), 3.into()),
            ((a, c), 7.into()),
            ((a, d), 1.into()),
            ((a, e), 2.into()),
            ((b, a), 3.into()),
            ((b, b), 0.into()),
            ((b, c), 5.into()),
            ((b, d), 2.into()),
            ((b, e), 2.into()),
            ((c, a), 7.into()),
            ((c, b), 5.into()),
            ((c, c), 0.into()),
            ((c, d), 6.into()),
            ((c, e), 5.into()),
            ((d, a), 1.into()),
            ((d, b), 2.into()),
            ((d, c), 6.into()),
            ((d, d), 0.into()),
            ((d, e), 1.into()),
            ((e, a), 2.into()),
            ((e, b), 2.into()),
            ((e, c), 5.into()),
            ((e, d), 1.into()),
            ((e, e), 0.into()),
        ]
        .iter()
        .copied()
        .collect();

        let vertices = [a, b, c, d, e];
        for v1_id in &vertices {
            for v2_id in &vertices {
                assert_eq!(
                    distance_map.get(&(*v1_id, *v2_id)),
                    expected.get(&(*v1_id, *v2_id))
                )
            }
        }
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

        // When: Performing FloydWarshall algorithm.
        let distance_map = FloydWarshall::init().execute(&graph);

        // Then:
        assert!(distance_map.is_ok());
        let distance_map = distance_map.unwrap();

        let expected: HashMap<(usize, usize), Magnitude<usize>> = [
            ((a, a), 0.into()),
            ((a, b), 4.into()),
            ((a, c), 3.into()),
            ((a, d), 1.into()),
            ((a, e), 2.into()),
            ((b, b), 0.into()),
            ((b, c), 3.into()),
            ((b, d), 2.into()),
            ((b, e), 2.into()),
            ((c, b), 1.into()),
            ((c, c), 0.into()),
            ((c, d), 3.into()),
            ((c, e), 3.into()),
            ((d, b), 3.into()),
            ((d, c), 2.into()),
            ((d, d), 0.into()),
            ((d, e), 1.into()),
            ((e, b), 2.into()),
            ((e, c), 1.into()),
            ((e, d), 4.into()),
            ((e, e), 0.into()),
        ]
        .iter()
        .copied()
        .collect();

        let vertices = [a, b, c, d, e];
        for v1_id in &vertices {
            for v2_id in &vertices {
                if let Some(dist) = expected.get(&(*v1_id, *v2_id)) {
                    assert_eq!(distance_map.get(&(*v1_id, *v2_id)).unwrap(), dist)
                } else {
                    assert!(distance_map
                        .get(&(*v1_id, *v2_id))
                        .unwrap()
                        .is_pos_infinite())
                }
            }
        }
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

        // When: Performing FloydWarshall algorithm.
        let distance_map = FloydWarshall::init().execute(&graph);

        assert!(distance_map.is_err());
    }

    #[test]
    fn undirected_graph_with_negative_walk() {
        // Given: Graph
        //          -1
        //      a ----- b
        //
        let mut graph = MatGraph::init(Mat::<isize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, (-1).into());

        let shortest_paths = FloydWarshall::init().execute(&graph);

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

        // When: Performing FloydWarshall algorithm.
        let shortest_paths = FloydWarshall::init().execute(&graph);

        assert!(shortest_paths.is_err());
    }
}
