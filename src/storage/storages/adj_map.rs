use crate::common::DynIter;
use crate::provide::{
    CheckedEdges, CheckedMutEdges, CheckedMutVertices, CheckedVertices, Edges, MutEdges,
    MutVertices, Vertices,
};
use crate::storage::edge::{Edge, EdgeDescriptor};
use crate::storage::token::UsizeTokenProvider;
use crate::storage::vertex::VertexDescriptor;
use itertools::Itertools;
use std::collections::HashMap;

// TODO: Benchmark
#[derive(Debug)]
pub struct AdjMap<V, E, const DIR: bool>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    vt_provider: UsizeTokenProvider,
    et_provider: UsizeTokenProvider,

    vt_to_v: HashMap<usize, V>,
    et_to_e: HashMap<usize, Edge<E>>,

    adj_map: HashMap<usize, HashMap<usize, Vec<usize>>>,
    pred: HashMap<usize, HashMap<usize, Vec<usize>>>,
}

impl<V, E, const DIR: bool> AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    pub fn init() -> Self {
        AdjMap {
            vt_provider: UsizeTokenProvider::init(),
            et_provider: UsizeTokenProvider::init(),

            vt_to_v: HashMap::new(),
            et_to_e: HashMap::new(),

            adj_map: HashMap::new(),
            pred: HashMap::new(),
        }
    }
}

impl<V, E, const DIR: bool> Vertices for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    type V = V;

    fn vertex(&self, vt: usize) -> &Self::V {
        &self.vt_to_v[&vt]
    }

    fn vertex_count(&self) -> usize {
        self.vt_to_v.keys().count()
    }

    fn vertex_tokens(&self) -> DynIter<'_, usize> {
        DynIter::init(self.vt_to_v.keys().copied())
    }

    fn vertices(&self) -> DynIter<'_, &Self::V> {
        DynIter::init(self.vt_to_v.values())
    }

    fn neighbors(&self, vt: usize) -> DynIter<'_, usize> {
        self.successors(vt)
    }

    fn has_vt(&self, vt: usize) -> bool {
        self.vt_to_v.contains_key(&vt)
    }

    fn successors(&self, vt: usize) -> DynIter<'_, usize> {
        if DIR {
            DynIter::init(self.adj_map[&vt].keys().copied())
        } else {
            DynIter::init(
                self.adj_map[&vt]
                    .keys()
                    .chain(self.pred[&vt].keys())
                    .unique()
                    .copied(),
            )
        }
    }

    fn predecessors(&self, vt: usize) -> DynIter<'_, usize> {
        if DIR {
            DynIter::init(self.pred[&vt].keys().copied())
        } else {
            DynIter::init(
                self.pred[&vt]
                    .keys()
                    .chain(self.adj_map[&vt].keys())
                    .unique()
                    .copied(),
            )
        }
    }
}

impl<V, E, const DIR: bool> CheckedVertices for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
}

impl<V, E, const DIR: bool> MutVertices for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    fn has_free_token(&mut self) -> bool {
        self.vt_provider.has_next()
    }

    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V {
        self.vt_to_v.get_mut(&vt).unwrap()
    }

    fn add_vertex(&mut self, vertex: Self::V) -> usize {
        let vt = self
            .vt_provider
            .get()
            .expect("There are no more vertex tokens available");

        self.vt_to_v.insert(vt, vertex);
        self.adj_map.insert(vt, HashMap::new());
        self.pred.insert(vt, HashMap::new());

        vt
    }

    fn remove_vertex(&mut self, vt: usize) -> Self::V {
        let neighbor_vts: Vec<usize> = self.adj_map[&vt].keys().copied().collect();
        for n_vt in neighbor_vts {
            let ets: Vec<usize> = self.adj_map[&vt][&n_vt].iter().copied().collect();

            for et in ets {
                self.remove_edge(vt, n_vt, et);
            }
        }

        let preds: Vec<usize> = self.pred[&vt].keys().copied().collect();
        for pred_vt in preds {
            let ets: Vec<usize> = self.adj_map[&vt][&pred_vt].iter().copied().collect();

            for et in ets {
                self.remove_edge(pred_vt, vt, et);
            }
        }

        self.vt_to_v.remove(&vt).unwrap()
    }
}

impl<V, E, const DIR: bool> CheckedMutVertices for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
}

