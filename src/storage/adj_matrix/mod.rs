mod utils;

use std::collections::HashSet;
use std::marker::PhantomData;
use std::{any::Any, fmt::Debug};

use anyhow::Result;
use quickcheck::Arbitrary;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeDir, FlowEdge, UndirectedEdge};
use crate::storage::{Error, GraphStorage};

/// An adjacency matrix that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type Mat<W, Dir = UndirectedEdge> = AdjMatrix<W, DefaultEdge<W>, Dir>;

/// An adjacency matrix that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type DiMat<W> = AdjMatrix<W, DefaultEdge<W>, DirectedEdge>;

/// An adjacency matrix that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type FlowMat<W, Dir = UndirectedEdge> = AdjMatrix<W, FlowEdge<W>, Dir>;

/// An adjacency matrix that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type DiFlowMat<W> = AdjMatrix<W, FlowEdge<W>, DirectedEdge>;

/// `AdjMatrix` is a matrix used to represent a finite graph.
/// The elements of the matrix indicate whether pairs of vertices are adjacent or not in the graph.
///
/// ## Note
/// From now on
/// * |V|: Means total number of vertices that are stored in the storage.
/// Note that this is different from number of vertices that are valid.
/// Because even if you remove a vertex from storage, the allocated memory for that vertex will not get freed and will be reused again when adding a new vertex.
/// You can retrieve the amount of |V| using `total_vertex_count` function(as opposed to number of vertices which can be retrieved using `vertex_count` function).
/// * |E|: Means total number of edges.
/// * |E<sub>out</sub>|: Means number of edges exiting a vertex(out degree of the vertex).
/// * |E<sub>src->dst</sub>|: Means number of edges from vertex with id: `src` to vertex with id: `dst`.
///
/// ## Space complexity
/// Space complexity of `AdjMatrix` depends on wether `Dir` is [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge). \
/// * **Directed**: For directed graphs `AdjMatrix` stores matrix with |V|<sup>2</sup> + |E| elements.
/// * **Undirected**: For undirected graphs `AdjMatrix` stores a lower triangle matrix with (|V|<sup>2</sup> + |V|)/2 + |E| elements.
///
/// ## Generic Parameters
/// * `W`: **W**eight type of edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge)(default value).
pub struct AdjMatrix<W, E: Edge<W>, Dir: EdgeDir = UndirectedEdge> {
    vec: Vec<Vec<E>>,

    reusable_vertex_ids: HashSet<usize>,

    max_edge_id: usize,
    reusable_edge_ids: HashSet<usize>,

    vertex_count: usize,

    phantom_w: PhantomData<W>,
    phantom_dir: PhantomData<Dir>,
}

impl<W: Any + Clone, E: Edge<W> + Clone, Dir: EdgeDir> AdjMatrix<W, E, Dir> {
    /// Initializes an empty adjacency matrix.
    ///
    /// `AdjMatrix` defines multiple types with different combination of values for generic parameters.
    /// These types are:
    /// * [`Mat`](crate::storage::Mat): An adjacency matrix that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    /// * [`DiMat`](crate::storage::DiMat): An adjacency matrix that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    /// * [`FlowMat`](crate::storage::FlowMat): An adjacency matrix that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    /// * [`DiFlowMat`](crate::storage::DiFlowMat): An adjacency matrix that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    ///
    ///
    /// # Returns
    /// An empty `AdjMatrix`.
    ///
    /// # Examples
    /// ```
    /// use prepona::prelude::*;
    /// use prepona::storage::{Mat, DiMat};
    ///
    /// // To store an undirected graph with usize weights
    /// let mat = Mat::<usize>::init();
    ///
    /// // To store a directed graph with usize weights
    /// let di_mat = DiMat::<usize>::init();
    /// ```
    pub fn init() -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_vertex_ids: HashSet::new(),

            max_edge_id: 0,
            reusable_edge_ids: HashSet::new(),

            vertex_count: 0,

