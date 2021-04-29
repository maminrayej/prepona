use std::{collections::HashSet, marker::PhantomData};

use provide::{Edges, Graph, Vertices};

use crate::graph::{subgraph::Subgraph, Edge, EdgeDir};
use crate::provide;

/// Detects cycle in a graph.
///
/// # Examples
/// ```
/// use prepona::prelude::*;
/// use prepona::storage::DiMat;
/// use prepona::graph::MatGraph;
/// use prepona::algo::HasCycle;
///
/// // Given: Graph
/// //
/// //      a  -->  b  -->  c  -->  d
/// //      ^        \              ^
/// //      |         \             |
/// //      |          -->  e  -----'
/// //      |               |
/// //      '_______________'
/// //
/// let mut graph = MatGraph::init(DiMat::<usize>::init());
/// let a = graph.add_vertex();
/// let b = graph.add_vertex();
/// let c = graph.add_vertex();
/// let d = graph.add_vertex();
/// let e = graph.add_vertex();
///
/// let ab = graph.add_edge(a, b, 1.into());
/// graph.add_edge(b, c, 1.into());
/// graph.add_edge(c, d, 1.into());
/// let be = graph.add_edge(b, e, 1.into());
/// graph.add_edge(e, d, 1.into());
/// let ea = graph.add_edge(e, a, 1.into());
///
/// // When: Performing cycle detection.
/// let cycle = HasCycle::init(&graph).execute(&graph).unwrap();
///
/// // Cycle contains vertices a,b and e.
/// assert_eq!(cycle.vertex_count(), 3);
/// assert!(vec![a, b, e].iter().all(|v_id| cycle.vertices().contains(v_id)));
///
/// // Cycle contains edges ab, be, ea.
/// assert_eq!(cycle.edges_count(), 3);
/// assert!(vec![ab, be, ea].iter().all(|edge_id| cycle.edge(*edge_id).is_ok()));
/// ```
pub struct HasCycle<'a, W, E: Edge<W>> {
    is_visited: Vec<bool>,
    is_finished: Vec<bool>,
    edge_stack: Vec<(usize, usize, &'a E)>,
    id_map: provide::IdMap,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
}

impl<'a, W, E: Edge<W>> HasCycle<'a, W, E> {
    /// # Arguments
    /// `graph`: Graph to search for cycle in it.
    pub fn init<Dir, G>(graph: &G) -> Self
    where
        Dir: EdgeDir,
        G: provide::Neighbors + Vertices + Graph<W, E, Dir>,
    {
        HasCycle {
            is_visited: vec![false; graph.vertex_count()],
            is_finished: vec![false; graph.vertex_count()],
            edge_stack: vec![],
            id_map: graph.continuos_id_map(),

            phantom_w: PhantomData,
            phantom_e: PhantomData,
        }
    }