impl<V, E, const DIR: bool> Edges for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    type E = E;

    fn edge(&self, et: usize) -> (usize, usize, &E) {
        self.et_to_e[&et].view()
    }

    fn edge_count(&self) -> usize {
        self.et_to_e.keys().count()
    }

    fn edge_tokens(&self) -> DynIter<'_, usize> {
        DynIter::init(self.et_to_e.keys().copied())
    }

    fn edges(&self) -> DynIter<'_, (usize, usize, &Self::E)> {
        DynIter::init(self.et_to_e.values().map(|e| e.view()))
    }

    fn ingoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        if DIR {
            DynIter::init(self.pred[&vt].values().flatten().copied())
        } else {
            DynIter::init(
                self.pred[&vt]
                    .values()
                    .flatten()
                    .chain(self.adj_map[&vt].values().flatten())
                    .unique()
                    .copied(),
            )
        }
    }

    fn outgoing_edges(&self, vt: usize) -> DynIter<'_, usize> {
        if DIR {
            DynIter::init(self.adj_map[&vt].values().flatten().copied())
        } else {
            DynIter::init(
                self.adj_map[&vt]
                    .values()
                    .flatten()
                    .chain(self.pred[&vt].values().flatten())
                    .unique()
                    .copied(),
            )
        }
    }

    fn has_et(&self, et: usize) -> bool {
        self.et_to_e.contains_key(&et)
    }
}

impl<V, E, const DIR: bool> CheckedEdges for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
}

impl<V, E, const DIR: bool> MutEdges for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
    fn has_free_et(&mut self) -> bool {
        self.et_provider.has_next()
    }

    fn edge_mut(&mut self, et: usize) -> (usize, usize, &mut E) {
        self.et_to_e.get_mut(&et).unwrap().view_mut()
    }

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize {
        let et = self.et_provider.get().unwrap();

        self.adj_map
            .get_mut(&src_vt)
            .unwrap()
            .entry(dst_vt)
            .or_insert_with(Vec::new)
            .push(et);

        self.pred
            .get_mut(&dst_vt)
            .unwrap()
            .entry(src_vt)
            .or_insert_with(Vec::new)
            .push(et);

        self.et_to_e.insert(et, Edge::init(src_vt, dst_vt, edge));

        et
    }

    fn remove_edge(&mut self, mut src_vt: usize, mut dst_vt: usize, et: usize) -> Self::E {
        if self.pred[&src_vt][&dst_vt].contains(&et) {
            std::mem::swap(&mut src_vt, &mut dst_vt)
        }

        let vec = self
            .adj_map
            .get_mut(&src_vt)
            .unwrap()
            .get_mut(&dst_vt)
            .unwrap();

        vec.retain(|_et| *_et != et);

        if vec.is_empty() {
            self.adj_map.get_mut(&src_vt).unwrap().remove(&dst_vt);
        }

        let vec = self
            .pred
            .get_mut(&dst_vt)
            .unwrap()
            .get_mut(&src_vt)
            .unwrap();

        vec.retain(|_et| *_et != et);

        if vec.is_empty() {
            self.pred.get_mut(&dst_vt).unwrap().remove(&src_vt);
        }

        self.et_provider.free(et);

        self.et_to_e.remove(&et).unwrap().into_inner()
    }
}

impl<V, E, const DIR: bool> CheckedMutEdges for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor,
{
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::AdjMap;
    use crate::provide::{Edges, MutEdges, MutVertices, Vertices};
    use crate::storage::{edge::EdgeDescriptor, vertex::VertexDescriptor};
    use quickcheck::Arbitrary;
    use rand::{thread_rng, Rng};

    impl<V, E, const DIR: bool> Clone for AdjMap<V, E, DIR>
    where
        V: VertexDescriptor + Arbitrary,
        E: EdgeDescriptor + Arbitrary,
    {
        fn clone(&self) -> Self {
            Self {
                vt_provider: self.vt_provider.clone(),
                et_provider: self.et_provider.clone(),
                vt_to_v: self.vt_to_v.clone(),
                et_to_e: self.et_to_e.clone(),
                adj_map: self.adj_map.clone(),
                pred: self.pred.clone(),
            }
        }
    }

