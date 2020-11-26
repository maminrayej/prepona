use std::marker::PhantomData;

use crate::graph::{subgraph::Subgraph, Edge};
use crate::provide;

pub struct HasCycle<'a, W, E: Edge<W>> {
    is_visited: Vec<bool>,
    is_finished: Vec<bool>,
    edge_stack: Vec<(usize, usize, &'a E)>,
    id_map: provide::IdMap,

    phantom_w: PhantomData<W>,
}

impl<'a, W, E: Edge<W>> HasCycle<'a, W, E> {
    pub fn init<G>(graph: &G) -> Self
    where
        G: provide::Neighbors + provide::Vertices + provide::Direction,
    {
        HasCycle {
            is_visited: vec![false; graph.vertex_count()],
            is_finished: vec![false; graph.vertex_count()],
            edge_stack: vec![],
            id_map: graph.continuos_id_map(),

            phantom_w: PhantomData,
        }
    }

    pub fn execute<G>(mut self, graph: &'a G) -> Option<Subgraph<W, E>>
    where
        G: provide::Edges<W, E> + provide::Vertices + provide::Direction,
    {
        if graph.vertex_count() != 0 && self.has_cycle(graph, 0, 0) {
            Some(Subgraph::init(self.edge_stack))
        } else {
            None
        }
    }

    fn has_cycle<G>(&mut self, graph: &'a G, src_virt_id: usize, parent_virt_id: usize) -> bool
    where
        G: provide::Edges<W, E> + provide::Vertices + provide::Direction,
    {
        self.is_visited[src_virt_id] = true;

        let src_real_id = self.id_map.real_id_of(src_virt_id);

        for (dst_real_id, edge) in graph.edges_from(src_real_id) {
            let dst_virt_id = self.id_map.virt_id_of(dst_real_id);

            if !self.is_visited[dst_virt_id] {
                self.edge_stack.push((src_real_id, dst_real_id, edge));
                if self.has_cycle(graph, dst_virt_id, src_virt_id) {
                    return true;
                } else {
                    self.edge_stack.pop();
                }
            } else if !self.is_finished[dst_virt_id]
                && (G::is_directed() || dst_virt_id != parent_virt_id)
            {
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
        let ab = graph.add_edge(a, b, 1.into());
        let ba = graph.add_edge(b, a, 2.into());

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
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, d, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(b, e, 1.into());
        graph.add_edge(d, e, 1.into());

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
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(b, e, 1.into());
        graph.add_edge(d, e, 1.into());

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

        let ab = graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, d, 1.into());
        let be = graph.add_edge(b, e, 1.into());
        graph.add_edge(e, d, 1.into());
        let ea = graph.add_edge(e, a, 1.into());

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
            .all(|edge_id| cycle.edge(*edge_id).is_some()));
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

        let ab = graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, d, 1.into());
        let be = graph.add_edge(b, e, 1.into());
        let ea = graph.add_edge(e, a, 1.into());

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
            .all(|edge_id| cycle.edge(*edge_id).is_some()));
    }
}
