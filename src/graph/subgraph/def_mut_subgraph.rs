use std::{collections::HashSet, marker::PhantomData};

use crate::{
    graph::{error::Error, EdgeDir},
    prelude::{Edge, Edges, Graph, Neighbors, Vertices},
};
use anyhow::Result;

use super::{AsFrozenSubgraph, AsMutSubgraph, AsSubgraph};

/// Default implementation of [`AsMutSubgraph`](crate::graph::subgraph::AsMutSubgraph) trait.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that subgraph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
/// * `G`: **G**raph type that subgraph is representing.
pub struct MutSubgraph<'a, W, E: Edge<W>, Dir: EdgeDir, G: Graph<W, E, Dir>> {
    graph: &'a mut G,

    edges: Vec<(usize, usize, usize)>,
    vertex_ids: HashSet<usize>,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<'a, W, E, Dir, G> MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    /// Initializes a subgraph with provided `edges` and `vertex_ids`.
    ///
    /// # Arguments
    /// * `graph`: Graph that this subgraph is representing.
    /// * `edges`: Edges present in the subgraph in the format of (`src_id`, `dst_id`, `edge_id`).
    /// * `vertex_ids`: Vertices present in the subgraph.
    ///
    /// # Returns
    /// An initialized subgraph containing the provided edges and vertices.
    pub fn init(
        graph: &'a mut G,
        edges: Vec<(usize, usize, usize)>,
        vertex_ids: HashSet<usize>,
    ) -> Self {
        MutSubgraph {
            graph,
            edges,
            vertex_ids,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<'a, W, E, Dir, G> Neighbors for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `src_id` is not present in the subgraph.
    /// * `Ok`: Containing Id of vertices accessible from source vertex using one edge.
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self
                .edges
                .iter()
                .filter_map(|(s_id, dst_id, _)| if *s_id == src_id { Some(*dst_id) } else { None })
                .collect())
        }
    }
}

impl<'a, W, E, Dir, G> Vertices for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir>,
{
    /// # Returns
    /// Id of vertices that are present in the graph.
    fn vertices(&self) -> Vec<usize> {
        self.vertex_ids.iter().copied().collect()
    }

    /// # Returns
    /// * `true`: If subgraph contains the vertex with id: `vertex_id`.
    /// * `false`: Otherwise
    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.vertex_ids.contains(&vertex_id)
    }
}

impl<'a, W, E, Dir, G> Edges<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `src_id` does not exist.
    /// * `Ok`: Containin all edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self
                .graph
                .edges_from(src_id)?
                .into_iter()
                .filter(|(dst_id, edge)| {
                    self.contains_vertex(*dst_id) && self.contains_edge(edge.get_id())
                })
                .collect())
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Err`: If either `src_id` or `dst_id` is invalid.
    /// * `Ok`: Containing edges from source vertex to destination vertex.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self
                .graph
                .edges_between(src_id, dst_id)?
                .into_iter()
                .filter(|edge| self.contains_edge(edge.get_id()))
                .collect())
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// * `Err`: If either vertices with `src_id` or `dst_id` does not exist.
    /// Also when there is not edge from source to destination with id: `edge_id`.
    /// * `Ok`: Containing reference to edge with id: `edge_id` from `src_id` to `dst_id`.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            self.graph.edge_between(src_id, dst_id, edge_id)
        }
    }

    /// # Note:
    /// Consider using `edge_between` or `edges_from` functions instead of this one.
    /// Because default implementation of this function iterates over all edges to find the edge with specified id.
    /// And it's likely that other storages use the same approach. So:
    /// * if you have info about source of the edge, consider using `edges_from` function instead.
    /// * if you have info about both source and destination of the edge, consider using `edge_between` function instead.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge to be retrieved.
    ///
    /// # Returns
    /// * `Err`: If there is not edge with id: `edge_id`.
    /// * `Ok`: Containing reference to edge with id: `edge_id`.
    fn edge(&self, edge_id: usize) -> Result<&E> {
        if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            self.graph.edge(edge_id)
        }
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Err`: If either `src_id` or `dst_id` is invalid.
    /// * `Ok`: Containing `true` if there is at least one edge from `src_id` to `dst_id` and `false` otherwise.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self
                .edges
                .iter()
                .find(|(s_id, d_id, _)| *s_id == src_id && *d_id == dst_id)
                .is_some())
        }
    }

    /// # Returns
    /// All edges in the graph in the format: (`src_id`, `dst_id`, `edge`).
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

    /// Difference between this function and `edges` is that this function treats each edge as a directed edge. \
    /// For example consider graph: a --- b \
    /// If you call `edges` on this graph, you will get: (a, b, edge). \
    /// But if you call `as_directed_edges`, you will get two elements: (a, b, edge) and (b, a, edge). \
    /// It's specifically useful in algorithms that are for directed graphs but can also be applied to undirected graphs if we treat the edges as directed.
    /// One example is [`BellmanFord`](crate::algo::BellmanFord) algorithm.
    ///
    /// # Returns
    /// All edges(as directed edges) in the graph in the format of: (`src_id`, `dst_id`, `edge`).
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

    /// # Returns
    /// Number of edges in the graph.
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

