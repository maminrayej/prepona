use crate::algo::{Dfs, DfsListener};
use crate::graph::{DirectedEdge, Edge};
use crate::provide;

pub struct TopologicalSort {
    sorted_vertex_ids: Vec<usize>,
}

impl DfsListener for TopologicalSort {
    fn on_black(&mut self, _: &Dfs<Self>, virt_id: usize) {
        self.sorted_vertex_ids.push(virt_id);
    }
}

impl TopologicalSort {
    pub fn init() -> Self {
        TopologicalSort {
            sorted_vertex_ids: vec![],
        }
    }

    pub fn execute<W, E: Edge<W>, G>(mut self, graph: &G) -> Vec<usize>
    where
        G: provide::Graph<W, E, DirectedEdge> + provide::Vertices + provide::Neighbors,
    {
        let dfs = Dfs::init(graph, &mut self);

        dfs.execute(graph);

        let id_map = dfs.id_map();

        self.sorted_vertex_ids.reverse();

        self.sorted_vertex_ids
            .into_iter()
            .map(|virt_id| id_map.get_virt_to_real(virt_id).unwrap())
            .collect()
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

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 0);
    }

    #[test]
    fn one_vertex_graph() {
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let _ = graph.add_vertex();

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 1);
    }

    #[test]
    fn trivial_graph() {
        // Given: Graph
        //
        //      a  -->  b  -->  c  -->  f
        //      |        \      |
        //      |         '-----|
        //      |               |
        //      |               v
        //      '-----> d  -->  e
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge((a, b, 1).into());
        graph.add_edge((a, d, 1).into());
        graph.add_edge((b, c, 1).into());
        graph.add_edge((b, e, 1).into());
        graph.add_edge((d, e, 1).into());
        graph.add_edge((c, e, 1).into());
        graph.add_edge((c, f, 1).into());

        let sorted_vertices = TopologicalSort::init().execute(&graph);

        assert_eq!(sorted_vertices.len(), 6);
        for edge in graph.edges() {
            let (src_id, dst_id) = (edge.get_src_id(), edge.get_dst_id());

            // src must appear before dst in topological sort
            let src_index = sorted_vertices
                .iter()
                .position(|v_id| *v_id == src_id)
                .unwrap();
            let dst_index = sorted_vertices
                .iter()
                .position(|v_id| *v_id == dst_id)
                .unwrap();

            assert!(src_index < dst_index)
        }
    }
}
