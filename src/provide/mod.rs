mod edge;
mod vertex;

pub use edge::*;
pub use vertex::*;

use crate::storage::edge::Direction;

pub trait Storage {
    type Dir: Direction;
}

pub trait InitializableStorage: Storage {
    fn init() -> Self;
}

#[cfg(test)]
pub(crate) mod storage_test_suit {
    use rand::distributions::Standard;
    use rand::prelude::{Distribution, IteratorRandom};
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::hash::Hash;

    use crate::test_utils::get_non_duplicate;

    use super::{Edges, MutEdges, MutVertices, Vertices};

    fn validate_storage<S>(storage: S)
    where
        S: Edges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
    {
        let vertex_tokens_set: HashSet<usize> = storage.vertex_tokens().collect();

        assert_eq!(vertex_tokens_set.len(), storage.vertex_count());

        for vt in vertex_tokens_set.iter().copied() {
            assert!(storage.has_vt(vt));
        }

        let vertex_set: HashSet<S::V> = vertex_tokens_set
            .into_iter()
            .map(|vt| storage.vertex(vt).clone())
            .collect();
        assert_eq!(
            vertex_set,
            storage.vertices().cloned().collect::<HashSet<S::V>>()
        );

        for (src_vt, dst_vt, edge) in storage.edges() {
            assert!(storage
                .neighbors(src_vt)
                .find(|n_vt| *n_vt == dst_vt)
                .is_some());

            assert!(storage
                .successors(src_vt)
                .find(|s_vt| *s_vt == dst_vt)
                .is_some());

            assert!(storage
                .predecessors(dst_vt)
                .find(|p_id| *p_id == src_vt)
                .is_some());

            assert!(storage
                .outgoing_edges(src_vt)
                .map(|et| storage.edge(et))
                .find(|(_, _, e)| *e == edge)
                .is_some());

            assert!(storage
                .ingoing_edges(dst_vt)
                .map(|et| storage.edge(et))
                .find(|(_, _, e)| *e == edge)
                .is_some());
        }

        assert_eq!(storage.edge_tokens().count(), storage.edge_count());
        for et in storage.edge_tokens() {
            assert!(storage.has_et(et))
        }

        assert_eq!(
            storage
                .edge_tokens()
                .map(|et| storage.edge(et))
                .collect::<HashSet<(usize, usize, &S::E)>>(),
            storage.edges().collect()
        );
    }

    pub fn prop_storage<S>(storage: S)
    where
        S: Edges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
    {
        validate_storage(storage)
    }

