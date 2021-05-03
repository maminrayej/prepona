#[cfg(test)]
mod prop_tests {
    use std::{collections::HashSet, fmt::Debug};

    use rand::seq::SliceRandom;

    use crate::{
        prelude::*,
        storage::{AdjList, AdjMap},
    };

    use crate::{
        graph::EdgeDir,
        prelude::{DefaultEdge, DirectedEdge, UndirectedEdge},
        storage::AdjMatrix,
    };

    #[test]
    fn add_vertex() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir>>(mut storage: S) -> bool {
            let before_vertex_count = storage.vertex_count();
            let before_edge_count = storage.edge_count();

            let vertex_id = storage.add_vertex();

            assert!(storage.vertex_count() == before_vertex_count + 1);
            assert!(storage.edge_count() == before_edge_count);
            assert!(storage.neighbors(vertex_id).unwrap().is_empty());
            assert!(storage.edges_from(vertex_id).unwrap().is_empty());
            assert!(!storage
                .edges()
                .iter()
                .any(|(src_id, dst_id, _)| *src_id == vertex_id || *dst_id == vertex_id));

            true
        }

        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, UndirectedEdge>) -> bool);

        quickcheck::quickcheck(prop as fn(AdjList<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjList<_, _, UndirectedEdge>) -> bool);

        quickcheck::quickcheck(prop as fn(AdjMap<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMap<_, _, UndirectedEdge>) -> bool);
    }

    #[test]
    fn remove_vertex() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            mut storage: S,
        ) -> bool {
            if storage.vertex_count() > 0 {
                // Choose a random vertex id to remove from storage.
                let vertex_id = *storage.vertices().choose(&mut rand::thread_rng()).unwrap();

                let before_vertex_count = storage.vertex_count();
                let before_edge_count = storage.edge_count();
                let edges_associated_with_vertex: HashSet<usize> = storage
                    .edges()
                    .iter()
                    .filter_map(|(src_id, dst_id, edge)| {
                        if *src_id == vertex_id || *dst_id == vertex_id {
                            Some(edge.get_id())
                        } else {
                            None
                        }
                    })
                    .collect();

                storage.remove_vertex(vertex_id).unwrap();

                assert!(storage.vertex_count() == before_vertex_count - 1);
                assert!(
                    storage.edge_count() == before_edge_count - edges_associated_with_vertex.len()
                );
                assert!(!storage
                    .edges()
                    .iter()
                    .any(|(_, _, edge)| edges_associated_with_vertex.contains(&edge.get_id())))
            }

            true
        }

        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, UndirectedEdge>) -> bool);

        quickcheck::quickcheck(prop as fn(AdjList<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjList<_, _, UndirectedEdge>) -> bool);

        quickcheck::quickcheck(prop as fn(AdjMap<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMap<_, _, UndirectedEdge>) -> bool);
    }
}
