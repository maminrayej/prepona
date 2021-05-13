#[cfg(test)]
mod tests {
    use crate::{
        algo::{ConnectedComponents, TopologicalSort, VertexEdgeCut},
        graph::SimpleGraph,
        prelude::*,
        storage::AdjMatrix,
    };

    use rand::seq::SliceRandom;

    #[test]
    fn vertex_cut() {
        fn prop(
            mut graph: SimpleGraph<
                i32,
                DefaultEdge<i32>,
                UndirectedEdge,
                AdjMatrix<i32, DefaultEdge<i32>, UndirectedEdge>,
            >,
        ) -> bool {
            let before_ccs = ConnectedComponents::init(&graph).execute(&graph).len();

            let (cut_vertices, _) = VertexEdgeCut::init(&graph).execute(&graph);

            if !cut_vertices.is_empty() {
                let vertex_id = *cut_vertices.choose(&mut rand::thread_rng()).unwrap();

                graph.remove_vertex(vertex_id).unwrap();

                let after_ccs = ConnectedComponents::init(&graph).execute(&graph).len();

                assert!(after_ccs > before_ccs);
            }

            true
        }

        quickcheck::quickcheck(
            prop as fn(
                SimpleGraph<
                    i32,
                    DefaultEdge<i32>,
                    UndirectedEdge,
                    AdjMatrix<i32, DefaultEdge<i32>, UndirectedEdge>,
                >,
            ) -> bool,
        );
    }

    #[test]
    fn edge_cut() {
        fn prop(
            mut graph: SimpleGraph<
                i32,
                DefaultEdge<i32>,
                UndirectedEdge,
                AdjMatrix<i32, DefaultEdge<i32>, UndirectedEdge>,
            >,
        ) -> bool {
            let before_ccs = ConnectedComponents::init(&graph).execute(&graph).len();

            let (_, cut_edges) = VertexEdgeCut::init(&graph).execute(&graph);

            if !cut_edges.is_empty() {
                let (src_id, dst_id, &edge) = *cut_edges.choose(&mut rand::thread_rng()).unwrap();

                graph.remove_edge(src_id, dst_id, edge.get_id()).unwrap();

                let after_ccs = ConnectedComponents::init(&graph).execute(&graph).len();

                assert!(after_ccs > before_ccs);
            }

            true
        }

        quickcheck::quickcheck(
            prop as fn(
                SimpleGraph<
                    i32,
                    DefaultEdge<i32>,
                    UndirectedEdge,
                    AdjMatrix<i32, DefaultEdge<i32>, UndirectedEdge>,
                >,
            ) -> bool,
        );
    }

    #[test]
    fn topological_sort() {
        // TODO: Graph should be DAG.
        fn prop(
            graph: SimpleGraph<
                i32,
                DefaultEdge<i32>,
                DirectedEdge,
                AdjMatrix<i32, DefaultEdge<i32>, DirectedEdge>,
            >,
        ) -> bool {
            let sorted_vertices = TopologicalSort::init().execute(&graph);

            for (src_id, dst_id, _) in graph.edges() {
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

            true
        }

        quickcheck::quickcheck(
            prop as fn(
                SimpleGraph<
                    i32,
                    DefaultEdge<i32>,
                    DirectedEdge,
                    AdjMatrix<i32, DefaultEdge<i32>, DirectedEdge>,
                >,
            ) -> bool,
        );
    }
}
