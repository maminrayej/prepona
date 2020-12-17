use std::marker::PhantomData;

use crate::{
    graph::EdgeDir,
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};

use super::AsSubgraph;

pub struct Subgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    graph: &'a G,

    edge_ids: Vec<usize>,
    vertex_ids: Vec<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E, Dir, G> Neighbors for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Neighbors,
{
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.graph
            .neighbors(src_id)
            .into_iter()
            .filter(|vertex_id| self.vertex_ids.contains(vertex_id))
            .collect()
    }
}

impl<'a, W, E, Dir, G> Vertices for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    fn vertices(&self) -> Vec<usize> {
        self.vertex_ids.clone()
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E>,
{
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.graph
            .edges_from(src_id)
            .into_iter()
            .filter(|(dst_id, edge)| {
                self.vertex_ids.contains(dst_id) && self.edge_ids.contains(&edge.get_id())
            })
            .collect()
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.graph
            .edges_from(src_id)
            .into_iter()
            .filter_map(|(d_id, edge)| if d_id == dst_id { Some(edge) } else { None })
            .collect()
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges().into_iter().find_map(|(_, _, edge)| {
            if edge.get_id() == edge_id {
                Some(edge)
            } else {
                None
            }
        })
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.as_directed_edges()
        } else {
            self.as_directed_edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.graph
            .as_directed_edges()
            .into_iter()
            .filter(|(_, _, edge)| self.edge_ids.contains(&edge.get_id()))
            .collect()
    }

    fn edges_count(&self) -> usize {
        self.edge_ids.len()
    }
}

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Neighbors + Edges<W, E>,
{
}
