use magnitude::Magnitude;
use num_traits::Zero;
use std::any::Any;
use std::collections::HashMap;

use crate::graph::Edge;
use crate::provide;

pub struct Dijkstra<W> {
    visited: Vec<bool>,
    dist: Vec<Magnitude<W>>,
    prev: Vec<Magnitude<usize>>,
}

impl<W: Clone + Ord + Zero + Any> Dijkstra<W> {
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
        let min_dist = self
            .dist
            .iter()
            .enumerate()
            .filter(|(virt_id, dist)| dist.is_finite() && self.visited[*virt_id] == false)
            .min();

        if let Some((v_id, _)) = min_dist {
            Some(v_id)
        } else {
            None
        }
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

        let src_virt_id = id_map.get_real_to_virt(src_id);

        if src_virt_id.is_none() {
            panic!(format!("{} is not valid.", src_id))
        }

        self.dist[src_virt_id.unwrap()] = W::zero().into();

        while let Some(virt_id) = self.next_id() {

            self.visited[virt_id] = true;

            let real_id = id_map.get_virt_to_real(virt_id).unwrap();

            for (n_id, edge) in graph.edges_from(real_id) {
                let n_virt_id = id_map.get_real_to_virt(n_id).unwrap();

                let alt = self.dist[virt_id].clone() + edge.get_weight().clone();
                if alt < self.dist[n_virt_id] {
                    self.dist[n_virt_id] = alt;
                    self.prev[n_virt_id] = virt_id.into();
                }
            }
        }

        let mut distance_map = HashMap::new();
        for virt_id in 0..graph.vertex_count() {
            let real_id = id_map.get_virt_to_real(virt_id).unwrap();
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
    use crate::storage::Mat;

    #[test]
    fn dijkstra_test() {
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
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

        let dijkstra = Dijkstra::init(&graph).execute(&graph, a);

        let mut tags = std::collections::HashMap::<usize, &'static str>::new();
        tags.insert(a, "a");
        tags.insert(b, "b");
        tags.insert(c, "c");
        tags.insert(d, "d");
        tags.insert(e, "e");

        println!(
            "{:?}",
            dijkstra.into_iter()
            .map(|((v1, v2), dist)| (tags.get(&v1).unwrap(), tags.get(&v2).unwrap(), dist))
            .collect::<Vec<(&&str, &&str, Magnitude<usize>)>>()
        );
    }
}