    pub fn prop_vertex_checked<S>(storage: S)
    where
        S: Vertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let valid_vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.vertex_checked(non_existent_vt).is_err());
            assert!(storage.vertex_checked(valid_vt).is_ok());
        }
    }

    pub fn prop_vertex_count_checked<S>(storage: S)
    where
        S: Vertices,
    {
        assert!(storage.vertex_count_checked().is_ok())
    }

    pub fn prop_vertex_tokens_checked<S>(storage: S)
    where
        S: Vertices,
    {
        assert!(storage.vertex_tokens_checked().is_ok())
    }

    pub fn prop_vertices_checked<S>(storage: S)
    where
        S: Vertices,
    {
        assert!(storage.vertices_checked().is_ok())
    }

    pub fn prop_neighbors_checked<S>(storage: S)
    where
        S: Vertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.neighbors_checked(non_existent_vt).is_err());
            assert!(storage.neighbors_checked(vt).is_ok())
        }
    }

    pub fn prop_successors_checked<S>(storage: S)
    where
        S: Vertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.successors_checked(non_existent_vt).is_err());
            assert!(storage.successors_checked(vt).is_ok())
        }
    }

    pub fn prop_predecessors_checked<S>(storage: S)
    where
        S: Vertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.predecessors_checked(non_existent_vt).is_err());
            assert!(storage.predecessors_checked(vt).is_ok())
        }
    }

    pub fn prop_add_vertex<S>(mut storage: S)
    where
        S: MutVertices + Edges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
        Standard: Distribution<S::V>,
    {
        let before_vertex_count = storage.vertex_count();
        let vertex = thread_rng().gen();

        let vt = storage.add_vertex(vertex.clone());

        assert_eq!(storage.vertex_count(), before_vertex_count + 1);
        assert!(storage.has_vt(vt));
        assert_eq!(storage.vertex(vt).clone(), vertex);
        validate_storage(storage);
    }

    pub fn prop_remove_vertex<S>(mut storage: S)
    where
        S: Edges + MutVertices,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
    {
        let before_vertex_count = storage.vertex_count();
        if before_vertex_count > 0 {
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();
            let vertex = storage.vertex(vt).clone();

            let removed_vertex = storage.remove_vertex(vt);

            assert_eq!(storage.vertex_count(), before_vertex_count - 1);
            assert_eq!(vertex, removed_vertex);
            assert!(!storage.has_vt(vt));
            validate_storage(storage);
        }
    }

    pub fn prop_update_vertex<S>(mut storage: S)
    where
        S: Edges + MutVertices,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
        Standard: Distribution<S::V>,
    {
        let before_vertex_count = storage.vertex_count();
        let mut rng = thread_rng();

        if before_vertex_count > 0 {
            let vt = storage.vertex_tokens().choose(&mut rng).unwrap();
            let mut new_vertex = rng.gen();
            let new_vertex_copy = new_vertex.clone();

            std::mem::swap(storage.vertex_mut(vt), &mut new_vertex);

            assert_eq!(storage.vertex_count(), before_vertex_count);
            assert_eq!(storage.vertex(vt).clone(), new_vertex_copy);
            assert!(storage.has_vt(vt));
            validate_storage(storage);
        }
    }

    pub fn prop_remove_vertex_checked<S>(mut storage: S)
    where
        S: MutVertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.remove_vertex_checked(non_existent_vt).is_err());
            assert!(storage.remove_vertex_checked(vt).is_ok());
        }
    }

    pub fn prop_vertex_mut_checked<S>(mut storage: S)
    where
        S: MutVertices,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.vertex_mut_checked(non_existent_vt).is_err());
            assert!(storage.vertex_mut_checked(vt).is_ok());
        }
    }

    pub fn prop_edge_checked<S>(storage: S)
    where
        S: Edges,
    {
        if storage.edge_count() > 0 {
            let non_existent_et = get_non_duplicate(storage.edge_tokens(), 1)[0];
            let et = storage.edge_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.edge_checked(non_existent_et).is_err());
            assert!(storage.edge_checked(et).is_ok());
        }
    }

    pub fn prop_edge_count_checked<S>(storage: S)
    where
        S: Edges,
    {
        assert!(storage.edge_count_checked().is_ok())
    }

    pub fn prop_edge_tokens_checked<S>(storage: S)
    where
        S: Edges,
    {
        assert!(storage.edge_tokens_checked().is_ok())
    }

    pub fn prop_edges_checked<S>(storage: S)
    where
        S: Edges,
    {
        assert!(storage.edges_checked().is_ok())
    }

    pub fn prop_ingoing_edges_checked<S>(storage: S)
    where
        S: Edges,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.ingoing_edges_checked(non_existent_vt).is_err());
            assert!(storage.ingoing_edges_checked(vt).is_ok());
        }
    }

    pub fn prop_outgoing_edges_checked<S>(storage: S)
    where
        S: Edges,
    {
        if storage.vertex_count() > 0 {
            let non_existent_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let vt = storage.vertex_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.outgoing_edges_checked(non_existent_vt).is_err());
            assert!(storage.outgoing_edges_checked(vt).is_ok());
        }
    }

    pub fn prop_add_edge<S>(mut storage: S)
    where
        S: MutEdges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
        Standard: Distribution<S::E>,
    {
        if storage.vertex_count() > 0 {
            let mut rng = thread_rng();
            let src_vt = storage.vertex_tokens().choose(&mut rng).unwrap();
            let dst_vt = storage.vertex_tokens().choose(&mut rng).unwrap();

            let before_total_edge = storage.edge_count();
            let before_outgoing_edges = storage.outgoing_edges(src_vt).count();
            let before_ingoing_edges = storage.ingoing_edges(dst_vt).count();

            let edge: S::E = rng.gen();
            let et = storage.add_edge(src_vt, dst_vt, edge.clone());

            assert_eq!(storage.edge_count(), before_total_edge + 1);
            assert_eq!(
                storage.outgoing_edges(src_vt).count(),
                before_outgoing_edges + 1
            );
            assert_eq!(
                storage.ingoing_edges(dst_vt).count(),
                before_ingoing_edges + 1
            );
            assert_eq!(storage.edge(et).2.clone(), edge);
            assert!(storage.has_et(et));
            validate_storage(storage);
        }
    }

    pub fn prop_remove_edge<S>(mut storage: S)
    where
        S: MutEdges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
        Standard: Distribution<S::E>,
    {
        if storage.edge_count() > 0 {
            let et = storage.edge_tokens().choose(&mut thread_rng()).unwrap();
            let (src_vt, dst_vt, _edge) = storage.edge(et);
            let edge = _edge.clone();

            let before_total_edge = storage.edge_count();
            let before_outgoing_edges = storage.outgoing_edges(src_vt).count();
            let before_ingoing_edges = storage.ingoing_edges(dst_vt).count();

            let removed_edge = storage.remove_edge(src_vt, dst_vt, et);

            assert_eq!(storage.edge_count(), before_total_edge - 1);
            assert_eq!(
                storage.outgoing_edges(src_vt).count(),
                before_outgoing_edges - 1
            );
            assert_eq!(
                storage.ingoing_edges(dst_vt).count(),
                before_ingoing_edges - 1
            );
            assert_eq!(edge, removed_edge);
            assert!(!storage.has_et(et));
            validate_storage(storage);
        }
    }

    pub fn prop_update_edge<S>(mut storage: S)
    where
        S: MutEdges,
        S::V: Debug + Hash + Clone,
        S::E: Debug + Hash + Clone,
        Standard: Distribution<S::E>,
    {
        if storage.edge_count() > 0 {
            let mut rng = thread_rng();
            let et = storage.edge_tokens().choose(&mut rng).unwrap();

            let (src_vt, dst_vt, _) = storage.edge(et);
            let mut new_edge: S::E = rng.gen();
            let new_edge_copy = new_edge.clone();

            let before_total_edge = storage.edge_count();
            let before_outgoing_edges = storage.outgoing_edges(src_vt).count();
            let before_ingoing_edges = storage.ingoing_edges(dst_vt).count();

            std::mem::swap(&mut new_edge, storage.edge_mut(et).2);

            assert_eq!(storage.edge_count(), before_total_edge);
            assert_eq!(
                storage.outgoing_edges(src_vt).count(),
                before_outgoing_edges
            );
            assert_eq!(storage.ingoing_edges(dst_vt).count(), before_ingoing_edges);
            assert_eq!(*storage.edge(et).2, new_edge_copy);
            assert!(storage.has_et(et));
            validate_storage(storage);
        }
    }

    pub fn prop_remove_edge_checked<S>(mut storage: S)
    where
        S: MutEdges,
    {
        if storage.edge_count() > 0 {
            let non_existent_src_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let non_existent_dst_vt = get_non_duplicate(storage.vertex_tokens(), 1)[0];
            let non_existent_et = get_non_duplicate(storage.edge_tokens(), 1)[0];

            let et = storage.edge_tokens().choose(&mut thread_rng()).unwrap();
            let (src_vt, dst_vt, _) = storage.edge(et);

            assert!(storage
                .remove_edge_checked(non_existent_src_vt, dst_vt, et)
                .is_err());
            assert!(storage
                .remove_edge_checked(src_vt, non_existent_dst_vt, et)
                .is_err());
            assert!(storage
                .remove_edge_checked(src_vt, dst_vt, non_existent_et)
                .is_err());
            assert!(storage.remove_edge_checked(src_vt, dst_vt, et).is_ok());
        }
    }

    pub fn prop_edge_mut_checked<S>(mut storage: S)
    where
        S: MutEdges,
    {
        if storage.edge_count() > 0 {
            let non_existent_et = get_non_duplicate(storage.edge_tokens(), 1)[0];
            let et = storage.edge_tokens().choose(&mut thread_rng()).unwrap();

            assert!(storage.edge_mut_checked(non_existent_et).is_err());
            assert!(storage.edge_mut_checked(et).is_ok());
        }
    }
}