    impl<V, E, const DIR: bool> Arbitrary for AdjMap<V, E, DIR>
    where
        V: VertexDescriptor + Arbitrary,
        E: EdgeDescriptor + Arbitrary,
    {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 100;

            let mut rng = thread_rng();
            let edge_probability = rng.gen::<f64>() * rng.gen::<f64>();

            let mut adj_map = AdjMap::<V, E, DIR>::init();

            let vts: Vec<usize> = (0..vertex_count)
                .map(|_| adj_map.add_vertex(V::arbitrary(g)))
                .collect();

            vts.iter().zip(vts.iter()).for_each(|(i, j)| {
                let p = rng.gen::<f64>();

                if p <= edge_probability {
                    let num_of_edges = (usize::arbitrary(g) % 3) + 1;

                    for _ in 0..num_of_edges {
                        adj_map.add_edge(*i, *j, E::arbitrary(g));
                    }
                }
            });

            adj_map
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let mut adj_map_even = Self::init();
            let mut adj_map_odd = Self::init();

            let mut vt_map = HashMap::new();

            for (index, vt) in self.vertex_tokens().enumerate() {
                let new_vt = if index % 2 == 0 {
                    adj_map_even.add_vertex(self.vertex(vt).clone())
                } else {
                    adj_map_odd.add_vertex(self.vertex(vt).clone())
                };

                vt_map.insert(vt, new_vt);
            }

            for (src_vt, dst_vt, edge) in self.edges() {
                let new_src_vt = vt_map[&src_vt];
                let new_dst_vt = vt_map[&dst_vt];

                if adj_map_even.has_vt(new_src_vt) && adj_map_even.has_vt(new_dst_vt) {
                    adj_map_even.add_edge(new_src_vt, new_dst_vt, edge.clone());
                } else if adj_map_odd.has_vt(new_src_vt) && adj_map_odd.has_vt(new_dst_vt) {
                    adj_map_odd.add_edge(new_src_vt, new_dst_vt, edge.clone());
                }
            }

            let before_count = self.vertex_count();

            Box::new(
                [adj_map_even, adj_map_odd]
                    .into_iter()
                    .filter(move |adj_map| adj_map.vertex_count() < before_count),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::provide::storage_test_suit;
    use crate::storage::AdjMap;

    #[quickcheck]
    fn prop_storage(storage: AdjMap<usize, usize, false>, dir_storage: AdjMap<usize, usize, true>) {
        storage_test_suit::prop_storage(storage);
        storage_test_suit::prop_storage(dir_storage);
    }

    #[quickcheck]
    fn prop_vertex_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_vertex_checked(storage);
        storage_test_suit::prop_vertex_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_vertex_count_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_vertex_count_checked(storage);
        storage_test_suit::prop_vertex_count_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_vertex_tokens_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_vertex_tokens_checked(storage);
        storage_test_suit::prop_vertex_tokens_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_vertices_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_vertices_checked(storage);
        storage_test_suit::prop_vertices_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_neighbors_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_neighbors_checked(storage);
        storage_test_suit::prop_neighbors_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_successors_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_successors_checked(storage);
        storage_test_suit::prop_successors_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_predecessors_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_predecessors_checked(storage);
        storage_test_suit::prop_predecessors_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_add_vertex(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_add_vertex(storage);
        storage_test_suit::prop_add_vertex(dir_storage);
    }

    #[quickcheck]
    fn prop_remove_vertex(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_remove_vertex(storage);
        storage_test_suit::prop_remove_vertex(dir_storage);
    }

    #[quickcheck]
    fn prop_update_vertex(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_update_vertex(storage);
        storage_test_suit::prop_update_vertex(dir_storage);
    }

    #[quickcheck]
    fn prop_remove_vertex_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_remove_vertex_checked(storage);
        storage_test_suit::prop_remove_vertex_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_vertex_mut_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_vertex_mut_checked(storage);
        storage_test_suit::prop_vertex_mut_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_edge_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_edge_checked(storage);
        storage_test_suit::prop_edge_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_edge_count_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_edge_count_checked(storage);
        storage_test_suit::prop_edge_count_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_edge_tokens_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_edge_tokens_checked(storage);
        storage_test_suit::prop_edge_tokens_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_edges_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_edges_checked(storage);
        storage_test_suit::prop_edges_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_ingoing_edges_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_ingoing_edges_checked(storage);
        storage_test_suit::prop_ingoing_edges_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_outgoing_edges_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_outgoing_edges_checked(storage);
        storage_test_suit::prop_outgoing_edges_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_add_edge(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_add_edge(storage);
        storage_test_suit::prop_add_edge(dir_storage);
    }

    #[quickcheck]
    fn prop_remove_edge(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_remove_edge(storage);
        storage_test_suit::prop_remove_edge(dir_storage);
    }

    #[quickcheck]
    fn prop_update_edge(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_update_edge(storage);
        storage_test_suit::prop_update_edge(dir_storage);
    }

    #[quickcheck]
    fn prop_remove_edge_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_remove_edge_checked(storage);
        storage_test_suit::prop_remove_edge_checked(dir_storage);
    }

    #[quickcheck]
    fn prop_edge_mut_checked(
        storage: AdjMap<usize, usize, false>,
        dir_storage: AdjMap<usize, usize, true>,
    ) {
        storage_test_suit::prop_edge_mut_checked(storage);
        storage_test_suit::prop_edge_mut_checked(dir_storage);
    }
}
