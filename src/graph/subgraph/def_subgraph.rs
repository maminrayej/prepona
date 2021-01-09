use std::{collections::HashSet, marker::PhantomData};

use anyhow::{Context, Result};

use crate::{
    graph::{error::Error, EdgeDir},
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};

use super::{AsFrozenSubgraph, AsSubgraph};

pub struct Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    graph: &'a G,

    edges: Vec<(usize, usize, usize)>,
    vertex_ids: HashSet<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E, Dir, G> Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    pub fn init(
        graph: &'a G,
        edges: Vec<(usize, usize, usize)>,
        vertex_ids: HashSet<usize>,
    ) -> Self {
        Subgraph {
            graph,
            edges,
            vertex_ids,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, W, E, Dir, G> Neighbors for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.neighbors_unchecked(src_id))
        }
    }

    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|(s_id, dst_id, _)| if *s_id == src_id { Some(*dst_id) } else { None })
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
        self.vertex_ids.iter().copied().collect()
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.vertex_ids.contains(&vertex_id)
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Argument invalid")?
        } else {
            Ok(self.edges_from_unchecked(src_id))
        }
    }

    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.graph
            .edges_from_unchecked(src_id)
            .into_iter()
            .filter(|(dst_id, edge)| {
                self.contains_vertex(*dst_id) && self.contains_edge(edge.get_id())
            })
            .collect()
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.edges_between_unchecked(src_id, dst_id))
        }
    }

    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.graph
            .edges_between_unchecked(src_id, dst_id)
            .into_iter()
            .filter(|edge| self.contains_edge(edge.get_id()))
            .collect()
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<Option<&E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.edge_between_unchecked(src_id, dst_id, edge_id))
        }
    }

    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.graph.edge_between_unchecked(src_id, dst_id, edge_id)
    }

    fn edge(&self, edge_id: usize) -> Result<Option<&E>> {
        if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.edge_unchecked(edge_id))
        }
    }

    fn edge_unchecked(&self, edge_id: usize) -> Option<&E> {
        self.graph.edge_unchecked(edge_id)
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.has_any_edge_unchecked(src_id, dst_id))
        }
    }

    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(s_id, d_id, _)| *s_id == src_id && *d_id == dst_id)
            .is_some()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.graph
            .edges()
            .into_iter()
            .filter(|(src_id, dst_id, edge)| {
                self.contains_vertex(*src_id)
                    && self.contains_vertex(*dst_id)
                    && self.contains_edge(edge.get_id())
            })
            .collect()
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.edges()
        } else {
            self.edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    fn edges_count(&self) -> usize {
        self.edges().len()
    }

    fn contains_edge(&self, edge_id: usize) -> bool {
        self.edges
            .iter()
            .find(|(_, _, e_id)| *e_id == edge_id)
            .is_some()
    }
}

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
}

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for Subgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id)).with_context(|| "Subgraph failed")?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.remove_edge_unchecked(src_id, dst_id, edge_id))
        }
    }

    fn remove_edge_unchecked(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, e_id)| *e_id != edge_id)
    }

    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.remove_vertex_unchecked(vertex_id))
        }
    }

    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        self.vertex_ids.retain(|v_id| *v_id != vertex_id);

        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);
    }

    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()> {
        if !self.graph.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.add_vertex_from_graph_unchecked(vertex_id))
        }
    }

    fn add_vertex_from_graph_unchecked(&mut self, vertex_id: usize) {
        self.vertex_ids.insert(vertex_id);
    }

    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if !self.graph.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id)).with_context(|| "Subgraph failed")?
        } else if !self.graph.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id)).with_context(|| "Subgraph failed")?
        } else if !self.graph.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id)).with_context(|| "Subgraph failed")?
        } else if self.contains_edge(edge_id) {
            Err(Error::new_eae(edge_id)).with_context(|| "Subgraph failed")?
        } else {
            Ok(self.add_edge_from_graph_unchecked(src_id, dst_id, edge_id))
        }
    }

    fn add_edge_from_graph_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) {
        self.edges.push((src_id, dst_id, edge_id));

        self.vertex_ids.insert(src_id);
        self.vertex_ids.insert(dst_id);
    }
}
