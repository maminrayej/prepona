mod adj_list;
mod adj_matrix;
mod error;

pub use adj_list::{AdjList, DiFlowList, DiList, FlowList, List};
pub use adj_matrix::{AdjMatrix, DiFlowMat, DiMat, FlowMat, Mat};
pub use error::{Error, ErrorKind};

use crate::graph::{Edge, EdgeDir};

use anyhow::Result;

/// Defines the api that a storage must provide in order to be usable for storing graph data.
///
/// ## Implementing a storage
/// `GraphStorage` provides default implementation for as many functions as it can so new storages can implement it easily.
/// But you should consider to provide specialized implementations instead of using the default ones.
/// Because it's likely that your implementation is faster.
///
/// A good example when providing a specialized implementation is better than the default one is in [`AdjMatrix`](crate::storage::AdjMatrix).
/// The default implementation of `edges_between` internally uses `edges_from` to compute the result.
/// But specialized implementation of `AdjMatrix` compute the result more efficiently because it can access the edges between two vertices in O(1).
/// As a counter example [`AdjList`](crate::storage::AdjList) uses the default implementation of `edges_between` function.
/// Because it must iterate over all edges from a source to compute the result. Which is actually the default implementation of `edges_between`.
///
/// ## Using a storage
/// Regarding using storages(if you want to implement a new type of graph for example), Many functions have two checked(default) and unchecked versions.
/// In checked versions some checking will occur before computing the end result. So these functions will return `Result` and will not panic.
/// But checking on every call will have some overheads so if you want to skip the checking(For example you are sure some invariants are never violated),
/// You can use unchecked version of the function. Regarding that what situations will cause a function to `panic` or return `Err`,
/// It depends on wether storage uses the default implementation provided by `GraphStorage` or not. Causes of returning `Err` is specified for default implementations.
/// But if storage does use the default implementation, refer to the documentation of the storage to find out about causes of `panic` or `Err`.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
pub trait GraphStorage<W, E: Edge<W>, Dir: EdgeDir> {
    /// Adds a vertex to the storage.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    fn add_vertex(&mut self) -> usize;