            phantom_w: PhantomData,
            phantom_dir: PhantomData,
        }
    }

    // # Returns
    // * Some: Containing an id that can be reused.
    // * None: If there is no id to reuse and storage must allocate memory for a new id.
    //
    // # Complexity
    // O(1)
    fn next_reusable_vertex_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_vertex_ids.iter().take(1).next().copied() {
            self.reusable_vertex_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    // # Returns
    // * Some: Containing an id that can be reused.
    // * None: If there is no id to reuse and storage must allocate memory for a new id.
    //
    // # Complexity
    // O(1)
    fn next_reusable_edge_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_edge_ids.iter().take(1).next().copied() {
            self.reusable_edge_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    /// # Returns
    /// Total number of vertices stored in the storage(|V|).
    ///
    /// # Complexity
    /// O(1)
    pub fn total_vertex_count(&self) -> usize {
        self.vertex_count + self.reusable_vertex_ids.len()
    }

    ////////// Checked "Vector of Edges" Retrieval Functions //////////
    // `get` and `get_mut` are "checked" functions to retrieve vector of edges from source to destination.
    // It means that if you use `get(src_id, dst_id)` and any of the two vertex ids are invalid, It causes the function to return an `Error`.
    //////////////////////////////////////////////

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // * `Ok`: Containing vector of edges from source to destination.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) If either source vertex or destination vertex does not exist in the storage.
    ///
    /// # Complexity
    /// O(1)
    fn get(&self, src_id: usize, dst_id: usize) -> Result<&Vec<E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            let index = utils::from_ij(src_id, dst_id, Dir::is_directed());

            Ok(&self.vec[index])
        }
    }

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // * `Ok`: Containing vector of edges from source to destination.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) If either source vertex or destination vertex does not exist in the storage.
    ///
    /// # Complexity
    /// O(1)
    fn get_mut(&mut self, src_id: usize, dst_id: usize) -> Result<&mut Vec<E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            let index = utils::from_ij(src_id, dst_id, Dir::is_directed());

            Ok(&mut self.vec[index])
        }
    }

    ////////// Unsafe "Vector of Edges" Retrieval Functions //////////
    // `get_unsafe` and `get_mut_unsafe` are "unsafe" alternatives for `get` and `get_mut` functions.
    // It means that if you use `get_unsafe(src_id, dst_id)` it causes the function to return a vector that belongs to a "reusable" vertex id if src_id and dst_id are in 0..|V| range and, panics otherwise.
    // So it does not always panic. The returned vector is invalid and mutating it may leave the storage in an inconsistent state.
    // The only place that you should use them is when you are absolutely sure `src_id` and `dst_id` are both valid.
    // One good example is the implementation of `edges_from`:
    //
    //    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
    //        if !self.contains_vertex(src_id) {
    //            Err(Error::new_vnf(src_id))?
    //        }
    //
    //        Ok(self
    //            .vertices()
    //            .into_iter()
    //            .flat_map(|dst_id| {
    //                self.get_unsafe(src_id, dst_id)
    //                    .iter()
    //                    .into_iter()
    //                    .map(|edge| (dst_id, edge))
    //                    .collect::<Vec<(usize, &E)>>()
    //            })
    //            .collect())
    //    }
    // As you can see `dst_id` is always valid because it comes from the vertices() function. On the other hand `src_id` is checked to be valid at the start of the function.
    // calling self.get(src_id, dst_id) for every dst_id in flat_map() is not optimal and is not even needed. Therefore calling get_unsafe is reasonable and leads to better performance.
    //
    // ** Note **: Be sure to document why your usage is reasonable whenever you use `get_unsafe` or `get_mut_unsafe`.
    //////////////////////////////////////////////

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // The vector of edges from source to destination.
    //
    // # Complexity
    // O(1)
    //
    // # Panics
    // If `src_id` or `dst_id` are are not in range 0..|V|.
    fn get_unsafe(&self, src_id: usize, dst_id: usize) -> &Vec<E> {
        let index = utils::from_ij(src_id, dst_id, Dir::is_directed());

        &self.vec[index]
    }

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // The vector of edges from source to destination.
    //
    // # Complexity
    // O(1)
    //
    // # Panics
    // If `src_id` or `dst_id` are are not in range 0..|V|.
    fn get_mut_unsafe(&mut self, src_id: usize, dst_id: usize) -> &mut Vec<E> {
        let index = utils::from_ij(src_id, dst_id, Dir::is_directed());

        &mut self.vec[index]
    }
}

