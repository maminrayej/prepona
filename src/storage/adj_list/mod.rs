use std::collections::HashSet;
use std::marker::PhantomData;

use anyhow::Result;
use quickcheck::Arbitrary;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeDir, FlowEdge, UndirectedEdge};
use crate::storage::{Error, GraphStorage};

/// An adjacency list that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type List<W, Dir = UndirectedEdge> = AdjList<W, DefaultEdge<W>, Dir>;

/// An adjacency list that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type DiList<W> = AdjList<W, DefaultEdge<W>, DirectedEdge>;

/// An adjacency list that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type FlowList<W, Dir = UndirectedEdge> = AdjList<W, FlowEdge<W>, Dir>;

/// An adjacency list that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type DiFlowList<W> = AdjList<W, FlowEdge<W>, DirectedEdge>;

/// `AdjList` is a collection of unordered lists used to represent a finite graph. Each list describes the set of neighbors of a vertex in the graph.
///
/// ## Note
/// From now on
/// * |V|: Means total number of vertices that are stored in the storage.
/// Note that this is different from number of vertices that are present in the graph.
/// Because even if you remove a vertex from storage, the allocated memory for that vertex will not get freed and will be reused again when adding a new vertex.
/// You can retrieve the amount of |V| using `total_vertex_count` function(as opposed to number of vertices present in the graph which can be retrieved using `vertex_count` function).
/// * |E|: Means number of edges present in the graph.
/// * |E<sub>out</sub>|: Means number of edges exiting a vertex(out degree of the vertex).
///
/// ## Space complexity
/// Space complexity of `AdjList` depends on wether `Dir` is [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge). \
/// * **Directed**: For directed graphs `AdjList` stores |V| + |E| elements.
/// * **Undirected**: For undirected graphs `AdjList` stores each edge twice so it stores |V| + 2*|E| elements.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
pub struct AdjList<W, E: Edge<W>, Dir: EdgeDir = UndirectedEdge> {
    edges_of: Vec<Vec<(usize, E)>>,
    reusable_vertex_ids: HashSet<usize>,

    max_edge_id: usize,
    reusable_edge_ids: HashSet<usize>,

    vertex_count: usize,

