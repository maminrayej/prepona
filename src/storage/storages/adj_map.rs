use crate::common::DynIter;
use crate::provide::{Edges, MutEdges, MutVertices, Vertices};
use crate::storage::edge::EdgeDescriptor;
use crate::storage::token::UsizeTokenProvider;
use crate::storage::vertex::VertexDescriptor;
use itertools::Itertools;
use std::collections::HashMap;

// TODO: Test
// TODO: Benchmark

pub struct AdjMap<V, E, const DIR: bool>
where
    V: VertexDescriptor,
    E: EdgeDescriptor<DIR>,
{
    vt_provider: UsizeTokenProvider,
    et_provider: UsizeTokenProvider,

    vt_to_v: HashMap<usize, V>,
    et_to_e: HashMap<usize, E>,

    adj_map: HashMap<usize, HashMap<usize, Vec<usize>>>,
    pred: HashMap<usize, HashMap<usize, Vec<usize>>>,
}

impl<V, E, const DIR: bool> AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor<DIR>,
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
    E: EdgeDescriptor<DIR>,
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

impl<V, E, const DIR: bool> MutVertices for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor<DIR>,
{
    type V = V;

    fn vertex_mut(&mut self, vt: usize) -> &mut Self::V {
        self.vt_to_v.get_mut(&vt).unwrap()
    }

    fn add_vertex(&mut self, vertex: Self::V) -> usize {
        let vt = self
            .vt_provider
            .next()
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

impl<V, E, const DIR: bool> Edges<DIR> for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor<DIR>,
{
    type E = E;

    fn edge(&self, et: usize) -> &Self::E {
        &self.et_to_e[&et]
    }

    fn edge_count(&self) -> usize {
        self.et_to_e.keys().count()
    }

    fn edge_tokens(&self) -> DynIter<'_, usize> {
        DynIter::init(self.et_to_e.keys().copied())
    }

    fn edges(&self) -> DynIter<'_, &Self::E> {
        DynIter::init(self.et_to_e.values())
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
                    .copied(),
            )
        }
    }

    fn has_edge(&self, et: usize) -> bool {
        self.et_to_e.contains_key(&et)
    }
}

impl<V, E, const DIR: bool> MutEdges<DIR> for AdjMap<V, E, DIR>
where
    V: VertexDescriptor,
    E: EdgeDescriptor<DIR>,
{
    type E = E;

    fn edge_mut(&mut self, et: usize) -> &mut Self::E {
        self.et_to_e.get_mut(&et).unwrap()
    }

    fn add_edge(&mut self, src_vt: usize, dst_vt: usize, edge: Self::E) -> usize {
        let et = self.et_provider.next().unwrap();

        self.adj_map
            .get_mut(&src_vt)
            .unwrap()
            .entry(dst_vt)
            .or_insert(Vec::new())
            .push(et);

        self.pred
            .get_mut(&dst_vt)
            .unwrap()
            .entry(src_vt)
            .or_insert(Vec::new())
            .push(et);

        self.et_to_e.insert(et, edge);

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

        self.et_to_e.remove(&et).unwrap()
    }
}