    /// Performs cycle detection.
    ///
    /// # Arguments
    /// `graph`: Graph to search for cycles in it.
    ///
    /// # Returns
    /// * `Some`: Containing the found cycle in the form of a subgraph.
    /// * `None`: If graph does not have any cycle.
    pub fn execute<Dir, G>(mut self, graph: &'a G) -> Option<Subgraph<W, E, Dir, G>>
    where
        E: Edge<W>,
        Dir: EdgeDir,
        G: provide::Neighbors + Vertices + Graph<W, E, Dir> + Edges<W, E>,
    {
        if graph.vertex_count() != 0 && self.has_cycle(graph, 0, 0) {
            let vertices = self
                .edge_stack
                .iter()
                .flat_map(|(src_id, dst_id, _)| vec![*src_id, *dst_id])
                .collect::<HashSet<usize>>();

            Some(Subgraph::init(graph, self.edge_stack, vertices))
        } else {
            None
        }
    }

    // Recursively searches for cycles in graph
    //
    // # Arguments
    // * `graph`: Graph to search for cycles in it.
    // * `src_virt_id`: Virtual id of the current vertex.
    // * `parent_virt_id`: Virtual id of the parent of the current vertex.
    //
    // # Returns
    // * `true`: If graph has cycle.
    // * `false`: Otherwise.
    fn has_cycle<Dir, G>(&mut self, graph: &'a G, src_virt_id: usize, parent_virt_id: usize) -> bool
    where
        E: Edge<W>,
        Dir: EdgeDir,
        G: provide::Neighbors + Vertices + Graph<W, E, Dir> + Edges<W, E>,
    {
        self.is_visited[src_virt_id] = true;

        let src_real_id = self.id_map.real_id_of(src_virt_id);

        for (dst_real_id, edge) in graph.edges_from(src_real_id).unwrap() {
            let dst_virt_id = self.id_map.virt_id_of(dst_real_id);

            // If child is not already visited, try to find a cycle that contains (src_id, dst_id) edge.
            if !self.is_visited[dst_virt_id] {
                self.edge_stack.push((src_real_id, dst_real_id, edge));
                if self.has_cycle(graph, dst_virt_id, src_virt_id) {
                    return true;
                } else {
                    // If failed to find any cycle that contains this edge, remove it from stack.
                    self.edge_stack.pop();
                }
            } else if !self.is_finished[dst_virt_id]
                && (Dir::is_directed() || dst_virt_id != parent_virt_id)
            {
                // If child is visited but not finished, means we visited it once before and got back to it using a different route.
                // But for this route to be cycle, the graph either:
                // * Must be directed to prevent detecting: v1 --- v2 as cycle.
                // * Is undirected which child can not be parent of the current vertex. Again to prevent detecting cycle in scenario: v1 --- v2.
                self.edge_stack.push((src_real_id, dst_real_id, edge));
                return true;
            }
        }

        self.is_finished[src_virt_id] = true;

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::storage::{DiMat, Mat};
    use provide::*;

    #[test]
    fn empty_directed_graph() {
        // Given: An empty graph.
        let graph = MatGraph::init(DiMat::<usize>::init());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert!(cycle.is_none());
    }

    #[test]
    fn empty_undirected_graph() {
        // Given: An empty graph.
        let graph = MatGraph::init(Mat::<usize>::init());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert!(cycle.is_none());
    }

    #[test]
    fn two_node_directed() {
        // Given:
        //
        //      a --> b
        //      ^     |
        //      |_____|
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let ab = graph.add_edge(a, b, 1.into()).unwrap();
        let ba = graph.add_edge(b, a, 2.into()).unwrap();

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| cycle.vertices().contains(vertex_id)));

        for (src_id, dst_id, edge) in cycle.edges() {
            match (src_id, dst_id) {
                (0, 1) => {
                    assert_eq!(edge.get_id(), ab);
                    assert_eq!(*edge.get_weight(), 1.into());
                }
                (1, 0) => {
                    assert_eq!(edge.get_id(), ba);
                    assert_eq!(*edge.get_weight(), 2.into())
                }
                _ => unreachable!("Unkown edge"),
            }
        }
    }

    #[test]
    fn two_node_undirected() {
        // Given:
        //
        //  a --- b
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let _ = graph.add_edge(a, b, 1.into());

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert!(cycle.is_none());
    }

    #[test]
    fn trivial_directed_graph_without_cycle() {
        // Given: Graph
        //
        //      a  -->  b  -->  c
        //      |       |
        //      v       v
        //      d  -->  e
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(a, d, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(b, e, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        assert!(cycle.is_none());
    }

    #[test]
    fn trivial_undirected_graph_without_cycle() {
        // Given: Graph
        //
        //      a  ---  b  ---  c
        //              |
        //              |
        //      d  ---  e
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(b, e, 1.into()).unwrap();
        graph.add_edge(d, e, 1.into()).unwrap();

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        // Then:
        assert!(cycle.is_none());
    }

    #[test]
    fn trivial_directed_graph_with_cycle() {
        // Given: Graph
        //
        //      a  -->  b  -->  c  -->  d
        //      ^        \              ^
        //      |         \             |
        //      |          -->  e  -----'
        //      |               |
        //      '_______________'
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        let ab = graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        let be = graph.add_edge(b, e, 1.into()).unwrap();
        graph.add_edge(e, d, 1.into()).unwrap();
        let ea = graph.add_edge(e, a, 1.into()).unwrap();

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        assert!(cycle.is_some());
        let cycle = cycle.unwrap();

        assert_eq!(cycle.vertex_count(), 3);
        assert!(vec![a, b, e]
            .iter()
            .all(|v_id| cycle.vertices().contains(v_id)));

        assert_eq!(cycle.edges_count(), 3);
        assert!(vec![ab, be, ea]
            .iter()
            .all(|edge_id| cycle.edge(*edge_id).is_ok()));
    }

    #[test]
    fn trivial_undirected_graph() {
        // Given: Graph
        //
        //      a  ---  b  ---  c  ---  d
        //      |        \
        //      |         \
        //      |          ---  e
        //      |               |
        //      '_______________'
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();

        let ab = graph.add_edge(a, b, 1.into()).unwrap();
        graph.add_edge(b, c, 1.into()).unwrap();
        graph.add_edge(c, d, 1.into()).unwrap();
        let be = graph.add_edge(b, e, 1.into()).unwrap();
        let ea = graph.add_edge(e, a, 1.into()).unwrap();

        // When: Performing cycle detection.
        let cycle = HasCycle::init(&graph).execute(&graph);

        assert!(cycle.is_some());
        let cycle = cycle.unwrap();

        assert!(vec![a, b, e]
            .iter()
            .all(|v_id| cycle.vertices().contains(v_id)));

        assert_eq!(cycle.edges_count(), 3);
        assert!(vec![ab, be, ea]
            .iter()
            .all(|edge_id| cycle.edge(*edge_id).is_ok()));
    }
}
