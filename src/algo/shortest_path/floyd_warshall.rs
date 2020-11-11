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

    pub fn execute<G, W: Copy + Zero + Any + Ord, E: Edge<W>>(
        self,
        graph: &G,
    ) -> HashMap<(usize, usize), Magnitude<W>>
    where
        G: provide::Edges<W, E> + provide::Vertices,
    {
        let vertices = graph.vertices();
        let vertex_count = vertices.len();

        let id_map = graph.continuos_id_map();

        let mut dist = vec![vec![Magnitude::PosInfinite; vertex_count]; vertex_count];

        for &u_real_id in &vertices {
            let u_virt_id = id_map.get_real_to_virt(u_real_id).unwrap();

            for edge in graph.edges_from(u_real_id) {
                let v_real_id = edge.get_dst_id();
                let v_virt_id = id_map.get_real_to_virt(v_real_id).unwrap();
                if u_virt_id == v_virt_id {
                    dist[u_virt_id][v_virt_id] = W::zero().into();
                } else {
                    dist[u_virt_id][v_virt_id] = edge.get_weight().clone();
                }
            }
        }

        for k in 0..vertex_count {
            for &i in &vertices {
                let i_virt_id = id_map.get_real_to_virt(i).unwrap();
                for &j in &vertices {
                    let j_virt_id = id_map.get_real_to_virt(j).unwrap();

                    if dist[i_virt_id][j_virt_id] > dist[i_virt_id][k] + dist[k][j_virt_id] {
                        dist[i_virt_id][j_virt_id] = dist[i_virt_id][k] + dist[k][j_virt_id]
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

        distance_map
    }
}