impl<W: Any + Clone, E: Edge<W> + Clone, Dir: EdgeDir> GraphStorage<W, E, Dir>
    for AdjMatrix<W, E, Dir>
{
    /// Adds a vertex to the storage.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    ///
    /// # Complexity
    /// O(|V|)
    ///
    /// # Panics
    /// If number of vertices exceeds the maximum value that `usize` can represent.
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_vertex_id() {
            self.vertex_count += 1;

            reusable_id
        } else {
            let new_size = if self.is_directed() {
                // AdjMatrix must allocate a new row(|V|), a new column(|V|) and a slot for the new diagonal entry(1):
                // #    #   *
                // #    #   *
                // *    *   *
                // which causes the new length to be: old_length + |V| + |V| + 1 => old_length + 2 * |V| + 1.
                self.vec.len() + 2 * self.total_vertex_count() + 1
            } else {
                // AdjMatrix must only allocate a new row(|V|) and a slot for the new diagonal entry(1).
                // #
                // #    #
                // *    *   *
                // which causes the new length to be: old_length + |V| + 1.
                self.vec.len() + self.total_vertex_count() + 1
            };

            self.vec.resize_with(new_size, || vec![]);

            self.vertex_count += 1;

            self.vertex_count - 1
        }
    }

    /// Removes the vertex with id: `vertex_id` from storage.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Returns
    /// * `Ok`: If vertex removed successfully.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with specified id does not exist.
    ///
    /// # Complexity
    /// O(|V|)
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        }

        // `other_id` comes from `vertices()` so it's always valid. `vertex_id` is already checked to be valid.
        // So calling `get_mut_unsafe` is reasonable.
        for other_id in self.vertices() {
            self.get_mut_unsafe(vertex_id, other_id).clear();
            self.get_mut_unsafe(other_id, vertex_id).clear();
        }

        self.reusable_vertex_ids.insert(vertex_id);

        self.vertex_count -= 1;

        Ok(())
    }

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
    ///
    /// # Complexity
    /// O(1)
    fn add_edge(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> Result<usize> {
        let edge_id = if let Some(id) = self.next_reusable_edge_id() {
            id
        } else {
            self.max_edge_id += 1;

            self.max_edge_id - 1
        };

        edge.set_id(edge_id);

        self.get_mut(src_id, dst_id)?.push(edge);

        Ok(edge_id)
    }

    /// Replaces the edge with id: `edge_id` with `edge`.
    ///
    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the old edge.
    /// * `edge`: New edge to replace the old one.
    ///
    /// # Returns
    /// * `Ok`: If edge updated successfully.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`InvalidEdgeId`](crate::storage::ErrorKind::InvalidEdgeId) if edge with specified id does not exist from source to destination.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn update_edge(
        &mut self,
        src_id: usize,
        dst_id: usize,
        edge_id: usize,
        mut edge: E,
    ) -> Result<()> {
        let edges_vec = self.get_mut(src_id, dst_id)?;

        if let Some(index) = edges_vec.iter().position(|edge| edge.get_id() == edge_id) {
            edge.set_id(edge_id);
            edges_vec[index] = edge;

            Ok(())
        } else {
            Err(Error::new_iei(src_id, dst_id, edge_id))?
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
    /// * `Ok`: Containing the removed edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`InvalidEdgeId`](crate::storage::ErrorKind::InvalidEdgeId) if edge with specified id does not exist from source to destination.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        if let Some(index) = self
            .get_mut(src_id, dst_id)?
            .iter()
            .position(|edge| edge.get_id() == edge_id)
        {
            self.reusable_edge_ids.insert(edge_id);

            // Calling `get_mut(src_id, dst_id)` at the start of this function made sure that both ids are valid.
            // So it's reasonable to call `get_mut_unsafe`.
            Ok(self.get_mut_unsafe(src_id, dst_id).swap_remove(index))
        } else {
            Err(Error::new_iei(src_id, dst_id, edge_id))?
        }
    }

    /// # Returns
    /// Number of vertices in the storage.
    ///
    /// # Complexity
    /// O(1)
    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// # Returns
    /// Number of edges in the storage.
    ///
    /// # Complexity:
    /// O(1)
    fn edge_count(&self) -> usize {
        self.edges().len()
    }

    /// # Returns
    /// Id of vertices that are present in the storage.
    ///
    /// # Complexity
    /// O(|V|)
    fn vertices(&self) -> Vec<usize> {
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|v_id| !self.reusable_vertex_ids.contains(v_id))
            .collect()
    }

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Ok`: Containing all edges from the source vertex in the format of: (`dst_id`, `edge`).
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with id: `src_id` does not exist.
    ///
    /// # Complexity
    /// O(|E<sub>out</sub>|)
    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        }

        // Map each dst_id to the list of edges that start from src_id and end in dst_id.
        // Then flatten all the lists into a final list that contains all the outgoing edges of vertex with id: src_id.
        Ok(self
            .vertices()
            .into_iter()
            .flat_map(|dst_id| {
                // `dst_id` comes from  `vertices()` so it's always valid. `src_id` is checked to be valid at the start of this function.
                // So it's reasonable to use `get_unsafe`.
                self.get_unsafe(src_id, dst_id)
                    .into_iter()
                    .map(|edge| (dst_id, edge))
                    .collect::<Vec<(usize, &E)>>()
            })
            .collect())
    }

    /// # Arguments:
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * `Ok`: Containing id of vertices accessible from source vertex using one edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with id: `src_id` does not exist.
    ///
    /// # Complexity
    /// O(|V|)
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        }

        // `dst_id` comes from `vertices()` so it's always valid. `src_id` is checked to be valid at the start of this function.
        // So it's reasonable to call `get_unsafe`.
        Ok(self
            .vertices()
            .into_iter()
            .filter(|dst_id| !self.get_unsafe(src_id, *dst_id).is_empty())
            .collect())
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Ok`: Edges from source vertex to destination vertex.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        Ok(self.get(src_id, dst_id)?.iter().collect())
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
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        self.get(src_id, dst_id)?
            .iter()
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
            .ok_or(Error::new_iei(src_id, dst_id, edge_id).into())
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
    ///
    /// # Complexity
    /// O(|E|)
    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        let vertices = self.vertices();

        let mut edges = vec![];

        for src_id in vertices.iter() {
            for dst_id in vertices.iter() {
                // `src_id` and `dst_id` both come from `vertices()`. Making them both valid.
                // So using `get_unsafe` is reasonable.
                let mut src_to_dst_edges = self
                    .get_unsafe(*src_id, *dst_id)
                    .iter()
                    .map(|edge| (*src_id, *dst_id, edge))
                    .collect::<Vec<(usize, usize, &E)>>();

                edges.append(&mut src_to_dst_edges);
            }
        }

        edges
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// * `Ok`: Containing `true` if there is any edge between specified source and destination, `false` otherwise.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    ///
    /// # Complexity
    /// O(1)
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        Ok(!self.get(src_id, dst_id)?.is_empty())
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
    ///
    /// # Complexity
    /// O(|E|)
    fn edge(&self, edge_id: usize) -> Result<&E> {
        self.edges()
            .into_iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .map(|(_, _, edge)| edge)
            .ok_or(Error::new_enf(edge_id).into())
    }

    /// # Arguments
    /// `vertex_id`: Id of the vertex to search for its existence in the storage.
    ///
    /// # Returns
    /// * `true`: If storage contains the vertex with id: `vertex_id`.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O(1)
    fn contains_vertex(&self, vertex_id: usize) -> bool {
        vertex_id < self.total_vertex_count() && !self.reusable_vertex_ids.contains(&vertex_id)
    }

    /// # Arguments
    /// `edge_id`: Id of the edge to search for its existence in the storage.
    ///
    /// # Returns
    /// * `true`: If storage contains the edge with id: `edge_id`.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O(1)
    fn contains_edge(&self, edge_id: usize) -> bool {
        self.edges()
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .is_some()
    }

    /// Filters vertices using `vertex_filter` and then filters edges.
    /// Note that edges that are passed to `edge_filter` are the ones that their source and destination vertices are included by `vertex_filter`.
    ///
    /// # Arguments
    /// * `vertex_filter`: Function that receives a vertex id as an argument and returns `true` if vertex should be included in the returned storage.
    /// * `edge_filter`: Function that receives (source id, destination id, edge) and returns `true` if edge should be included in the returned storage.
    ///
    /// # Returns
    /// Storage containing vertices and edges filtered by `vertex_filter` and `edge_filter`.
    fn filter(
        &self,
        vertex_filter: impl FnMut(&usize) -> bool,
        mut edge_filter: impl FnMut(&usize, &usize, &E) -> bool,
    ) -> Self {
        // 1. Filter the vertices
        let filtered_vertices: Vec<usize> =
            self.vertices().into_iter().filter(vertex_filter).collect();

        // 2. Filter edges: Only edges that their source and destination vertices are among filtered vertices and also `edge_filter` returns true for them are included.
        let filtered_edges: Vec<(usize, usize, &E)> = self
            .edges()
            .into_iter()
            .filter(|(src_id, dst_id, edge)| {
                filtered_vertices.contains(src_id)
                    && filtered_vertices.contains(dst_id)
                    && edge_filter(src_id, dst_id, edge)
            })
            .collect();

        // 3. Initialize an empty storage.
        let mut storage = AdjMatrix::init();

        // 4. Add the vertices to the new storage.
        for _ in &filtered_vertices {
            storage.add_vertex();
        }

        // 5. Add filtered edges.
        for (src_id, dst_id, edge) in &filtered_edges {
            // We use index of each id in the filtered_vertices as its new id in the new storage.
            let src_new_id = filtered_vertices
                .iter()
                .position(|vertex_id| vertex_id == src_id)
                .unwrap();
            let dst_new_id = filtered_vertices
                .iter()
                .position(|vertex_id| vertex_id == dst_id)
                .unwrap();

            storage
                .add_edge(src_new_id, dst_new_id, (*edge).clone())
                .unwrap();
        }

        storage
    }
}

