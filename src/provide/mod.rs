mod id_map;

use anyhow::Result;
pub use id_map::IdMap;

use crate::graph::{Edge, EdgeDir};

/// Provides access to neighbors of an arbitrary vertex.
pub trait Neighbors {
    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// Id of vertices accessible from source vertex using one edge.
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>>;

    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize>;
}

/// Provides access to vertices of the graph.
pub trait Vertices {
    /// # Returns
    /// Id of vertices that are present in the graph.
    fn vertices(&self) -> Vec<usize>;

    /// # Returns
    /// Number of vertices in the graph.
    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    /// In many algorithms assuming graph vertices have a continuos series of ids makes implementing the algorithms easier.
    /// So this function maps potentially scattered vertex ids into a continuos one.
    /// In this mapping, scattered ids are real and continuos ones are virtual.
    ///
    /// # Returns
    /// The two-way mapping between scattered and continuos ids.
    fn continuos_id_map(&self) -> IdMap {
        let vertex_count = self.vertex_count();

        let mut id_map = IdMap::init(vertex_count);

        self.vertices()
            .iter()
            .enumerate()
            .for_each(|(virt_id, &real_id)| {
                id_map.put_virt_to_real(virt_id, real_id);
                id_map.put_real_to_virt(real_id, virt_id);
            });

        id_map
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool;
}

/// Provides access to edges of the graph.
pub trait Edges<W, E: Edge<W>> {
    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>>;

    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>>;
    
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// * `Some`: Containing reference to the retrieved edge.
    /// * `None`: If edge with id: `edge_id` does not exist from source vertex to destination vertex.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<Option<&E>>;

    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E>;

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
    /// * `Some`: Containing reference to the retrieved edge.
    /// * `None`: If edge with id: `edge_id` does not exist in the graph.
    fn edge(&self, edge_id: usize) -> Result<Option<&E>>;

    fn edge_unchecked(&self, edge_id: usize) -> Option<&E>;

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `true`: If there is any edge from source to destination.
    /// * `false`: Otherwise.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool>;

    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool;

    /// # Returns
    /// All edges in the graph in the format: (`src_id`, `dst_id`, `edge`).
    fn edges(&self) -> Vec<(usize, usize, &E)>;

    /// Difference between this function and `edges` is that this function treats each edge as a directed edge. \
    /// For example consider graph: a --- b \
    /// If you call `edges` on this graph, you will get: (a, b, edge). \
    /// But if you call `as_directed_edges`, you will get two elements: (a, b, edge) and (b, a, edge). \
    /// It's specifically useful in algorithms that are for directed graphs but can also be applied to undirected graphs if we treat the edges as directed.
    /// One example is [`BellmanFord`](crate::algo::BellmanFord) algorithm.
    ///
    /// # Returns
    /// All edges(as directed edges) in the graph in the format of: (`src_id`, `dst_id`, `edge`).
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)>;

    /// # Returns
    /// Number of edges in the graph.
    fn edges_count(&self) -> usize;

    fn contains_edge(&self, edge_id: usize) -> bool;
}

/// Provides basic functionalities to store graph information.
pub trait Graph<W, E: Edge<W>, Ty: EdgeDir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()>;

    fn remove_vertex_unchecked(&mut self, vertex_id: usize);

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// Unique id of the newly added edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize>;

    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) -> Result<()>;

    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E);

    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    ///
    /// # Returns
    /// * `Some`: Containing the removed edge.
    /// * `None`: If edge with `edge_id` does not exist in the graph.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<Option<E>>;

    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E>;
}
