use std::{collections::HashSet, marker::PhantomData};

use anyhow::Result;

use crate::{
    algo::Error,
    graph::{Edge, EdgeDir},
    provide::{Edges, Graph, IdMap, Vertices},
};

/// Finds Eulerian trail and circuit.
///
/// # Examples
/// ```
/// use prepona::prelude::*;
/// use prepona::algo::Eulerian;
/// use prepona::storage::Mat;
/// use prepona::graph::MatGraph;
///
/// // Given:
/// //
/// //      a --- b --.
/// //     /  \       |
/// //    e    \      |
/// //     \    \     |
/// //      c -- d    |
/// //      |_________'
/// //
/// let mut graph = MatGraph::init(Mat::<usize>::init());
/// let a = graph.add_vertex();
/// let b = graph.add_vertex();
/// let c = graph.add_vertex();
/// let d = graph.add_vertex();
/// let e = graph.add_vertex();
/// graph.add_edge(a, b, 1.into());
/// graph.add_edge(a, e, 1.into());
/// graph.add_edge(a, d, 1.into());
/// graph.add_edge(b, c, 1.into());
/// graph.add_edge(c, d, 1.into());
/// graph.add_edge(c, e, 1.into());
///
/// // When: Performing Eulerian trail detection algorithm.
/// let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();
///
/// // Then:
/// assert_eq!(trail.len(), 7);
/// assert_eq!(trail, vec![a, b, c, d, a, e, c]);
/// ```
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
    /// Initializes the structure.
    ///
    /// # Arguments
    /// `graph`: The graph to find the Eulerian trail or circuit in.
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

    /// # Returns
    /// * `Some`: Containing the id of the vertex that is suitable for starting the Eulerian algorithm from.
    /// * `None`: If there is no suitable vertex.
    fn find_start_virt_id(&self) -> Option<usize> {
        // If any of the bellow searches result in a vertex id, then there is a unique vertex that is suitable for starting the search from.
        let unique_start = if Ty::is_undirected() {
            // If graph is undirected the vertex with an odd out degree is suitable to start the search from.
            self.out_deg.iter().position(|out_deg| (*out_deg % 2) != 0)
        } else {
            // If graph is directed the vertex with out_degree - in_degree = 1 is suitable to start the search from.
            self.diff_deg.iter().position(|diff| *diff == 1)
        };

        // If there is no unique suitable vertex to start the algorithm from, any vertex with an outgoing edge is suitable.
        unique_start.or(self.out_deg.iter().position(|out_deg| *out_deg > 0))
    }

    /// Finds id of the vertex to start the Eulerian trail from.
    ///
    /// # Returns
    /// * `Some`: Containing the id of the starting vertex of Eulerian trail.
    /// * `None`: If can not find a starting vertex for Eulerian trail.
    pub fn start_of_eulerian_trail(&self) -> Option<usize> {
        let has_trail = if Ty::is_undirected() {
            // An undirected graph has an Eulerian trail if and only if exactly zero or two vertices have odd degree.
            let num_of_odd_degrees = self
                .in_deg
                .iter()
                .filter(|in_deg| (**in_deg % 2) != 0)
                .count();
            num_of_odd_degrees == 0 || num_of_odd_degrees == 2
        } else {
            // A directed graph has an Eulerian trail if and only if at most one vertex has (out-degree) − (in-degree) = 1
            let pos_diff = self.diff_deg.iter().filter(|diff| **diff == 1).count();

            pos_diff == 1 || pos_diff == 0
        };

        has_trail.then(|| self.find_start_virt_id()).flatten()
    }

    /// Finds id of the vertex to start the Eulerian circuit from.
    ///
    /// # Returns
    /// * `Some`: Containing the id of the starting vertex.
    /// * `None`: If can not find a starting vertex for Eulerian circuit.
    pub fn start_of_eulerian_circuit(&self) -> Option<usize> {
        let has_circuit = if Ty::is_undirected() {
            // An undirected graph has an Eulerian cycle if and only if every vertex has even degree.
            self.in_deg.iter().all(|in_deg| (*in_deg % 2) == 0)
        } else {
            // A directed graph has an Eulerian cycle if and only if every vertex has equal in degree and out degree(in other words out-degree - in-degree = 0).
            self.diff_deg.iter().all(|diff| *diff == 0)
        };

        has_circuit.then(|| self.find_start_virt_id()).flatten()
    }

    /// Finds the Eulerian trail if there is one.
    ///
    /// # Arguments
    /// `graph`: Graph to search the Eulerian trail in it.
    ///
    /// # Returns
    /// * `Ok`: Containing list of vector ids that will get visited during the Eulerian trail.
    /// * `Err`: If graph does not have Eulerian trail.
    pub fn find_trail(mut self, graph: &G) -> Result<Vec<usize>> {
        // If graph has only one vertex, that single vertex is an Eulerian trail.
        if self.out_deg.len() <= 1 {
            return Ok(self.trail);
        }

        let trail_start_id = self.start_of_eulerian_trail();
        let circuit_start_id = self.start_of_eulerian_circuit();

        // If there is no suitable id to start the search from, graph does not have Eulerian trail.
        if trail_start_id.is_none() {
            Err(Error::new_etnf())?
        }

        // Make sure graph has at least one edge to traverse.
        if !self.unused_edges.is_empty() {
            self.rec_execute(graph, trail_start_id.unwrap());
        }

        // Trail actually has the visited vertices in the backward order. So first visited vertex is the last item in the `trail` structure.
        // So for trail to make sense to the user, reverse it before returning it so that first visited vertex is also the first item in the structure.
        self.trail.reverse();

        // IMPORTANT: Note that recursive algorithm that finds the trail actually finds the circuit if the graph also has Eulerian circuit.
        // So if graph also has an Eulerian circuit, pop the last item in the circuit for it to become a trail. This will convert (v1, v2, v3, v1) to (v1, v2, v3)
        if circuit_start_id.is_some() {
            self.trail.pop();
        }

        Ok(self.trail)
    }

    /// Finds the Eulerian circuit if there is one.
    ///
    /// # Arguments
    /// `graph`: Graph to search the eulerian circuit in it.
    ///
    /// # Returns
    /// * `Ok`: Containing list of vector ids that will get visited during the eulerian circuit.
    /// * `Err`: If graph does not have Eulerian circuit.
    pub fn find_circuit(mut self, graph: &G) -> Result<Vec<usize>> {
        // If graph has only one vertex, that single vertex is an Eulerian circuit.
        if self.out_deg.len() <= 1 {
            return Ok(self.trail);
        }
        let circuit_start_id = self.start_of_eulerian_circuit();

        // If there is no suitable id to start the search from, graph does not have Eulerian circuit.
        if circuit_start_id.is_none() {
            Err(Error::new_ecnf())?
        }

        // Make sure graph has at least one edge to traverse.
        if !self.unused_edges.is_empty() {
            self.rec_execute(graph, circuit_start_id.unwrap());
        }

        // Trail actually has the visited vertices in the backward order. So first visited vertex is the last item in the `trail` structure.
        // So for trail to make sense to the user, reverse it before returning it so that first visited vertex is also the first item in the structure.
        self.trail.reverse();

        Ok(self.trail)
    }

    // Recursively find the next vertex id in the Eulerian trail/circuit.
    fn rec_execute(&mut self, graph: &G, v_virt_id: usize) {
        let v_real_id = self.id_map.real_id_of(v_virt_id);

        if self.out_deg[v_virt_id] == 0 {
            self.trail.push(v_real_id)
        } else {
            for (dst_real_id, edge) in graph.edges_from(v_real_id).unwrap() {
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
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        let circuit = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        let circuit = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian circuit detection algorithm.
        let circuit = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, e, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(c, e, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();
        graph.add_edge(e, f, 1.into()).unwrap();
        graph.add_edge(f, a, 1.into()).unwrap();
        graph.add_edge(a, g, 1.into()).unwrap();
        graph.add_edge(g, e, 1.into()).unwrap();
        graph.add_edge(e, c, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, e, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();
        graph.add_edge(a, c, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(c, e, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();
        graph.add_edge(e, f, 1.into()).unwrap();
        graph.add_edge(f, a, 1.into()).unwrap();
        graph.add_edge(a, g, 1.into()).unwrap();
        graph.add_edge(g, e, 1.into()).unwrap();
        graph.add_edge(e, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_circuit(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, c, 1.into()).unwrap();
        graph.add_edge(c, b, 1.into()).unwrap();
        graph.add_edge(b, a, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, e, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();
        graph.add_edge(a, c, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(c, e, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

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
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();
        graph.add_edge(e, f, 1.into()).unwrap();
        graph.add_edge(f, a, 1.into()).unwrap();
        graph.add_edge(a, g, 1.into()).unwrap();
        graph.add_edge(g, e, 1.into()).unwrap();
        graph.add_edge(e, c, 1.into()).unwrap();
        graph.add_edge(c, a, 1.into()).unwrap();

        // When: Performing Eulerian trail detection algorithm.
        let trail = Eulerian::init(&graph).find_trail(&graph).unwrap();

        // Then:
        assert_eq!(trail.len(), 10);
        assert_eq!(trail, vec![a, b, c, a, g, e, c, d, e, f]);
    }

    // TODO: Add test for graphs that does not have Eulerian circuit or trail and check that algorithm returns error indeed.
}