    /// Removes the vertex with id: `vertex_id` from storage.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Returns
    /// * `Ok`: If vertex removed successfully.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with specified id does not exist.
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            Ok(self.remove_vertex_unchecked(vertex_id))
        }
    }

    /// Removes the vertex with id: `vertex_id` from storage.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize);

    /// # Arguments
    /// `vertex_id`: Id of the vertex.
    ///
    /// # Returns
    /// * `true`: if storage contains the vertex with specified id.
    /// * `false`: otherwise.
    fn contains_vertex(&self, vertex_id: usize) -> bool;

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// * `Ok`: Containing unique id of the newly added edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) -> Result<usize> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self.add_edge_unchecked(src_id, dst_id, edge))
        }
    }

    /// Adds `edge` from vertex with id `src_id`: to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// Unique id of the newly added edge.
    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge: E) -> usize;

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    ///
    /// # Returns
    /// * `Ok`: If edge updated successfully.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`EdgeNotFound`](crate::storage::ErrorKind::EdgeNotFound) if edge with specified id does not exist.
    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, edge: E) -> Result<()> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.update_edge_unchecked(src_id, dst_id, edge_id, edge))
        }
    }

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        edge.set_id(edge_id);

        self.remove_edge_unchecked(src_id, dst_id, edge_id);

        self.add_edge_unchecked(src_id, dst_id, edge);
    }

    /// Removes the edge with id: `edge_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of edge to be removed.
    ///
    /// # Returns
    /// * `Ok`: Containing the removed edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`EdgeNotFound`](crate::storage::ErrorKind::EdgeNotFound) if edge with specified id does not exist.
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.remove_edge_unchecked(src_id, dst_id, edge_id))
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
    /// The removed edge.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E;

    fn contains_edge(&self, edge_id: usize) -> bool;

    /// # Returns
    /// Number of vertices in the storage.
    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }

    /// # Returns
    /// Id of vertices that are present in the storage.
    fn vertices(&self) -> Vec<usize>;

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Ok`: Edges from source vertex to destination vertex.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self.edges_between_unchecked(src_id, dst_id))
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.edges_from_unchecked(src_id)
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
    /// * `Ok`: Containing the edge between specified source and destination with specified id.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`EdgeNotFound`](crate::storage::ErrorKind::EdgeNotFound) if edge with specified id does not exist.
    /// * `Err`: [`InvalidEdgeId`](crate::storage::ErrorKind::InvalidEdgeId) if edge with specified id does exist but it's not from source to destination.
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            let edge = self
                .edges_between_unchecked(src_id, dst_id)
                .into_iter()
                .find(|edge| edge.get_id() == edge_id);

            if edge.is_none() {
                Err(Error::new_iei(src_id, dst_id, edge_id))?
            } else {
                Ok(edge.unwrap())
            }
        }
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// Edge between specified source and destination with specified id.
    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E {
        self.edges_between_unchecked(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
            .unwrap()
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
    /// * `Ok`: Containing reference to the edge with specified id.
    /// * `Err`: [`EdgeNotFound`](crate::storage::ErrorKind::EdgeNotFound) if edge with specified id does not exist.
    fn edge(&self, edge_id: usize) -> Result<&E> {
        if !self.contains_edge(edge_id) {
            Err(Error::new_enf(edge_id))?
        } else {
            Ok(self.edge_unchecked(edge_id))
        }
    }

    /// # Note:
    /// Consider using `edge_between_unchecked` or `edges_from_unchecked` functions instead of this one.
    /// Because default implementation of this function iterates over all edges to find the edge with specified id.
    /// And it's likely that other storages use the same approach. So:
    /// * if you have info about source of the edge, consider using `edges_from_unchecked` function instead.
    /// * if you have info about both source and destination of the edge, consider using `edge_between_unchecked` function instead.
    ///
    /// # Arguments
    /// `edge_id`: Id of the edge to be retrieved.
    ///
    /// # Returns
    /// Containing reference to the edge with specified id.
    fn edge_unchecked(&self, edge_id: usize) -> &E {
        self.edges()
            .into_iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .and_then(|(_, _, edge)| Some(edge))
            .unwrap()
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Ok`: Containing `true` if there is any edge between specified source and destination, `false` otherwise.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            Ok(self.has_any_edge_unchecked(src_id, dst_id))
        }
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `true` if there is any edge between specified source and destination, `false` otherwise.
    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between_unchecked(src_id, dst_id).is_empty()
    }

    /// # Returns
    /// All edges in the storage in the format: (`src_id`, `dst_id`, `edge`).
    fn edges(&self) -> Vec<(usize, usize, &E)> {
        if Dir::is_directed() {
            self.as_directed_edges()
        } else {
            // Returning only triplets that have src_id <= dst_id property will filter duplicate edges.
            // Because storage is undirected, there is no difference between returning (1, 2, edge) or (2, 1, edge).
            // For example as_directed_edges() returns both (1, 2, edge) and (2, 1, edge). (2, 1, edge) will get removed.
            self.as_directed_edges()
                .into_iter()
                .filter(|(src_id, dst_id, _)| src_id <= dst_id)
                .collect()
        }
    }

    /// # Returns
    /// Number of edges present in the storage.
    fn edge_count(&self) -> usize {
        self.edges().len()
    }

    /// Difference between this function and `edges` is that this function treats each edge as a directed edge.
    /// For example consider storage: a --- b
    /// If you call `edges` on this storage, you will get: (a, b, edge).
    /// But if you call `as_directed_edges`, you will get two elements: (a, b, edge) and (b, a, edge).
    /// It's specifically useful in algorithms that are for directed graphs but can also be applied to undirected graphs if we treat the edges as directed.
    /// One example is [`BellmanFord`](crate::algo::BellmanFord) algorithm.
    ///
    /// # Returns
    /// All edges(as directed edges) in the storage in the format of: (`src_id`, `dst_id`, `edge`).
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        // Map each vertex to its list of outgoing edges:
        // v1 -> { (v1, v2, e12), (v1, v3, e13), ... }
        // Then combine all these generated lists into a final flat list which contains all the outgoing edges:
        // { { (v1, v2, e12), (v1, v3, e13) }, { (v2, v3, e23) } } -> { (v1, v2, e12), (v1, v3, e13), (v2, v3, e23) }
        // For a directed graph the final list will contain the edges but for an undirected graph the final list will contain two instance of each edge.
        // One instance is (src_id, dst_id, edge) and the other is (dst_id, src_id, edge).
        self.vertices()
            .into_iter()
            .flat_map(|src_id| {
                self.edges_from_unchecked(src_id)
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
    /// * `Ok`: Containing all edges from the source vertex in the format of: (`dst_id`, `edge`).
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with id: `src_id` does not exist.
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self.edges_from_unchecked(src_id))
        }
    }

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// All edges from the source vertex in the format of: (`dst_id`, `edge`).
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)>;

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Ok`: Containing id of vertices accessible from source vertex using one edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with id: `src_id` does not exist.
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else {
            Ok(self.neighbors_unchecked(src_id))
        }
    }

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// Id of vertices accessible from source vertex using one edge.
    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        self.edges_from_unchecked(src_id)
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
