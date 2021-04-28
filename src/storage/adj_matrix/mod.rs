mod utils;

use std::any::Any;
use std::collections::HashSet;
use std::marker::PhantomData;

use anyhow::Result;

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

// TODO: State in documentation that `unchecked` means as little checking as possible.

/// `AdjMatrix` is a matrix used to represent a finite graph.
/// The elements of the matrix indicate whether pairs of vertices are adjacent or not in the graph.
///
/// ## Note
/// From now on
/// * |V|: Means total number of vertices that are stored in the storage.
/// Note that this is different from number of vertices that are present in the graph.
/// Because even if you remove a vertex from storage, the allocated memory for that vertex will not get freed and will be reused again when adding a new vertex.
/// You can retrieve the amount of |V| using `total_vertex_count` function(as opposed to number of vertices present in the graph which can be retrieved using `vertex_count` function).
/// For more info and examples refer to `total_vertex_count` documentation.
/// * |E|: Means number of edges present in the graph.
/// * |E<sup>out</sup>|: Means number of edges exiting a vertex(out degree of the vertex).
/// * |E<sub>src->dst</sub>|: Means number of edges from vertex with id: `src` to vertex with id: `dst`.
///
/// ## Space complexity
/// Space complexity of `AdjMatrix` depends on wether `Dir` is [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge). \
/// * **Directed**: For directed graphs `AdjMatrix` stores matrix with |V|<sup>2</sup> + |E| elements.
/// * **Undirected**: For undirected graphs `AdjMatrix` stores a lower triangle matrix with (|V|<sup>2</sup> + |V|)/2 + |E| elements.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
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

impl<W: Any, E: Edge<W>, Dir: EdgeDir> AdjMatrix<W, E, Dir> {
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
    /// Total number of vertices in the storage(|V|).
    ///
    /// # Complexity
    /// O(1)
    pub fn total_vertex_count(&self) -> usize {
        self.vertex_count + self.reusable_vertex_ids.len()
    }

    ////////// Checked Edges Vec Retrieval Functions //////////
    // `get` and `get_mut` are "checked" alternatives for `index` and `index_mut` functions.
    // It means that if you use `self[(src_id, dst_id)]` and any of the two vertex ids are invalid, It causes the function to panic.
    // On the other hand `get(src_id, dst_id)` just returns an `Error`.
    // So in unchecked version of a function use `index` or `index_mut` and in checked version use `get` or `get_mut`.
    //////////////////////////////////////////////

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // * `Ok`: Containing vector of edges from source to destination.
    // * `Err`: If either source vertex or destination vertex does not exist in the storage.
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
    // * `Err`: If either source vertex or destination vertex does not exist in the storage.
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

    ////////// Unsafe Edges Vec Retrieval Functions //////////
    // `get_unsafe` and `get_mut_unsafe` are "unsafe" alternatives tor `index` and `index_mut` functions.
    // It means that if you use `self[(src_id, dst_id)]` and any of the two vertex ids are invalid, It causes the function to panic.
    // On the other hand `get_unsafe(src_id, dst_id)` will return a vector that belongs to a "reusable" vertex id if src_id and dst_id are in 0..|V| range and panics otherwise.
    // So it does not always panic. The returned vector is invalid and mutating it may leave the storage in an inconsistent state.
    // The only place that you should use them is when you are absolutely sure `src_id` and `dst_id` are both valid.
    // One good example is the implementation of `edges_from_unchecked`:
    //
    //    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
    //        if !self.contains_vertex(src_id) {
    //            panic!("Vertex with id: {} does not exist", src_id);
    //        }
    //
    //        self.vertices()
    //            .into_iter()
    //            .flat_map(|dst_id| {
    //                self.get_unsafe(src_id, dst_id)
    //                    .iter()
    //                    .into_iter()
    //                    .map(|edge| (dst_id, edge))
    //                    .collect::<Vec<(usize, &E)>>()
    //            })
    //            .collect()
    //    }
    // As you can see `dst_id` is always valid because it comes from the vertices() function. On the other hand `src_id` is checked to be valid at the start of the function.
    // calling self.get(src_id, dst_id) for every dst_id in flat_map() is not optimal and is not even needed. Therefore calling get_unsafe is reasonable and leads to better performance.
    //////////////////////////////////////////////

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // The vector of edges from source to destination.
    ///
    /// # Complexity
    /// O(1)
    //
    // # Panics
    // If `src_id` or `dst_id` are not stored in the storage.
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
    ///
    /// # Complexity
    /// O(1)
    //
    // # Panics
    // If `src_id` or `dst_id` are not stored in the storage.
    fn get_mut_unsafe(&mut self, src_id: usize, dst_id: usize) -> &mut Vec<E> {
        let index = utils::from_ij(src_id, dst_id, Dir::is_directed());

        &mut self.vec[index]
    }
}

