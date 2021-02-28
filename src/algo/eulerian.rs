use std::{collections::HashSet, marker::PhantomData};

use crate::{
    graph::{Edge, EdgeDir},
    provide::{Edges, Graph, IdMap, Vertices},
};

/// Calculates eulerian trail and circuit
pub struct Eulerian<W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> {
    unused_edges: HashSet<usize>,
    out_deg: Vec<u32>,
    in_deg: Vec<u32>,
    diff_deg: Vec<i32>,
    id_map: IdMap,
    trail: Vec<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_ty: PhantomData<Ty>,
    phantom_g: PhantomData<G>,
}

impl<W, E, Ty, G> Eulerian<W, E, Ty, G>
where
    E: Edge<W>,
    Ty: EdgeDir,
    G: Graph<W, E, Ty> + Vertices + Edges<W, E>,
{
    /// Initializes the structure
    pub fn init(graph: &G) -> Self {
        let id_map = graph.continuos_id_map();

        let vertex_count = graph.vertex_count();

        let mut out_deg = vec![0; vertex_count];
        let mut in_deg = vec![0; vertex_count];

        let mut unused_edges = HashSet::new();
        for (src_id, dst_id, edge) in graph.edges() {
            let src_virt_id = id_map.virt_id_of(src_id);
            let dst_virt_id = id_map.virt_id_of(dst_id);

            unused_edges.insert(edge.get_id());

            out_deg[src_virt_id] += 1;
            in_deg[dst_virt_id] += 1;

            if Ty::is_undirected() {
                in_deg[src_virt_id] += 1;
                out_deg[dst_virt_id] += 1;
            }
        }

        let mut diff_deg = vec![0; vertex_count];
        for v_id in 0..vertex_count {
            diff_deg[v_id] = (out_deg[v_id] as i32) - (in_deg[v_id] as i32);
        }

        Eulerian {
            unused_edges,
            out_deg,
            in_deg,
            diff_deg,
            id_map,
            trail: vec![],

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_ty: PhantomData,
            phantom_g: PhantomData,
        }
    }

    // Finds id of the vertex to start the trail/circuit from.
    fn find_start_virt_id(&self) -> Option<usize> {
        let unique_start = if Ty::is_undirected() {
            self.out_deg.iter().position(|out_deg| (*out_deg % 2) != 0)
        } else {
            self.diff_deg.iter().position(|diff| *diff == 1)
        };

        unique_start.or(self.out_deg.iter().position(|out_deg| *out_deg > 0))
    }

    /// Finds id of the vertex to start the eulerian trail from.
    ///
    /// # Returns
    /// * `Some`: Containing the id of the starting vertex.
    /// * `None`: If can not find a starting vertex for eulerian trail.
    pub fn start_of_eulerian_trail(&self) -> Option<usize> {
        let has_trail = if Ty::is_undirected() {
            let num_of_odd_degrees = self
                .in_deg
                .iter()
                .filter(|in_deg| (**in_deg % 2) != 0)
                .count();
            num_of_odd_degrees == 0 || num_of_odd_degrees == 2
        } else {
            let pos_diff: i32 = self.diff_deg.iter().filter(|diff| **diff > 0).sum();

            pos_diff == 1 || pos_diff == 0
        };

        if has_trail {
            self.find_start_virt_id()
        } else {
            None
        }
    }

    /// Finds id of the vertex to start the eulerian circuit from.
    ///
    /// # Returns
    /// * `Some`: Containing the id of the starting vertex.
    /// * `None`: If can not find a starting vertex for eulerian circuit.
    pub fn start_of_eulerian_circuit(&self) -> Option<usize> {
        let has_circuit = if Ty::is_undirected() {
            self.in_deg.iter().all(|in_deg| (*in_deg % 2) == 0)
        } else {
            self.diff_deg.iter().all(|diff| *diff == 0)
        };

        if has_circuit {
            self.find_start_virt_id()
        } else {
            None
        }
    }

    /// Finds the eulerian trail if there is one.
    ///
    /// # Arguments
    /// `graph`: Graph to search the eulerian trail in it.
    ///
    /// # Returns
    /// List of vector ids that will get visited during the eulerian trail.
    ///
    /// # Panics
    /// If graph does not have eulerian trail.
    pub fn find_trail(mut self, graph: &G) -> Vec<usize> {
        if self.out_deg.len() <= 1 {
            return self.trail;
        }

        let trail_start_id = self.start_of_eulerian_trail();
        let circuit_start_id = self.start_of_eulerian_circuit();

        if trail_start_id.is_none() {
            panic!("Graph does not have Eulerian trail.");
        }

        if !self.unused_edges.is_empty() {
            self.rec_execute(graph, trail_start_id.unwrap());
        }

        self.trail.reverse();

        if circuit_start_id.is_some() {
            self.trail.pop();
        }

        self.trail
    }

    /// Finds the eulerian circuit if there is one.
    ///
    /// # Arguments
    /// `graph`: Graph to search the eulerian circuit in it.
    ///
    /// # Returns
    /// List of vector ids that will get visited during the eulerian circuit.
    ///
    /// # Panics
    /// If graph does not have eulerian circuit.
    pub fn find_circuit(mut self, graph: &G) -> Vec<usize> {
        if self.out_deg.len() <= 1 {
            return self.trail;
        }
        let circuit_start_id = self.start_of_eulerian_circuit();

        if circuit_start_id.is_none() {
            panic!("Graph does not have Eulerian circuit.");
        }

        if !self.unused_edges.is_empty() {
            self.rec_execute(graph, circuit_start_id.unwrap());
        }

        self.trail.reverse();

        self.trail
    }

    // Recursively find the next vertex id in the trail/circuit.
    fn rec_execute(&mut self, graph: &G, v_virt_id: usize) {
        let v_real_id = self.id_map.real_id_of(v_virt_id);

        if self.out_deg[v_virt_id] == 0 {
            self.trail.push(v_real_id)
        } else {
            for (dst_real_id, edge) in graph.edges_from_unchecked(v_real_id) {
                let dst_virt_id = self.id_map.virt_id_of(dst_real_id);
                if self.unused_edges.contains(&edge.get_id()) {
                    self.unused_edges.remove(&edge.get_id());

                    self.out_deg[v_virt_id] -= 1;

                    if Ty::is_undirected() {
                        self.out_deg[dst_virt_id] -= 1;
                    }

                    self.rec_execute(graph, dst_virt_id);
                }
            }
            self.trail.push(v_real_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{graph::MatGraph, storage::DiMat, storage::Mat};

    #[test]
    fn one_vertex_directed_graph_trail() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        graph.add_vertex();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 0);
    }

    #[test]
    fn one_vertex_directed_graph_circuit() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        graph.add_vertex();

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn one_vertex_undirected_graph_trail() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 0);
    }

    #[test]
    fn one_vertex_undirected_graph_circuit() {
        // Given: Graph
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(circuit.len(), 0);
    }

    #[test]
    fn trivial_directed_graph_with_eulerian_trail() {
        // Given:
        //
        //      a --> b --> c
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 3);
        assert_eq!(trail, vec![a, b, c]);
    }

    #[test]
    fn trivial_undirected_graph_with_eulerian_trail() {
        // Given:
        //
        //      a --- b --- c
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 3);
        assert_eq!(trail, vec![a, b, c]);
    }

    #[test]
    fn trivial_directed_graph_with_eulerian_circuit() {
        // Given: Graph
        //
        //      a --> b
        //      ^     |
        //      |     v
        //      '---- c
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 3);
        assert_eq!(trail, vec![a, b, c,]);
    }

    #[test]
    fn trivial_directed_graph_with_eulerian_circuit2() {
        // Given: Graph
        //
        //      a --> b
        //      ^     |
        //      |     v
        //      '---- c
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(circuit.len(), 4);
        assert_eq!(circuit, vec![a, b, c, a]);
    }
    #[test]
    fn trivial_undirected_graph_with_eulerian_circuit() {
        // Given: Graph
        //
        //      a --- b
        //      |     |
        //      |     |
        //      '---- c
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 3);
        assert_eq!(trail, vec![a, b, c]);
    }

    #[test]
    fn trivial_undirected_graph_with_eulerian_circuit2() {
        // Given: Graph
        //
        //      a --- b
        //      |     |
        //      |     |
        //      '---- c
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(circuit.len(), 4);
        assert_eq!(circuit, vec![a, b, c, a]);
    }

    #[test]
    fn complex_undirected_graph_with_eulerian_trail() {
        // Given:
        //
        //      a --- b --.
        //     /  \       |
        //    e    \      |
        //     \    \     |
        //      c -- d    |
        //      |_________'
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(a, e, 1.into());
        graph.add_edge_unchecked(a, d, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(c, e, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 7);
        assert_eq!(trail, vec![a, b, c, d, a, e, c]);
    }

    #[test]
    fn complex_directed_graph_with_eulerian_trail() {
        // Given:
        //
        //       |`````|     .-------.
        //       v     |     |       |
        //       a -.  f <-- e <--.  |
        //       |  |        ^    |  |
        //       |  V        |    |  |
        //       |  g -------'    |  |
        //       v                |  |
        //       b --> c --> d ---'  |
        //             ^             |
        //             |_____________'
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(d, e, 1.into());
        graph.add_edge_unchecked(e, f, 1.into());
        graph.add_edge_unchecked(f, a, 1.into());
        graph.add_edge_unchecked(a, g, 1.into());
        graph.add_edge_unchecked(g, e, 1.into());
        graph.add_edge_unchecked(e, c, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 10);
        assert_eq!(trail, vec![a, b, c, d, e, f, a, g, e, c]);
    }

    #[test]
    fn complex_undirected_graph_with_eulerian_circuit() {
        // Given:
        //
        //      a --- b --.
        //     /| \       |
        //    e |  \      |
        //     \|   \     |
        //      c -- d    |
        //      |_________'
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(a, e, 1.into());
        graph.add_edge_unchecked(a, d, 1.into());
        graph.add_edge_unchecked(a, c, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(c, e, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(trail.len(), 8);
        assert_eq!(trail, vec![a, b, c, a, d, c, e, a]);
    }

    #[test]
    fn complex_directed_graph_with_eulerian_circuit() {
        // Given:
        //
        //        |`````|     .-------.
        //        v     |     |       |
        //   .--> a -.  f <-- e <--.  |
        //   |    |  |        ^    |  |
        //   |    |  V        |    |  |
        //   |    |  g -------'    |  |
        //   |    v                |  |
        //   |    b --> c --> d ---'  |
        //   |         /^             |
        //   .________/ |_____________'
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(d, e, 1.into());
        graph.add_edge_unchecked(e, f, 1.into());
        graph.add_edge_unchecked(f, a, 1.into());
        graph.add_edge_unchecked(a, g, 1.into());
        graph.add_edge_unchecked(g, e, 1.into());
        graph.add_edge_unchecked(e, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_circuit(&graph);

        // Then:
        assert_eq!(trail.len(), 11);
        assert_eq!(trail, vec![a, b, c, a, g, e, c, d, e, f, a]);
    }

    #[test]
    fn directed_graph_with_start_other_than_0() {
        // Given:
        //
        //  d --- a ----.
        //        |     |
        //        c --- b
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let b = graph.add_vertex();
        let d = graph.add_vertex();
        let c = graph.add_vertex();
        let a = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());
        graph.add_edge_unchecked(a, d, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then: It should start from a or d and not b.
        assert_eq!(trail.len(), 5);
        assert_eq!(trail, vec![1, 3, 0, 2, 3]);
    }

    #[test]
    fn undirected_graph_with_start_other_than_0() {
        // Given:
        //
        //  d <-- a <---.
        //        |     |
        //        v     |
        //        c --> b
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let b = graph.add_vertex();
        let d = graph.add_vertex();
        let c = graph.add_vertex();
        let a = graph.add_vertex();
        graph.add_edge_unchecked(a, c, 1.into());
        graph.add_edge_unchecked(c, b, 1.into());
        graph.add_edge_unchecked(b, a, 1.into());
        graph.add_edge_unchecked(a, d, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then: It should start from a or d and not b.
        assert_eq!(trail.len(), 5);
        assert_eq!(trail, vec![3, 2, 0, 3, 1]);
    }

    #[test]
    fn calling_trail_on_undirected_graph_with_circuit() {
        // Given:
        //
        //      a --- b --.
        //     /| \       |
        //    e |  \      |
        //     \|   \     |
        //      c -- d    |
        //      |_________'
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(a, e, 1.into());
        graph.add_edge_unchecked(a, d, 1.into());
        graph.add_edge_unchecked(a, c, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(c, e, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 7);
        assert_eq!(trail, vec![a, b, c, a, d, c, e]);
    }

    #[test]
    fn calling_trail_on_directed_graph_with_circuit() {
        // Given:
        //
        //        |`````|     .-------.
        //        v     |     |       |
        //   .--> a -.  f <-- e <--.  |
        //   |    |  |        ^    |  |
        //   |    |  V        |    |  |
        //   |    |  g -------'    |  |
        //   |    v                |  |
        //   |    b --> c --> d ---'  |
        //   |         /^             |
        //   .________/ |_____________'
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        graph.add_edge_unchecked(a, b, 1.into());
        graph.add_edge_unchecked(b, c, 1.into());
        graph.add_edge_unchecked(c, d, 1.into());
        graph.add_edge_unchecked(d, e, 1.into());
        graph.add_edge_unchecked(e, f, 1.into());
        graph.add_edge_unchecked(f, a, 1.into());
        graph.add_edge_unchecked(a, g, 1.into());
        graph.add_edge_unchecked(g, e, 1.into());
        graph.add_edge_unchecked(e, c, 1.into());
        graph.add_edge_unchecked(c, a, 1.into());

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph);

        // Then:
        assert_eq!(trail.len(), 10);
        assert_eq!(trail, vec![a, b, c, a, g, e, c, d, e, f]);
    }
}
