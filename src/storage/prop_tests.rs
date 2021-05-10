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

    #[test]
    fn contains_vertex() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            storage: S,
        ) -> bool {
            assert!(storage
                .vertices()
                .iter()
                .all(|vertex_id| storage.contains_vertex(*vertex_id)));

            assert!(storage
                .edges()
                .iter()
                .all(|(src_id, dst_id, _)| storage.contains_vertex(*src_id)
                    && storage.contains_vertex(*dst_id)));

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
    fn add_edge() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            mut storage: S,
        ) -> bool {
            if storage.vertex_count() > 0 {
                // Choose a src and dst randomly.
                let src_id = *storage.vertices().choose(&mut rand::thread_rng()).unwrap();
                let dst_id = *storage.vertices().choose(&mut rand::thread_rng()).unwrap();

                let before_vertex_count = storage.vertex_count();
                let before_edge_count = storage.edge_count();

                let edge_id = storage.add_edge(src_id, dst_id, 1.into()).unwrap();
                storage.add_edge(src_id, src_id, 1.into()).unwrap();

                assert!(storage.edge_count() == before_edge_count + 2);
                assert!(storage.vertex_count() == before_vertex_count);
                assert!(storage
                    .edges_from(src_id)
                    .unwrap()
                    .iter()
                    .find(|(d_id, edge)| *d_id == dst_id && edge.get_id() == edge_id)
                    .is_some());

                assert!(storage.contains_edge(edge_id));

                if storage.is_undirected() {
                    assert!(storage
                        .edges_from(dst_id)
                        .unwrap()
                        .iter()
                        .find(|(d_id, edge)| *d_id == src_id && edge.get_id() == edge_id)
                        .is_some());

                    assert!(storage
                        .edges()
                        .iter()
                        .find(|(s_id, d_id, edge)| *s_id == std::cmp::min(src_id, dst_id)
                            && *d_id == std::cmp::max(src_id, dst_id)
                            && edge.get_id() == edge_id)
                        .is_some());
                } else {
                    assert!(storage
                        .edges()
                        .iter()
                        .find(|(s_id, d_id, edge)| *s_id == src_id
                            && *d_id == dst_id
                            && edge.get_id() == edge_id)
                        .is_some());
                }
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

    #[test]
    fn update_edge() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            mut storage: S,
        ) -> bool {
            if storage.edge_count() > 0 {
                // Choose an edge randomly.
                let (src_id, dst_id, edge) =
                    *storage.edges().choose(&mut rand::thread_rng()).unwrap();

                let before_vertex_count = storage.vertex_count();
                let before_edge_count = storage.edge_count();
                let before_weight = edge.get_weight().unwrap();
                let edge_id = edge.get_id();

                let new_weight = if before_weight == i32::MAX {
                    before_weight - 1
                } else {
                    before_weight + 1
                };

                storage
                    .update_edge(src_id, dst_id, edge_id, new_weight.into())
                    .unwrap();

                assert!(storage.vertex_count() == before_vertex_count);
                assert!(storage.edge_count() == before_edge_count);
                assert!(
                    *storage
                        .edge_between(src_id, dst_id, edge_id)
                        .unwrap()
                        .get_weight()
                        == new_weight.into()
                )
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

    #[test]
    fn remove_edge() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            mut storage: S,
        ) -> bool {
            if storage.edge_count() > 0 {
                // Choose an edge randomly.
                let (src_id, dst_id, edge) =
                    *storage.edges().choose(&mut rand::thread_rng()).unwrap();

                let before_vertex_count = storage.vertex_count();
                let before_edge_count = storage.edge_count();
                let edge_id = edge.get_id();

                storage.remove_edge(src_id, dst_id, edge_id).unwrap();

                assert!(storage.vertex_count() == before_vertex_count);
                assert!(storage.edge_count() == before_edge_count - 1);
                assert!(!storage.contains_edge(edge_id));
                assert!(storage.edge(edge_id).is_err());
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

    #[test]
    fn edges_between() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            storage: S,
        ) -> bool {
            if storage.vertex_count() > 0 {
                // Choose a src and dst randomly.
                let src_id = *storage.vertices().choose(&mut rand::thread_rng()).unwrap();
                let dst_id = *storage.vertices().choose(&mut rand::thread_rng()).unwrap();

                let calculated_edges_between: Vec<&DefaultEdge<i32>> = storage
                    .edges()
                    .into_iter()
                    .filter_map(|(s_id, d_id, edge)| {
                        if (storage.is_directed() && s_id == src_id && d_id == dst_id)
                            || (storage.is_undirected()
                                && s_id == std::cmp::min(src_id, dst_id)
                                && d_id == std::cmp::max(src_id, dst_id))
                        {
                            Some(edge)
                        } else {
                            None
                        }
                    })
                    .collect();

                if let Ok(edges_between) = storage.edges_between(src_id, dst_id) {
                    assert!(edges_between.len() == calculated_edges_between.len());
                    assert!(edges_between
                        .iter()
                        .all(|edge| calculated_edges_between.contains(edge)));
                }
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

    #[test]
    fn neighbors() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            storage: S,
        ) -> bool {
            for vertex_id in storage.vertices() {
                let calculated_neighbors: Vec<usize> = storage
                    .edges_from(vertex_id)
                    .unwrap()
                    .into_iter()
                    .map(|(dst_id, _)| dst_id)
                    .collect();

                let neighbors = storage.neighbors(vertex_id).unwrap();

                assert!(neighbors.len() == calculated_neighbors.len());
                assert!(neighbors
                    .iter()
                    .all(|neighbor_id| calculated_neighbors.contains(neighbor_id)));
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

    #[test]
    fn as_directed_edges() {
        fn prop<Dir: EdgeDir, S: GraphStorage<i32, DefaultEdge<i32>, Dir> + Debug>(
            storage: S,
        ) -> bool {
            let edges = storage.edges();
            let directed_edges = storage.as_directed_edges();

            let must_be_duplicated_edges = edges
                .iter()
                .filter(|(src_id, dst_id, _)| src_id != dst_id)
                .count();
            let must_not_be_duplicated = edges.len() - must_be_duplicated_edges;

            if storage.is_undirected() {
                directed_edges.len() == must_not_be_duplicated + 2 * must_be_duplicated_edges
            } else {
                directed_edges.len() == edges.len()
            }
        }

        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMatrix<_, _, UndirectedEdge>) -> bool);
        
        quickcheck::quickcheck(prop as fn(AdjList<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjList<_, _, UndirectedEdge>) -> bool);
        
        quickcheck::quickcheck(prop as fn(AdjMap<_, _, DirectedEdge>) -> bool);
        quickcheck::quickcheck(prop as fn(AdjMap<_, _, UndirectedEdge>) -> bool);
    }
}