    phantom_w: PhantomData<W>,
    phantom_dir: PhantomData<Dir>,
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> AdjList<W, E, Dir> {
    /// Initializes an empty adjacency list.
    ///
    /// `AdjList` defines multiple types with different combination of values for generic parameters.
    /// These types are:
    /// * [`List`](crate::storage::List): An adjacency list that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    ///
    /// * [`DiList`](crate::storage::DiList): An adjacency list that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    ///
    /// * [`FlowList`](crate::storage::FlowList): An adjacency list that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    ///
    /// * [`DiFlowList`](crate::storage::DiFlowList): An adjacency list that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    ///
    /// # Returns
    /// An empty `AdjList`.
    ///
    /// # Examples
    /// ```
    /// use prepona::prelude::*;
    /// use prepona::storage::{List, DiList};
    ///
    /// // To store an undirected graph with usize weights
    /// let list = List::<usize>::init();
    ///
    /// // To store a directed graph with usize weights
    /// let di_list = DiList::<usize>::init();
    /// ```
    pub fn init() -> Self {
        AdjList {
            edges_of: vec![],
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
        self.edges_of.len()
    }

    ////////// Checked "Vector of Edges" Retrieval Functions //////////
    // `edges_of` and `edges_of_mut` are "checked" functions to retrieve vector of edges from a source vertex.
    // It means that if you use `edges_of(vertex_id)` and vertex_id is invalid, It causes the function to return an `Error`.
    //////////////////////////////////////////////

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // * `Ok`: Containing vector of edges that start from vertex with id: `vertex_id` in the format of: (`dst_id`, `edge`).
    // * `Error`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if `vertex_id` is not a valid vertex id.
    fn edges_of(&self, vertex_id: usize) -> Result<&Vec<(usize, E)>> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            Ok(&self.edges_of[vertex_id])
        }
    }

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // * `Ok`: Containing vector of edges that start from vertex with id: `vertex_id` in the format of: (`dst_id`, `edge`).
    // * `Error`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if `vertex_id` is not a valid vertex id.
    fn edges_of_mut(&mut self, vertex_id: usize) -> Result<&mut Vec<(usize, E)>> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        } else {
            Ok(&mut self.edges_of[vertex_id])
        }
    }

    ////////// Unsafe "Vector of Edges" Retrieval Functions //////////
    // `edges_of_unsafe` and `edges_of_mut_unsafe` are "unsafe" alternatives for `edges_of` and `edges_of_mut` functions.
    // It means that if you use `edges_of_unsafe(vertex_id)` it causes the function to return a vector that belongs to a "reusable" vertex id if vertex_id is in 0..|V| range and, panics otherwise.
    // So it does not always panic. The returned vector is invalid and mutating it may leave the storage in an inconsistent state.
    // The only place that you should use them is when you are absolutely sure `vertex_id` is valid.
    // One good example is the implementation of `remove_vertex`:
    //
    //    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
    //        self.edges_of_mut(vertex_id)?.clear();
    //
    //        for src_id in self.vertices() {
    //            self.edges_of_mut_unsafe(src_id).retain(|(dst_id, _)| *dst_id != vertex_id)
    //        }
    //
    //        self.vertex_count -= 1;
    //
    //        self.reusable_vertex_ids.insert(vertex_id);
    //
    //        Ok(())
    //    }
    // As you can see `src_id` comes from `vertices()` so it's always valid.
    // Calling self.edges_of_mut(src_id) for every src_id in for loop is not optimal and is not even needed. Therefore calling edges_of_mut_unsafe is reasonable and leads to better performance.
    //
    // ** Note **: Be sure to document why your usage is reasonable whenever you use `edges_of_unsafe` or `edges_of_mut_unsafe`.
    //////////////////////////////////////////////

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // Containing vector of edges that start from vertex with id: `vertex_id` in the format of: (`dst_id`, `edge`).
    //
    // # Panics
    // If `vertex_id` is not in range 0..|V|.
    fn edges_of_unsafe(&self, vertex_id: usize) -> &Vec<(usize, E)> {
        &self.edges_of[vertex_id]
    }

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // Containing vector of edges that start from vertex with id: `vertex_id` in the format of: (`dst_id`, `edge`).
    //
    // # Panics
    // If `vertex_id` is not in range 0..|V|.
    fn edges_of_mut_unsafe(&mut self, vertex_id: usize) -> &mut Vec<(usize, E)> {
        &mut self.edges_of[vertex_id]
    }
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjList<W, E, Dir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// If number of vertices exceeds the maximum value that `usize` can represent.
    fn add_vertex(&mut self) -> usize {
        self.vertex_count += 1;

        if let Some(reusable_id) = self.next_reusable_vertex_id() {
            reusable_id
        } else {
            self.edges_of.push(vec![]);

            self.edges_of.len() - 1
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
    /// O(|V| + |E|)
    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        self.edges_of_mut(vertex_id)?.clear();

        for src_id in self.vertices() {
            // `src_id` comes from `vertices()` so it's always valid.
            // So calling `edges_of_mut_unsafe` is reasonable.
            self.edges_of_mut_unsafe(src_id)
                .retain(|(dst_id, _)| *dst_id != vertex_id)
        }

        self.vertex_count -= 1;

        self.reusable_vertex_ids.insert(vertex_id);

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

        self.edges_of_mut(src_id)?.push((dst_id, edge.clone()));

        if self.is_undirected() {
            self.edges_of_mut(dst_id)?.push((src_id, edge));
        }

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
    /// * If graph is directed: O(|E<sub>out</sub> of src|).
    /// * If graph is undirected: O(|E<sub>out</sub> of src| + |E<sub>out</sub> of dst|).
    fn update_edge(
        &mut self,
        src_id: usize,
        dst_id: usize,
        edge_id: usize,
        mut edge: E,
    ) -> Result<()> {
        if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        }

        if let Some(index) = self
            .edges_of(src_id)?
            .iter()
            .position(|(d_id, edge)| *d_id == dst_id && edge.get_id() == edge_id)
        {
            edge.set_id(edge_id);

            // `src_id` is valid otherwise `edges_of(src_id)` would have returned error.
            // So calling `edges_of_mut_unsafe` is reasonable.
            self.edges_of_mut_unsafe(src_id)[index] = (dst_id, edge.clone());

            // Also update the edge stored for `dst_id`.
            if self.is_undirected() {
                // `dst_id` is checked to be valid at the start of this function.
                // So calling `edges_of_unsafe` and `edges_of_mut_unsafe` is reasonable.
                let index = self
                    .edges_of_unsafe(dst_id)
                    .iter()
                    .position(|(d_id, edge)| *d_id == src_id && edge.get_id() == edge_id)
                    .unwrap();

                self.edges_of_mut_unsafe(dst_id)[index] = (src_id, edge);
            }

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
    /// * If graph is directed: O(|E<sub>out</sub> of src|).
    /// * If graph is undirected: O(|E<sub>out</sub> of src| + |E<sub>out</sub> of dst|).
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        }

        if let Some(index) = self
            .edges_of(src_id)?
            .iter()
            .position(|(d_id, edge)| *d_id == dst_id && edge.get_id() == edge_id)
        {
            self.reusable_edge_ids.insert(edge_id);

            if self.is_undirected() {
                // `dst_id` is checked to be valid at the start of the functions.
                // So calling `edges_of_mut_unsafe` is reasonable.
                self.edges_of_mut_unsafe(dst_id)
                    .retain(|(_, edge)| edge.get_id() != edge_id);
            }

            // If `src_id` was not valid `self.edges_of(src_id)?` would have returned error already.
            Ok(self.edges_of_mut_unsafe(src_id).remove(index).1)
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
        self.max_edge_id - self.reusable_edge_ids.len()
    }

    /// # Returns
    /// Id of vertices that are present in the storage.
    ///
    /// # Complexity
    /// O(|V|)
    fn vertices(&self) -> Vec<usize> {
        (0..self.edges_of.len())
            .filter(|vertex_id| !self.reusable_vertex_ids.contains(vertex_id))
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
        Ok(self
            .edges_of(src_id)?
            .iter()
            .map(|(dst_id, edge)| (*dst_id, edge))
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
    /// O(|E<sub>out</sub>|)
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .map(|(dst_id, _)| *dst_id)
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
    /// O(|E<sub>out</sub>|)
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .filter_map(|(d_id, edge)| if *d_id == dst_id { Some(edge) } else { None })
            .collect())
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
    /// O(|E<sub>out</sub>|)
    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        self.edges_between(src_id, dst_id)?
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
        let mut edges = vec![];

        for src_id in self.vertices() {
            // `src_id` comes from `vertices()` so it's always valid.
            // So calling `edges_of_unsafe` is reasonable.
            let mut out_going_edges = self
                .edges_of_unsafe(src_id)
                .into_iter()
                .map(|(dst_id, edge)| (src_id, *dst_id, edge))
                .collect();

            edges.append(&mut out_going_edges)
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
    /// O(|E<sub>out</sub>|)
    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .any(|(d_id, _)| *d_id == dst_id))
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
        for vertex_id in self.vertices() {
            // `vertex_id` comes from `vertices()` so it's always valid.
            // So calling `edges_of_unsafe` is reasonable.
            if let Some(edge) = self
                .edges_of_unsafe(vertex_id)
                .into_iter()
                .find(|(_, edge)| edge.get_id() == edge_id)
                .map(|(_, edge)| edge)
            {
                return Ok(edge);
            }
        }

        Err(Error::new_enf(edge_id))?
    }

    /// # Arguments
    /// `vertex_id`: Id of the vertex to search for its existence in the storage.
    ///
    /// # Returns
    /// * `true`: If storage contains the vertex with id: `vertex_id`.
    /// * `false`: Otherwise.
    fn contains_vertex(&self, vertex_id: usize) -> bool {
        vertex_id < self.total_vertex_count() && !self.reusable_vertex_ids.contains(&vertex_id)
    }

    /// # Arguments
    /// `edge_id`: Id of the edge to search for its existence in the storage.
    ///
    /// # Returns
    /// * `true`: If storage contains the edge with id: `edge_id`.
    /// * `false`: Otherwise.
    fn contains_edge(&self, edge_id: usize) -> bool {
        edge_id < self.max_edge_id && !self.reusable_edge_ids.contains(&edge_id)
    }

    fn filter(
        &self,
        vertex_filter: impl Fn(&usize) -> bool,
        edge_filter: impl Fn(&usize, &usize, &E) -> bool,
    ) -> Self {
        let filtered_vertices: Vec<usize> =
            self.vertices().into_iter().filter(vertex_filter).collect();

        let filtered_edges: Vec<(usize, usize, &E)> = self
            .edges()
            .into_iter()
            .filter(|(src_id, dst_id, edge)| {
                filtered_vertices.contains(src_id)
                    && filtered_vertices.contains(dst_id)
                    && edge_filter(src_id, dst_id, edge)
            })
            .collect();

        let mut storage = AdjList::init();

        for _ in &filtered_vertices {
            storage.add_vertex();
        }

        for (src_id, dst_id, edge) in &filtered_edges {
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

impl<W: Clone + 'static, E: Edge<W> + Clone, Dir: EdgeDir> Clone for AdjList<W, E, Dir> {
    fn clone(&self) -> Self {
        AdjList {
            edges_of: self.edges_of.clone(),
            reusable_vertex_ids: self.reusable_vertex_ids.clone(),

            max_edge_id: self.max_edge_id,
            reusable_edge_ids: self.reusable_edge_ids.clone(),

            vertex_count: self.vertex_count,

            phantom_w: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<W: Clone + 'static, E: Edge<W> + Arbitrary, Dir: EdgeDir + 'static> Arbitrary
    for AdjList<W, E, Dir>
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let vertex_count = usize::arbitrary(g);

        let edge_prob = rand::random::<f64>() * rand::random::<f64>();

        let mut storage = AdjList::init();

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
            let graph_partition = graph.filter(|v_id| *v_id % 2 == partition_index, |_, _, _| true);

            if graph_partition.vertex_count() < graph.vertex_count() {
                Some(graph_partition)
            } else {
                None
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_empty_list() {
        // Given: An empty directed list.
        let list = DiList::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.edge_count(), 0);
        assert_eq!(list.vertex_count(), 0);
        assert_eq!(list.edges_of.len(), 0);
        assert_eq!(list.reusable_vertex_ids.len(), 0);
        assert_eq!(list.is_directed(), true);
    }

    #[test]
    fn undirected_empty_list() {
        // Given: An empty undirected list.
        let list = List::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.edge_count(), 0);
        assert_eq!(list.vertex_count(), 0);
        assert_eq!(list.edges_of.len(), 0);
        assert_eq!(list.reusable_vertex_ids.len(), 0);
        assert_eq!(list.is_directed(), false);
    }

    #[test]
    fn directed_add_vertex() {
        // Given: An empty directed list.
        let mut list = DiList::<usize>::init();

        // When: Adding 3 vertices.
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.is_empty()));
        assert_eq!(list.reusable_vertex_ids.len(), 0);

        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !list.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex() {
        // Given: An empty undirected list.
        let mut list = List::<usize>::init();

        // When: Adding 3 vertices.
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert_eq!(list.reusable_vertex_ids.len(), 0);

        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !list.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_delete_vertex() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b.
        list.remove_vertex(a).unwrap();
        list.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 1);
        assert_eq!(list.edges_of.len(), 3);

        // Vertices a and b must be reusable.
        assert_eq!(list.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.reusable_vertex_ids.contains(vertex_id)));

        // list must only contain c.
        assert_eq!(list.vertices().len(), 1);
        assert!(list.vertices().contains(&c));

        assert!(!list.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn undirected_delete_vertex() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b.
        list.remove_vertex(a).unwrap();
        list.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 1);
        assert_eq!(list.edges_of.len(), 3);

        // Vertices a and b must be reusable.
        assert_eq!(list.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.reusable_vertex_ids.contains(vertex_id)));

        // list must only contain c.
        assert_eq!(list.vertices().len(), 1);
        assert!(list.vertices().contains(&c));

        assert!(!list.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn directed_add_vertex_after_vertex_deletion() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        list.remove_vertex(a).unwrap();
        list.remove_vertex(b).unwrap();
        let _ = list.add_vertex();
        let _ = list.add_vertex();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);

        // There must be no reusable id.
        assert_eq!(list.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !list.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex_after_vertex_deletion() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        list.remove_vertex(a).unwrap();
        list.remove_vertex(b).unwrap();
        let _ = list.add_vertex();
        let _ = list.add_vertex();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);

        // There must be no reusable id.
        assert_eq!(list.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !list.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_add_edge() {
        // Given: Directed list
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Adding edges
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 1));

        assert_eq!(list.edges().len(), 3);
        for (src_id, dst_id, edge) in list.edges() {
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
        // Given: Undirected list
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Adding edges
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 2));

        assert_eq!(list.edges().len(), 3);
        for (src_id, dst_id, edge) in list.edges() {
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
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(list.has_any_edge(a, b).unwrap());
        assert!(list.has_any_edge(b, c).unwrap());
        assert!(list.has_any_edge(c, a).unwrap());
    }

    #[test]
    fn undirected_has_edge() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(list.has_any_edge(a, b).unwrap());
        assert!(list.has_any_edge(b, a).unwrap());

        assert!(list.has_any_edge(b, c).unwrap());
        assert!(list.has_any_edge(c, b).unwrap());

        assert!(list.has_any_edge(c, a).unwrap());
        assert!(list.has_any_edge(a, c).unwrap());
    }

    #[test]
    fn directed_update_edge() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        let ab = list.add_edge(a, b, 1.into()).unwrap();
        let bc = list.add_edge(b, c, 2.into()).unwrap();
        let ca = list.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing edge of each edge by 1.
        list.update_edge(a, b, ab, 2.into()).unwrap();
        list.update_edge(b, c, bc, 3.into()).unwrap();
        list.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 1));

        assert_eq!(list.edges().len(), 3);
        for (src_id, dst_id, edge) in list.edges() {
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
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        let ab = list.add_edge(a, b, 1.into()).unwrap();
        let bc = list.add_edge(b, c, 2.into()).unwrap();
        let ca = list.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing edge of each edge by 1.
        list.update_edge(a, b, ab, 2.into()).unwrap();
        list.update_edge(b, c, bc, 3.into()).unwrap();
        list.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 2));

        assert_eq!(list.edges().len(), 3);
        for (src_id, dst_id, edge) in list.edges() {
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
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        let ab = list.add_edge(a, b, 1.into()).unwrap();
        let bc = list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        list.remove_edge(a, b, ab).unwrap();
        list.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of[a].is_empty());
        assert!(list.edges_of[b].is_empty());
        assert_eq!(list.edges_of[c].len(), 1);

        assert_eq!(list.edges().len(), 1);
        assert_eq!(
            list.edges_between(c, a).unwrap()[0].get_weight().unwrap(),
            3
        );
    }

    #[test]
    fn undirected_remove_edge() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        let ab = list.add_edge(a, b, 1.into()).unwrap();
        let bc = list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        list.remove_edge(a, b, ab).unwrap();
        list.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of[a].len(), 1);
        assert!(list.edges_of[b].is_empty());
        assert_eq!(list.edges_of[c].len(), 1);

        assert_eq!(list.edges().len(), 1);
        assert_eq!(
            list.edges_between(a, c).unwrap()[0].get_weight().unwrap(),
            3
        );
    }

    #[test]
    fn directed_neighbors() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 1));

        assert_eq!(list.neighbors(a).unwrap().len(), 1);
        assert_eq!(*list.neighbors(a).unwrap().get(0).unwrap(), b);

        assert_eq!(list.neighbors(b).unwrap().len(), 1);
        assert_eq!(*list.neighbors(b).unwrap().get(0).unwrap(), c);

        assert_eq!(list.neighbors(c).unwrap().len(), 1);
        assert_eq!(*list.neighbors(c).unwrap().get(0).unwrap(), a);
    }

    #[test]
    fn undirected_neighbors() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge(a, b, 1.into()).unwrap();
        list.add_edge(b, c, 2.into()).unwrap();
        list.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 2));

        assert_eq!(list.neighbors(a).unwrap().len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| list.neighbors(a).unwrap().contains(vertex_id)));

        assert_eq!(list.neighbors(b).unwrap().len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| list.neighbors(b).unwrap().contains(vertex_id)));

        assert_eq!(list.neighbors(c).unwrap().len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.neighbors(c).unwrap().contains(vertex_id)));
    }

    #[test]
    fn edge_count() {
        // Undirected
        // Given: an empty matrix.
        let mut mat = List::<usize>::init();

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
        let mut di_mat = DiList::<usize>::init();

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
