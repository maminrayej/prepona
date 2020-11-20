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
                // let (u_real_id, v_real_id) = (edge.get_src_id(), edge.get_dst_id());

                let u_virt_id = id_map.get_real_to_virt(*u_real_id).unwrap();
                let v_virt_id = id_map.get_real_to_virt(*v_real_id).unwrap();

                let alt = self.distance[u_virt_id].clone() + edge.get_weight().clone();
                if alt < self.distance[v_virt_id] {
                    self.distance[v_virt_id] = alt;
                    self.prev[v_virt_id] = u_virt_id.into();
                }

                println!("***\n")
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
    use crate::storage::Mat;

    #[test]
    fn bellman_ford_test() {
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex(); // 0
        let b = graph.add_vertex(); // 1
        let c = graph.add_vertex(); // 2
        let d = graph.add_vertex(); // 3
        let e = graph.add_vertex(); // 4

        graph.add_edge(a, b, 6.into());
        graph.add_edge(a, d, 1.into());

        graph.add_edge(b, d, 2.into());
        graph.add_edge(b, c, 5.into());
        graph.add_edge(b, e, 2.into());

        graph.add_edge(c, e, 5.into());

        graph.add_edge(d, e, 1.into());

        let dijkstra = BellmanFord::init(&graph).execute(&graph, a).unwrap();

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");

        println!(
            "{:?}",
            dijkstra
                .into_iter()
                .map(|((v1, v2), dist)| (tags.get(&v1).unwrap(), tags.get(&v2).unwrap(), dist))
                .collect::<Vec<(&&str, &&str, Magnitude<usize>)>>()
        );
    }
}
