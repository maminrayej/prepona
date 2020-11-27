use magnitude::Magnitude;
use num_traits::Zero;
use std::any::Any;
use std::collections::HashMap;

use crate::graph::{Edge, subgraph::ShortestPathSubgraph};
use crate::provide;

pub struct BellmanFord<W> {
    distance: Vec<Magnitude<W>>,
    prev: Vec<Magnitude<usize>>,
}

impl<W: Copy + Any + Zero + Ord + std::fmt::Debug> BellmanFord<W> {
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
    ) -> Result<ShortestPathSubgraph<W, E>, String>
    where
        G: provide::Vertices + provide::Edges<W, E>,
    {
        let mut sp_edges = vec![];

        let vertex_count = graph.vertex_count();

        let id_map = graph.continuos_id_map();

        let src_virt_id = id_map.virt_id_of(src_id);

        self.distance[src_virt_id] = W::zero().into();

        let edges = graph.edges();

        for _ in 0..vertex_count - 1 {
            for (u_real_id, v_real_id, edge) in &edges {
                let u_virt_id = id_map.virt_id_of(*u_real_id);
                let v_virt_id = id_map.virt_id_of(*v_real_id);

                let alt = self.distance[u_virt_id] + *edge.get_weight();
                if alt < self.distance[v_virt_id] {
                    self.distance[v_virt_id] = alt;
                    self.prev[v_virt_id] = u_virt_id.into();

                    sp_edges.retain(|(_, dst_id, _)| dst_id != v_real_id); // remove edge to neighbor
                    sp_edges.push((*u_real_id, *v_real_id, *edge)); // add new edge
                }
            }
        }

        for (u_real_id, v_real_id, edge) in &edges {
            let u_virt_id = id_map.real_id_of(*u_real_id);
            let v_virt_id = id_map.real_id_of(*v_real_id);

            let alt = self.distance[u_virt_id].clone() + edge.get_weight().clone();
            if alt < self.distance[v_virt_id] {
                return Err("Cycle detected".to_string());
            }
        }

        let mut distance_map = HashMap::new();
        for virt_id in 0..graph.vertex_count() {
            let real_id = id_map.real_id_of(virt_id);
            distance_map.insert(real_id, self.distance[virt_id].clone());
        }


        let mut vertices = edges
            .iter()
            .flat_map(|(src_id, dst_id, _)| vec![*src_id, *dst_id])
            .chain(std::iter::once(src_id))
            .collect::<Vec<usize>>();

        // Remove duplicated vertices.
        vertices.sort();
        vertices.dedup();

        Ok(ShortestPathSubgraph::init(sp_edges, vertices, distance_map))
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

        let sp_subgraph = BellmanFord::init(&graph).execute(&graph, a);

        assert!(sp_subgraph.is_ok());
        let sp_subgraph = sp_subgraph.unwrap();
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

        let sp_subgraph = BellmanFord::init(&graph).execute(&graph, a);

        // Then 
        assert!(sp_subgraph.is_ok());
        let sp_subgraph = sp_subgraph.unwrap();
        assert_eq!(sp_subgraph.distance_to(a).unwrap(), 0.into());
        assert_eq!(sp_subgraph.vertex_count(), 1);
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

        // When: Performing BellmanFord algorithm.
        let sp_subgraph = BellmanFord::init(&graph).execute(&graph, a);

        // Then:
        assert!(sp_subgraph.is_ok());
        let sp_subgraph = sp_subgraph.unwrap();
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

        // When: Performing BellmanFord algorithm.
        let sp_subgraph = BellmanFord::init(&graph).execute(&graph, a);

        // Then:
        assert!(sp_subgraph.is_ok());
        let sp_subgraph = sp_subgraph.unwrap();
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
    fn undirected_graph_with_negative_walk() {
        // Given: Graph
        //          -1
        //      a ----- b
        //
        let mut graph = MatGraph::init(Mat::<isize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        graph.add_edge(a, b, (-1).into());

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