impl<W: Any + Clone, E: Edge<W> + Clone, Dir: EdgeDir> Clone for AdjMatrix<W, E, Dir> {
    fn clone(&self) -> Self {
        AdjMatrix {
            vec: self.vec.clone(),

            reusable_vertex_ids: self.reusable_vertex_ids.clone(),

            max_edge_id: self.max_edge_id,
            reusable_edge_ids: self.reusable_edge_ids.clone(),

            vertex_count: self.vertex_count,

            phantom_w: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<W: Any + Clone, E: Edge<W> + Arbitrary, Dir: EdgeDir + 'static> Arbitrary
    for AdjMatrix<W, E, Dir>
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let vertex_count = usize::arbitrary(g).clamp(0, 32);

        let edge_prob = rand::random::<f64>() * rand::random::<f64>();

        let mut storage = AdjMatrix::init();

        for _ in 0..vertex_count {
            storage.add_vertex();
        }

        let vertices = storage.vertices();

        for src_id in &vertices {
            for dst_id in &vertices {
                if storage.is_undirected() && src_id > dst_id {
                    continue;
                }

                let add_edge_prob = rand::random::<f64>();
                if add_edge_prob < edge_prob {
                    storage.add_edge(*src_id, *dst_id, E::arbitrary(g)).unwrap();
                }
            }
        }

        storage
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let graph = self.clone();
        Box::new((0..2).filter_map(move |partition_index| {
            let mut vertex_index = -1;
            let graph_partition = graph.filter(
                |_| {
                    vertex_index += 1;

                    vertex_index % 2 == partition_index
                },
                |_, _, _| true,
            );

            if graph_partition.vertex_count() < graph.vertex_count() {
                Some(graph_partition)
            } else {
                None
            }
        }))
    }
}

impl<W: Any + Clone + Debug, E: Edge<W> + Clone + Debug, Dir: EdgeDir> Debug
    for AdjMatrix<W, E, Dir>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let edges_str: Vec<String> = self
            .edges()
            .into_iter()
            .map(|(src_id, dst_id, edge)| format!("({}->{}: {:?})", src_id, dst_id, edge))
            .collect();