impl<W: Any, E: Edge<W>, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjMatrix<W, E, Dir> {
    /// Adds a vertex to the graph.
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

    /// Removes the vertex with id: `vertex_id` from graph.
    ///
    /// # Arguments
    /// `vertex_id`: Id of the vertex to be removed.
    ///
    /// # Complexity
    /// O(|V|)
    ///
    /// # Panics
    /// If there is no vertex with id: `vertex_id` in the storage.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        for other_id in self.vertices() {
            self[(vertex_id, other_id)].clear();
            self[(other_id, vertex_id)].clear();
        }

        self.reusable_vertex_ids.insert(vertex_id);

        self.vertex_count -= 1;
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

    /// Adds `edge` from vertex with id :`src_id` to vertex with id: `dst_id`.
    ///
    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    /// * `edge`: Edge to be added from source to destination.
    ///
    /// # Returns
    /// Unique id of the newly added edge.
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// If `src_id` or `dst_id` are not stored in the storage.
    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> usize {
        let edge_id = if let Some(id) = self.next_reusable_edge_id() {
            id
        } else {
            self.max_edge_id += 1;

            self.max_edge_id - 1
        };

        edge.set_id(edge_id);

        self[(src_id, dst_id)].push(edge);

        edge_id
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
    /// * `edge_id`: Id of the to be updated edge.
    /// * `edge`: New edge to replace the old one.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` are not stored in the storage.
    /// * If there is no edge from source to destination with id: `edge_id`.
    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        let edges_vec = &mut self[(src_id, dst_id)];

        let index = edges_vec
            .iter()
            .position(|edge| edge.get_id() == edge_id)
            .unwrap();

        edge.set_id(edge_id);
        edges_vec[index] = edge;
    }

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
    /// * `Some`: Containing the removed edge.
    /// * `None`: If edge with `edge_id` does not exist in the graph.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` are not stored in the storage.
    /// * If there is no edge with id: `edge_id` from `src_id` to `dst_id`.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E {
        let index = self[(src_id, dst_id)]
            .iter()
            .position(|edge| edge.get_id() == edge_id)
            .unwrap();

        self.reusable_edge_ids.insert(edge_id);

        // If any of `src_id` or `dst_id` were invalid, self[(src_id, dst_id)] would have panicked.
        // So calling `get_mut_unsafe` is reasonable.
        self.get_mut_unsafe(src_id, dst_id).swap_remove(index)
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
    /// Number of vertices in the graph.
    ///
    /// # Complexity
    /// O(1)
    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// # Returns
    /// Number of edges in the graph.
    ///
    /// # Complexity:
    /// O(1)
    fn edge_count(&self) -> usize {
        self.max_edge_id - self.reusable_edge_ids.len()
    }

    /// # Returns
    /// Id of vertices that are present in the graph.
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
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    ///
    /// # Complexity
    /// O(|E<sup>out</sup>|)
    ///
    /// # Panics
    /// If `src_id` is not stored in the storage.
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        if !self.contains_vertex(src_id) {
            panic!("Vertex with id: {} does not exist", src_id);
        }

        // Map each dst_id to the list of edges that start from src_id and end in dst_id.
        // Then flatten all the lists into a final list that contains all the outgoing edges of vertex with id: src_id.
        self.vertices()
            .into_iter()
            .flat_map(|dst_id| {
                // `dst_id` comes from  `vertices()` so it's always valid. `src_id` is checked to be valid at the start of this function.
                // So it's reasonable to use `get_unsafe`.
                self.get_unsafe(src_id, dst_id)
                    .iter()
                    .into_iter()
                    .map(|edge| (dst_id, edge))
                    .collect::<Vec<(usize, &E)>>()
            })
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
    /// O(|E<sup>out</sup>|)
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
                    .iter()
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
    /// Id of vertices accessible from source vertex using one edge.
    ///
    /// Complexity
    /// O(|V|)
    ///
    /// # Panics
    /// If `src_id` is not stored in the storage.
    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        if !self.contains_vertex(src_id) {
            panic!("Vertex with id: {} does not exist", src_id);
        }

        // `dst_id` comes from `vertices()` so it's always valid. `src_id` is checked to be valid at the start of this function.
        // So it's reasonable to call `get_unsafe`.
        self.vertices()
            .into_iter()
            .filter(|dst_id| !self.get_unsafe(src_id, *dst_id).is_empty())
            .collect()
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
    /// Edges from source vertex to destination vertex.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    ///
    /// # Panics
    /// * If vertex with either id: `src_id` or `dst_id` does not exist in the storage.
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self[(src_id, dst_id)].iter().collect()
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
        self.edges_between(src_id, dst_id)?
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
            .ok_or(Error::new_iei(src_id, dst_id, edge_id).into())
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    /// * `edge_id`: Id of the edge to retrieve.
    ///
    /// # Returns
    /// Edge between specified source and destination with specified id.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    ///
    /// # Panics
    /// * If vertex with either id: `src_id` or `dst_id` does not exist in the storage.
    /// * If there is no edge with id: `edge_id` from source to destination.    
    fn edge_between_unchecked(&self, src_id: usize, dst_id: usize, edge_id: usize) -> &E {
        self.edges_between_unchecked(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
            .unwrap()
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
    /// # Panics
    /// O(1)
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        Ok(!self.get(src_id, dst_id)?.is_empty())
    }

    /// # Arguments
    /// * `src_id`: Id of the source vertex.
    /// * `dst_id`: Id of the destination vertex.
    ///
    /// # Returns
    /// `true` if there is any edge between specified source and destination, `false` otherwise.
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// * If vertex with either id: `src_id` or `dst_id` does not exist in the storage.
    fn has_any_edge_unchecked(&self, src_id: usize, dst_id: usize) -> bool {
        !self[(src_id, dst_id)].is_empty()
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
    ///
    /// # Complexity
    /// O(|E|)
    ///
    /// # Panics
    /// If there is no edge with id: `edge_id` stored in the storage.
    fn edge_unchecked(&self, edge_id: usize) -> &E {
        self.edges()
            .into_iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .map(|(_, _, edge)| edge)
            .unwrap()
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
        edge_id < self.max_edge_id && !self.reusable_edge_ids.contains(&edge_id)
    }
}

use std::ops::{Index, IndexMut};
impl<W: Any, E: Edge<W>, Dir: EdgeDir> Index<(usize, usize)> for AdjMatrix<W, E, Dir> {
    type Output = Vec<E>;

    /// # Arguments
    /// * (`src_id`, `dst_id`): (Id of the source vertex, Id of the destination vertex).
    ///
    /// # Returns
    /// Edges from vertex with id: `src_id` to vertex with id: `dst_id`.
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` are not stored in the storage.
    fn index(&self, (src_id, dst_id): (usize, usize)) -> &Self::Output {
        if !self.contains_vertex(src_id) {
            panic!("Vertex with id: {} does not exist", src_id)
        } else if !self.contains_vertex(dst_id) {
            panic!("Vertex with id: {} does not exist", dst_id)
        } else {
            let index = utils::from_ij(src_id, dst_id, self.is_directed());

            &self.vec[index]
        }
    }
}

impl<W: Any, E: Edge<W>, Dir: EdgeDir> IndexMut<(usize, usize)> for AdjMatrix<W, E, Dir> {
    /// # Arguments
    /// * (`src_id`, `dst_id`): (Id of the source vertex, Id of the destination vertex).
    ///
    /// # Returns
    /// Edges from vertex with id: `src_id` to vertex with id: `dst_id`.
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not stored in the storage.
    fn index_mut(&mut self, (src_id, dst_id): (usize, usize)) -> &mut Self::Output {
        if !self.contains_vertex(src_id) {
            panic!("Vertex with id: {} does not exist", src_id)
        } else if !self.contains_vertex(dst_id) {
            panic!("Vertex with id: {} does not exist", dst_id)
        } else {
            let index = utils::from_ij(src_id, dst_id, self.is_directed());

            &mut self.vec[index]
        }
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge_unchecked(src_id, dst_id)));
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge_unchecked(src_id, dst_id)));
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
        matrix.remove_vertex_unchecked(a);
        matrix.remove_vertex_unchecked(b);

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
        assert!(!matrix.has_any_edge_unchecked(c, c));
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
        matrix.remove_vertex_unchecked(a);
        matrix.remove_vertex_unchecked(b);

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

        assert!(!matrix.has_any_edge_unchecked(c, c));
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
        matrix.remove_vertex_unchecked(a);
        matrix.remove_vertex_unchecked(b);
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge_unchecked(src_id, dst_id)));
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
        matrix.remove_vertex_unchecked(a);
        matrix.remove_vertex_unchecked(b);
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge_unchecked(src_id, dst_id)));
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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge_unchecked(a, b));
        assert!(matrix.has_any_edge_unchecked(b, c));
        assert!(matrix.has_any_edge_unchecked(c, a));
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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge_unchecked(a, b));
        assert!(matrix.has_any_edge_unchecked(b, a));

        assert!(matrix.has_any_edge_unchecked(b, c));
        assert!(matrix.has_any_edge_unchecked(c, b));

        assert!(matrix.has_any_edge_unchecked(c, a));
        assert!(matrix.has_any_edge_unchecked(a, c));
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
        let ab = matrix.add_edge_unchecked(a, b, 1.into());
        let bc = matrix.add_edge_unchecked(b, c, 2.into());
        let ca = matrix.add_edge_unchecked(c, a, 3.into());

        // When: Incrementing weight of each edge by 1.
        matrix.update_edge_unchecked(a, b, ab, 2.into());
        matrix.update_edge_unchecked(b, c, bc, 3.into());
        matrix.update_edge_unchecked(c, a, ca, 4.into());

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
        let ab = matrix.add_edge_unchecked(a, b, 1.into());
        let bc = matrix.add_edge_unchecked(b, c, 2.into());
        let ca = matrix.add_edge_unchecked(c, a, 3.into());

        // When: Incrementing weight of each edge by 1.
        matrix.update_edge_unchecked(a, b, ab, 2.into());
        matrix.update_edge_unchecked(b, c, bc, 3.into());
        matrix.update_edge_unchecked(c, a, ca, 4.into());

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
        let ab = matrix.add_edge_unchecked(a, b, 1.into());
        let bc = matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        matrix.remove_edge_unchecked(a, b, ab);
        matrix.remove_edge_unchecked(b, c, bc);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(
            matrix.edges_between_unchecked(c, a)[0]
                .get_weight()
                .unwrap(),
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
        let ab = matrix.add_edge_unchecked(a, b, 1.into());
        let bc = matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        matrix.remove_edge_unchecked(a, b, ab);
        matrix.remove_edge_unchecked(b, c, bc);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(
            matrix.edges_between_unchecked(a, c)[0]
                .get_weight()
                .unwrap(),
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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.neighbors_unchecked(a).len(), 1);
        assert_eq!(*matrix.neighbors_unchecked(a).get(0).unwrap(), b);

        assert_eq!(matrix.neighbors_unchecked(b).len(), 1);
        assert_eq!(*matrix.neighbors_unchecked(b).get(0).unwrap(), c);

        assert_eq!(matrix.neighbors_unchecked(c).len(), 1);
        assert_eq!(*matrix.neighbors_unchecked(c).get(0).unwrap(), a);
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
        matrix.add_edge_unchecked(a, b, 1.into());
        matrix.add_edge_unchecked(b, c, 2.into());
        matrix.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.neighbors_unchecked(a).len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| matrix.neighbors_unchecked(a).contains(vertex_id)));

        assert_eq!(matrix.neighbors_unchecked(b).len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| matrix.neighbors_unchecked(b).contains(vertex_id)));

        assert_eq!(matrix.neighbors_unchecked(c).len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.neighbors_unchecked(c).contains(vertex_id)));
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
        let ab = mat.add_edge_unchecked(a, b, 1.into());
        let bc = mat.add_edge_unchecked(b, c, 1.into());
        let ca = mat.add_edge_unchecked(c, a, 1.into());

        // Then: it must have 3 edges.
        assert_eq!(mat.edge_count(), 3);

        // When removing the 3 edges.
        mat.remove_edge_unchecked(a, b, ab);
        mat.remove_edge_unchecked(b, c, bc);
        mat.remove_edge_unchecked(c, a, ca);

        // Then: it must have zero edges again.
        assert_eq!(mat.edge_count(), 0);

        // Directed
        // Given: an empty matrix.
        let mut di_mat = DiMat::<usize>::init();

        // When: adding 3 edges.
        let a = di_mat.add_vertex();
        let b = di_mat.add_vertex();
        let c = di_mat.add_vertex();
        let ab = di_mat.add_edge_unchecked(a, b, 1.into());
        let bc = di_mat.add_edge_unchecked(b, c, 1.into());
        let ca = di_mat.add_edge_unchecked(c, a, 1.into());

        // Then: it must have 3 edges.
        assert_eq!(di_mat.edge_count(), 3);

        // When: removing the 3 edges.
        di_mat.remove_edge_unchecked(a, b, ab);
        di_mat.remove_edge_unchecked(b, c, bc);
        di_mat.remove_edge_unchecked(c, a, ca);

        // Then: it must have zero edges again.
        assert_eq!(di_mat.edge_count(), 0);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let mut mat = DiMat::<usize>::init();
        mat.add_vertex();
        mat.add_vertex();

        mat.remove_vertex_unchecked(0);

        &mat[(0, 0)];
    }
}
