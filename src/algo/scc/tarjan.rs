use magnitude::Magnitude;

use crate::graph::{DirectedEdge, Edge};
use crate::provide;

pub struct TarjanSCC {
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    index_of: Vec<Magnitude<usize>>,
    low_link_of: Vec<Magnitude<usize>>,
    index: usize,
    id_map: provide::IdMap,
    scc: Vec<Vec<usize>>,
}

impl TarjanSCC {
    pub fn init<W, E: Edge<W>, G>(graph: &G) -> Self
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        let vertex_count = graph.vertex_count();

        TarjanSCC {
            stack: vec![],
            on_stack: vec![false; vertex_count],
            index_of: vec![Magnitude::PosInfinite; vertex_count],
            low_link_of: vec![Magnitude::PosInfinite; vertex_count],
            index: 0,
            scc: vec![],
            id_map: graph.continuos_id_map(),
        }
    }

    pub fn execute<W, E: Edge<W>, G>(mut self, graph: &G) -> Vec<Vec<usize>>
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        for virt_id in 0..graph.vertex_count() {
            if self.index_of[virt_id].is_pos_infinite() {
                self._execute(graph, virt_id);
            }
        }

        self.scc
    }

    pub fn _execute<W, E: Edge<W>, G>(&mut self, graph: &G, virt_id: usize)
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        self.index_of[virt_id] = self.index.into();
        self.low_link_of[virt_id] = self.index.into();

        self.index += 1;

        self.stack.push(virt_id);
        self.on_stack[virt_id] = true;

        let real_id = self.id_map.get_virt_to_real(virt_id).unwrap();

        for dst_real_id in graph.neighbors(real_id) {
            let dst_virt_id = self.id_map.get_real_to_virt(dst_real_id).unwrap();

            if self.index_of[dst_virt_id].is_pos_infinite() {
                self._execute(graph, dst_virt_id);

                self.low_link_of[virt_id] =
                    std::cmp::min(self.low_link_of[virt_id], self.low_link_of[dst_virt_id]);
            } else if self.on_stack[dst_virt_id] {
                self.low_link_of[virt_id] =
                    std::cmp::min(self.low_link_of[virt_id], self.low_link_of[dst_virt_id]);
            }
        }

        if self.low_link_of[virt_id] == self.index_of[virt_id] {
            let mut scc = Vec::<usize>::new();
            loop {
                let w_virt_id = self.stack.pop().unwrap();
                self.on_stack[w_virt_id] = false;

                // add w to new scc
                scc.push(self.id_map.get_virt_to_real(w_virt_id).unwrap());
                if w_virt_id == virt_id {
                    break;
                }
            }

            self.scc.push(scc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::DiMat;

    #[test]
    fn empty_graph() {
        let graph = MatGraph::init(DiMat::<usize>::init());

        let sccs = TarjanSCC::init(&graph).execute(&graph);

        assert_eq!(sccs.len(), 0);
    }

    #[test]
    fn single_component_graph() {
        // Given: Graph
        //
        //     a ----> b
        //     ^       |
        //     |       |
        //     c <-----'
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        // When: Performing Tarjan.
        let sccs = TarjanSCC::init(&graph).execute(&graph);

        // Then:
        assert_eq!(sccs.len(), 1);
        assert!(vec![a, b, c].iter().all(|v_id| sccs[0].contains(v_id)));
    }

    #[test]
    fn graph_with_no_edge() {
        // Given: Graph
        //
        //      a   b   c
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();

        // When: Preforming Tarjan.
        let sccs = TarjanSCC::init(&graph).execute(&graph);

        // Then:
        assert_eq!(sccs.len(), 3);
        assert_eq!(sccs.concat(), [a, b, c]);
    }

    #[test]
    fn trivial_graph() {
        //
        //              .--- e <--.
        //              |         |
        //              v         |
        //      a  -->  b    -->  f  -->  g <----.
        //     ^ |      |   /     |      /       |
        //     | |      |  /      |     /        |
        //     | v      v /       v    /         |
        //      d  -->  c  ---->  h <------- i --'
        //                        |          ^
        //                        |          |
        //                        ````````````
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        let g = graph.add_vertex();
        let h = graph.add_vertex();
        let i = graph.add_vertex();

        graph.add_edge(a, d, 1.into());
        graph.add_edge(d, a, 1.into());
        graph.add_edge(a, b, 1.into());
        graph.add_edge(d, c, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(f, e, 1.into());
        graph.add_edge(e, b, 1.into());
        graph.add_edge(f, h, 1.into());
        graph.add_edge(c, h, 1.into());
        graph.add_edge(c, f, 1.into());
        graph.add_edge(f, g, 1.into());
        graph.add_edge(h, i, 1.into());
        graph.add_edge(g, h, 1.into());
        graph.add_edge(i, g, 1.into());

        let sccs = TarjanSCC::init(&graph).execute(&graph);

        for scc in sccs {
            match scc.len() {
                2 => assert!(vec![d, a].iter().all(|v_id| scc.contains(v_id))),
                3 => assert!(vec![i, h, g].iter().all(|v_id| scc.contains(v_id))),
                4 => assert!(vec![e, f, c, b].iter().all(|v_id| scc.contains(v_id))),
                _ => unreachable!("Unknown scc"),
            }
        }
    }
}
