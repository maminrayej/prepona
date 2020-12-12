mod adj_list;
mod adj_matrix;

pub use adj_list::{AdjList, DiFlowList, DiList, FlowList, List};
pub use adj_matrix::{AdjMatrix, DiFlowMat, DiMat, FlowMat, Mat};

use crate::graph::{Edge, EdgeDir};

/// Defines the api that a storage must provide in order to be usable for storing graph data.
///
/// `GraphStorage` provides default implementation for as many functions as it can so new storages can implement it easily.
/// But you should consider to provide specialized implementations instead of using the default ones.
/// Because it's likely that your implementation is faster.
///
/// A good example when providing a specialized implementation is better than the default one is in [`AdjMatrix`](crate::storage::AdjMatrix).
/// The default implementation of `edges_between` internally uses `edges_from` to compute the result.
/// But specialized implementation of `AdjMatrix` compute the result more efficiently because it can access the edges between two vertices in O(1).
///
/// As a counter example [`AdjList`](crate::storage::AdjList) uses the default implementation of `edges_between` function.
/// Because it must iterate over all edges from a source to compute the result. Which is actually the default implementation of `edges_between`.
///
/// ## Generic Parameters:
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
pub trait GraphStorage<W, E: Edge<W>, Dir: EdgeDir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex(&mut self, vertex_id: usize);

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// Unique id of the newly added edge.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        edge.set_id(edge_id);

        if let Some(_) = self.remove_edge(src_id, dst_id, edge_id) {
            self.add_edge(src_id, dst_id, edge);
        }
    }

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
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E>;

    /// # Returns
    /// Number of vertices in the graph.
    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    /// # Returns
    /// Id of vertices that are present in the graph.
    fn vertices(&self) -> Vec<usize>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.edges_from(src_id)
            .into_iter()
            .filter_map(|(d_id, edge)| if d_id == dst_id { Some(edge) } else { None })
            .collect()
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// * `Some`: Containing reference to the retrieved edge.
    /// * `None`: If edge with id: `edge_id` does not exist from source vertex to destination vertex.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
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
    /// * `Some`: Containing reference to the retrieved edge.
    /// * `None`: If edge with id: `edge_id` does not exist in the graph.
    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges()
            .into_iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .and_then(|(_, _, edge)| Some(edge))
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `true`: If there is any edge from source to destination.
    /// * `false`: Otherwise.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    /// # Returns
    /// All edges in the graph in the format: (`src_id`, `dst_id`, `edge`).
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
        self.vertices()
            .into_iter()
            .flat_map(|src_id| {
                self.edges_from(src_id)
                    .into_iter()
                    .map(|(dst_id, edge)| (src_id, dst_id, edge))
                    .collect::<Vec<(usize, usize, &E)>>()
            })
            .collect()
    }

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)>;

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// Id of vertices accessible from source vertex using one edge.
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.edges_from(src_id)
            .into_iter()
            .map(|(dst_id, _)| dst_id)
            .collect()
    }

    /// # Returns
    /// * `true`: If storage is being used to store directed edges.
    /// * 'false`: Otherwise.
    fn is_directed(&self) -> bool {
        Dir::is_directed()
    }

    /// Returns
    /// * `true`: If storage is being used to store undirected edges.
    /// * `false`: Otherwise.
    fn is_undirected(&self) -> bool {
        Dir::is_undirected()
    }
}