impl<'a, W, E, Dir, G> AsFrozenSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Edges<W, E> + Neighbors,
{
}

impl<'a, W, E, Dir, G> AsSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    /// Removes an edge from the subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source to destination to be removed.
    ///
    /// # Returns
    /// * `Err`: If either vertices with `src_id` or `dst_id` does not exist.
    /// Also when there is not edge from source to destination with id: `edge_id`.
    /// * `Ok`:
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            if let Some(index) = self.edges.iter().position(|(s_id, d_id, e_id)| {
                *s_id == src_id && *d_id == dst_id && *e_id == edge_id
            }) {
                self.edges.swap_remove(index);

                Ok(())
            } else {
                Err(Error::new_iei(src_id, dst_id, edge_id))?
            }
        }
    }

    /// Removes a vertex from the subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Returns
    /// * `Err`: If vertex with id: `vertex_id` is not present in the subgraph.
    /// * `Ok`:
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            self.vertex_ids.remove(&vertex_id);

            self.edges
                .retain(|(src_id, dst_id, _)| *src_id != vertex_id || *dst_id != vertex_id);

            Ok(())
        }
    }

    /// Adds a vertex from the graph to subgraph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be added.
    ///
    /// # Returns
    /// * `Err`: If graph does not contain vertex with id: `vertex_id`.
    /// * `Ok`:
    fn add_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()> {
        if !self.graph.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else if self.contains_vertex(vertex_id) {
            Err(Error::new_vae(vertex_id))?
        } else {
            self.vertex_ids.insert(vertex_id);

            Ok(())
        }
    }

    /// Adds an edge from the graph to subgraph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge to be added.
    ///
    /// # Returns
    /// * `Err`:
    ///     * If vertex with id: `src_id` does not exist in graph.
    ///     * If vertex with id: `dst_id` dost not exist in graph.
    ///     * If edge with id: `edge_id` does not exist in graph(from src to dst).
    ///     * If edge already exists in the subgraph.
    /// * `Ok`:
    fn add_edge_from_graph(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<()> {
        if self.contains_edge(edge_id) {
            Err(Error::new_eae(edge_id))?
        } else {
            if let Some(_) = self
                .graph
                .edges_between(src_id, dst_id)?
                .into_iter()
                .find(|edge| edge.get_id() == edge_id)
            {
                self.edges.push((src_id, dst_id, edge_id));

                self.vertex_ids.insert(src_id);
                self.vertex_ids.insert(dst_id);

                Ok(())
            } else {
                Err(Error::new_iei(src_id, dst_id, edge_id))?
            }
        }
    }
}

impl<'a, W, E, Dir, G> AsMutSubgraph<W, E> for MutSubgraph<'a, W, E, Dir, G>
where
    E: Edge<W>,
    Dir: EdgeDir,
    G: Graph<W, E, Dir> + Vertices + Neighbors + Edges<W, E>,
{
    /// Removes a vertex from the graph and consequently from the subgraph as well if it contains the vertex.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed
    ///
    /// # Returns
    /// Result of calling `remove_vertex` on the graph(so it depends on the graph/storage).
    fn remove_vertex_from_graph(&mut self, vertex_id: usize) -> Result<()> {
        if self.contains_vertex(vertex_id) {
            self.remove_vertex(vertex_id)?;
        }

        self.graph.remove_vertex(vertex_id)
    }

    /// Removes an edge from the graph and consequently from the subgraph as well if it contains the edge.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge_id`: Id of the edge from source to destination.
    ///
    /// # Returns
    /// Result of calling `remove_edge` on the graph(so it depends on the graph/storage).
    fn remove_edge_from_graph(
        &mut self,
        src_id: usize,
        dst_id: usize,
        edge_id: usize,
    ) -> Result<E> {
        if self.contains_edge(edge_id) {
            self.remove_edge(src_id, dst_id, edge_id)?;
        }

        self.graph.remove_edge(src_id, dst_id, edge_id)
    }

    /// Adds a vertex to the subgraph and consequently to the graph.
    ///
    /// # Returns
    /// Id of the newly added vertex.
    fn add_vertex(&mut self) -> usize {
        let vertex_id = self.graph.add_vertex();

        self.vertex_ids.insert(vertex_id);

        vertex_id
    }

    /// Adds an edge to the subgraph and consequently to the graph.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be add from source to destination.
    ///
    /// # Returns
    /// * `Err`: Error of calling `add_edge` on the graph(so it depends on the graph/storage).
    /// * `Ok`: Containing id of the newly created edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize> {
        let edge_id = self.graph.add_edge(src_id, dst_id, edge)?;

        self.add_edge_from_graph(src_id, dst_id, edge_id)?;

        Ok(edge_id)
    }
}
