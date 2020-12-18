use magnitude::Magnitude;
use num_traits::{Unsigned, Zero};
use std::any::Any;
use std::collections::HashMap;

use crate::provide::{Edges, Graph, Vertices};
use crate::{
    graph::{subgraph::ShortestPathSubgraph, Edge, EdgeDir},
    prelude::Neighbors,
};

pub struct Dijkstra<W> {
    visited: Vec<bool>,
    dist: Vec<Magnitude<W>>,
    prev: Vec<Magnitude<usize>>,
}

impl<W: Copy + Ord + Zero + Any + Unsigned> Dijkstra<W> {
    pub fn init<E, Ty, G>(graph: &G) -> Self
    where
        E: Edge<W>,
        Ty: EdgeDir,
        G: Edges<W, E> + Vertices + Graph<W, E, Ty>,
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
            .min_by(|(_, dist1), (_, dist2)| dist1.cmp(dist2))
            .and_then(|(v_id, _)| Some(v_id))
    }

    pub fn execute<E, Ty, G>(
        mut self,
        graph: &G,
        src_id: usize,
    ) -> ShortestPathSubgraph<W, E, Ty, G>
    where
        E: Edge<W>,
        Ty: EdgeDir,
        G: Edges<W, E> + Neighbors + Vertices + Graph<W, E, Ty>,
    {
        let mut edges = vec![];

        let id_map = graph.continuos_id_map();

        let src_virt_id = id_map.virt_id_of(src_id);

        self.dist[src_virt_id] = W::zero().into();

        while let Some(virt_id) = self.next_id() {
            self.visited[virt_id] = true;

            let real_id = id_map.real_id_of(virt_id);

            for (n_id, edge) in graph.edges_from(real_id) {
                let n_virt_id = id_map.virt_id_of(n_id);

                let alt = self.dist[virt_id] + *edge.get_weight();
                if alt < self.dist[n_virt_id] {
                    self.dist[n_virt_id] = alt;
                    self.prev[n_virt_id] = virt_id.into();

                    edges.retain(|(_, dst_id, _)| *dst_id != n_id); // remove edge to neighbor
                    edges.push((real_id, n_id, edge.get_id())); // add new edge
                }
            }
        }

        let mut distance_map = HashMap::new();
        for virt_id in 0..graph.vertex_count() {
            let real_id = id_map.real_id_of(virt_id);
            distance_map.insert(real_id, self.dist[virt_id]);
        }

        let mut vertices = edges
            .iter()
            .flat_map(|(src_id, dst_id, _)| vec![*src_id, *dst_id])
            .chain(std::iter::once(src_id))
            .collect::<Vec<usize>>();

        // Remove duplicated vertices.
        vertices.sort();
        vertices.dedup();

        ShortestPathSubgraph::init(graph, edges, vertices, distance_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::storage::{DiMat, Mat};

    #[test]
    fn one_vertex_undirected_graph() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();

        let sp_subgraph = Dijkstra::init(&graph).execute(&graph, a);

        // assert_eq!(sp_subgraph.keys().len(), 1);
        // assert_eq!(*sp_subgraph.get(&(a, a)).unwrap(), 0.into());
        assert_eq!(sp_subgraph.distance_to(a).unwrap(), 0.into());
        assert_eq!(sp_subgraph.vertex_count(), 1);
        assert_eq!(sp_subgraph.vertices()[0], a);
        assert_eq!(sp_subgraph.edges_count(), 0);
    }

    #[test]
    fn one_vertex_directed_graph() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();

        let sp_subgraph = Dijkstra::init(&graph).execute(&graph, a);

        assert_eq!(sp_subgraph.distance_to(a).unwrap(), 0.into());
        assert_eq!(sp_subgraph.vertex_count(), 1); // how to add vertex? without edge? problem with Subgraph definition.
        assert_eq!(sp_subgraph.vertices()[0], a);
        assert_eq!(sp_subgraph.edges_count(), 0);
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
        let ad = graph.add_edge(a, d, 1.into());
        let bd = graph.add_edge(b, d, 2.into());
        graph.add_edge(b, c, 5.into());
        graph.add_edge(b, e, 2.into());
        let ce = graph.add_edge(c, e, 5.into());
        let de = graph.add_edge(d, e, 1.into());

        // When: Performing Dijkstra algorithm.
        let sp_subgraph = Dijkstra::init(&graph).execute(&graph, a);

        // Then:
        assert_eq!(sp_subgraph.vertex_count(), 5);
        assert_eq!(sp_subgraph.edges_count(), 4);
        assert!(vec![a, b, c, d, e]
            .iter()
            .all(|vertex_id| sp_subgraph.vertices().contains(vertex_id)));
        assert!(vec![ad, bd, ce, de]
            .into_iter()
            .all(|edge_id| sp_subgraph.edge(edge_id).is_some()));
        assert_eq!(sp_subgraph.distance_to(a).unwrap(), 0.into());
        assert_eq!(sp_subgraph.distance_to(b).unwrap(), 3.into());
        assert_eq!(sp_subgraph.distance_to(c).unwrap(), 7.into());
        assert_eq!(sp_subgraph.distance_to(d).unwrap(), 1.into());
        assert_eq!(sp_subgraph.distance_to(e).unwrap(), 2.into());
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
        let ad = graph.add_edge(a, d, 1.into());
        graph.add_edge(b, d, 2.into());
        graph.add_edge(b, e, 2.into());
        let cb = graph.add_edge(c, b, 1.into());
        let ec = graph.add_edge(e, c, 1.into());
        let de = graph.add_edge(d, e, 1.into());

        // When: Performing Dijkstra algorithm.
        let sp_subgraph = Dijkstra::init(&graph).execute(&graph, a);

        // Then:
        assert_eq!(sp_subgraph.vertex_count(), 5);
        assert_eq!(sp_subgraph.edges_count(), 4);
        assert!(vec![a, b, c, d, e]
            .iter()
            .all(|vertex_id| sp_subgraph.vertices().contains(vertex_id)));
        assert!(vec![ad, cb, ec, de]
            .into_iter()
            .all(|edge_id| sp_subgraph.edge(edge_id).is_some()));
        assert_eq!(sp_subgraph.distance_to(a).unwrap(), 0.into());
        assert_eq!(sp_subgraph.distance_to(b).unwrap(), 4.into());
        assert_eq!(sp_subgraph.distance_to(c).unwrap(), 3.into());
        assert_eq!(sp_subgraph.distance_to(d).unwrap(), 1.into());
        assert_eq!(sp_subgraph.distance_to(e).unwrap(), 2.into());
    }
}