        f.write_str(&edges_str.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_empty_matrix() {
        // Given: An empty directed matrix.
        let matrix = DiMat::<usize>::init();

        // Then:
        assert_eq!(matrix.edge_count(), 0);
        assert_eq!(matrix.vertex_count(), 0);
        assert_eq!(matrix.total_vertex_count(), 0);
        assert_eq!(matrix.vec.len(), 0);
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);
        assert_eq!(matrix.is_directed(), true);
    }

    #[test]
    fn undirected_empty_matrix() {
        // Given: An empty undirected matrix.
        let matrix = Mat::<usize>::init();

        // Then:
        assert_eq!(matrix.edge_count(), 0);
        assert_eq!(matrix.vertex_count(), 0);
        assert_eq!(matrix.total_vertex_count(), 0);
        assert_eq!(matrix.vec.len(), 0);
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);
        assert_eq!(matrix.is_directed(), false);
    }

    #[test]
    fn directed_add_vertex() {
        // Given: An empty directed matrix.
        let mut matrix = DiMat::<usize>::init();

        // When: Adding 3 vertices.
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);

        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex() {
        // Given: An empty undirected matrix.
        let mut matrix = Mat::<usize>::init();

        // When: Adding 3 vertices.
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);

        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_delete_vertex() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Removing vertices a and b.
        matrix.remove_vertex(a).unwrap();
        matrix.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(matrix.edges().len(), 0);
        assert_eq!(matrix.vertex_count(), 1);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        // Vertices a and b must be reusable.
        assert_eq!(matrix.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.reusable_vertex_ids.contains(vertex_id)));

        // Matrix must only contain c.
        assert_eq!(matrix.vertices().len(), 1);
        assert!(matrix.vertices().contains(&c));
        assert!(!matrix.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn undirected_delete_vertex() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Removing vertices a and b.
        matrix.remove_vertex(a).unwrap();
        matrix.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 1);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        // Vertices a and b must be reusable.
        assert_eq!(matrix.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.reusable_vertex_ids.contains(vertex_id)));

        // Matrix must only contain c.
        assert_eq!(matrix.vertices().len(), 1);
        assert!(matrix.vertices().contains(&c));

        assert!(!matrix.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn directed_add_vertex_after_vertex_deletion() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        matrix.remove_vertex(a).unwrap();
        matrix.remove_vertex(b).unwrap();
        let _ = matrix.add_vertex();
        let _ = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        // There must be no reusable id.
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex_after_vertex_deletion() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        matrix.remove_vertex(a).unwrap();
        matrix.remove_vertex(b).unwrap();
        let _ = matrix.add_vertex();
        let _ = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        // There must be no reusable id.
        assert_eq!(matrix.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_add_edge() {
        // Given: Directed matrix
        //
        //      a   b   c
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Adding edges
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 3);
        for (src_id, dst_id, edge) in matrix.edges() {
            match (src_id, dst_id) {
                (0, 1) => assert_eq!(edge.get_weight().unwrap(), 1),
                (1, 2) => assert_eq!(edge.get_weight().unwrap(), 2),
                (2, 0) => assert_eq!(edge.get_weight().unwrap(), 3),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn undirected_add_edge() {
        // Given: Undirected matrix
        //
        //      a   b   c
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();

        // When: Adding edges
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 3);
        for (src_id, dst_id, edge) in matrix.edges() {
            match (src_id, dst_id) {
                (0, 1) | (1, 0) => assert_eq!(edge.get_weight().unwrap(), 1),
                (1, 2) | (2, 1) => assert_eq!(edge.get_weight().unwrap(), 2),
                (2, 0) | (0, 2) => assert_eq!(edge.get_weight().unwrap(), 3),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn directed_has_edge() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge(a, b).unwrap());
        assert!(matrix.has_any_edge(b, c).unwrap());
        assert!(matrix.has_any_edge(c, a).unwrap());
    }

    #[test]
    fn undirected_has_edge() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge(a, b).unwrap());
        assert!(matrix.has_any_edge(b, a).unwrap());

        assert!(matrix.has_any_edge(b, c).unwrap());
        assert!(matrix.has_any_edge(c, b).unwrap());

        assert!(matrix.has_any_edge(c, a).unwrap());
        assert!(matrix.has_any_edge(a, c).unwrap());
    }

    #[test]
    fn directed_update_edge() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        let ab = matrix.add_edge(a, b, 1.into()).unwrap();
        let bc = matrix.add_edge(b, c, 2.into()).unwrap();
        let ca = matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing weight of each edge by 1.
        matrix.update_edge(a, b, ab, 2.into()).unwrap();
        matrix.update_edge(b, c, bc, 3.into()).unwrap();
        matrix.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 3);
        for (src_id, dst_id, edge) in matrix.edges() {
            match (src_id, dst_id) {
                (0, 1) => assert_eq!(edge.get_weight().unwrap(), 2),
                (1, 2) => assert_eq!(edge.get_weight().unwrap(), 3),
                (2, 0) => assert_eq!(edge.get_weight().unwrap(), 4),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn undirected_update_edge() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        let ab = matrix.add_edge(a, b, 1.into()).unwrap();
        let bc = matrix.add_edge(b, c, 2.into()).unwrap();
        let ca = matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing weight of each edge by 1.
        matrix.update_edge(a, b, ab, 2.into()).unwrap();
        matrix.update_edge(b, c, bc, 3.into()).unwrap();
        matrix.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 3);
        for (src_id, dst_id, edge) in matrix.edges() {
            match (src_id, dst_id) {
                (0, 1) | (1, 0) => assert_eq!(edge.get_weight().unwrap(), 2),
                (1, 2) | (2, 1) => assert_eq!(edge.get_weight().unwrap(), 3),
                (2, 0) | (0, 2) => assert_eq!(edge.get_weight().unwrap(), 4),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn directed_remove_edge() {
        // Given: Directed matrix
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        let ab = matrix.add_edge(a, b, 1.into()).unwrap();
        let bc = matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        matrix.remove_edge(a, b, ab).unwrap();
        matrix.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(
            matrix.edges_between(c, a).unwrap()[0].get_weight().unwrap(),
            3
        );
    }

    #[test]
    fn undirected_remove_edge() {
        // Given: Undirected matrix
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        let ab = matrix.add_edge(a, b, 1.into()).unwrap();
        let bc = matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        matrix.remove_edge(a, b, ab).unwrap();
        matrix.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(
            matrix.edges_between(a, c).unwrap()[0].get_weight().unwrap(),
            3
        );
    }

    #[test]
    fn directed_neighbors() {
        // Given: Directed matrix
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut matrix = DiMat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.neighbors(a).unwrap().len(), 1);
        assert_eq!(*matrix.neighbors(a).unwrap().get(0).unwrap(), b);

        assert_eq!(matrix.neighbors(b).unwrap().len(), 1);
        assert_eq!(*matrix.neighbors(b).unwrap().get(0).unwrap(), c);

        assert_eq!(matrix.neighbors(c).unwrap().len(), 1);
        assert_eq!(*matrix.neighbors(c).unwrap().get(0).unwrap(), a);
    }

    #[test]
    fn undirected_neighbors() {
        // Given: Undirected matrix
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();
        let c = matrix.add_vertex();
        matrix.add_edge(a, b, 1.into()).unwrap();
        matrix.add_edge(b, c, 2.into()).unwrap();
        matrix.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.neighbors(a).unwrap().len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| matrix.neighbors(a).unwrap().contains(vertex_id)));

        assert_eq!(matrix.neighbors(b).unwrap().len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| matrix.neighbors(b).unwrap().contains(vertex_id)));

        assert_eq!(matrix.neighbors(c).unwrap().len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.neighbors(c).unwrap().contains(vertex_id)));
    }

    #[test]
    fn edge_count() {
        // Undirected
        // Given: an empty matrix.
        let mut mat = Mat::<usize>::init();

        // When: adding 3 edges.
        let a = mat.add_vertex();
        let b = mat.add_vertex();
        let c = mat.add_vertex();
        let ab = mat.add_edge(a, b, 1.into()).unwrap();
        let bc = mat.add_edge(b, c, 1.into()).unwrap();
        let ca = mat.add_edge(c, a, 1.into()).unwrap();

        // Then: it must have 3 edges.
        assert_eq!(mat.edge_count(), 3);

        // When removing the 3 edges.
        mat.remove_edge(a, b, ab).unwrap();
        mat.remove_edge(b, c, bc).unwrap();
        mat.remove_edge(c, a, ca).unwrap();

        // Then: it must have zero edges again.
        assert_eq!(mat.edge_count(), 0);

        // Directed
        // Given: an empty matrix.
        let mut di_mat = DiMat::<usize>::init();

        // When: adding 3 edges.
        let a = di_mat.add_vertex();
        let b = di_mat.add_vertex();
        let c = di_mat.add_vertex();
        let ab = di_mat.add_edge(a, b, 1.into()).unwrap();
        let bc = di_mat.add_edge(b, c, 1.into()).unwrap();
        let ca = di_mat.add_edge(c, a, 1.into()).unwrap();

        // Then: it must have 3 edges.
        assert_eq!(di_mat.edge_count(), 3);

        // When: removing the 3 edges.
        di_mat.remove_edge(a, b, ab).unwrap();
        di_mat.remove_edge(b, c, bc).unwrap();
        di_mat.remove_edge(c, a, ca).unwrap();

        // Then: it must have zero edges again.
        assert_eq!(di_mat.edge_count(), 0);
    }
}
