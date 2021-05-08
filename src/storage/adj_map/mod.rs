use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::{
    graph::{Edge, EdgeDir, FlowEdge},
    prelude::{DefaultEdge, DirectedEdge, UndirectedEdge},
};

use anyhow::Result;
use quickcheck::Arbitrary;

use super::{Error, GraphStorage};

/// An adjacency map that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type Map<W, Dir = UndirectedEdge> = AdjMap<W, DefaultEdge<W>, Dir>;

/// An adjacency map that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
pub type DiMap<W> = AdjMap<W, DefaultEdge<W>, DirectedEdge>;

/// An adjacency map that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type FlowMap<W, Dir = UndirectedEdge> = AdjMap<W, FlowEdge<W>, Dir>;

/// An adjacency map that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
pub type DiFlowMap<W> = AdjMap<W, FlowEdge<W>, DirectedEdge>;

/// `AdjMap` is a two-layer hash map in the form of: (source id) -> (destination id) -> (list of edges from source to destination).
///
/// ## Note
/// From now on
/// * |V|: Means number of vertices that are stored in the storage.
/// * |E<sub>src->dst</sub>|: Means number of edges from source to destination.
/// * |E|: Means number of edges stored in the storage.
/// * |N|: Means number of neighbors of a vertex.
///
/// ## Space complexity
/// Space complexity of `AdjMap` depends on wether `Dir` is [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge). \
/// * **Directed**: For directed graphs `AdjMap` stores |V| + |E| elements.
/// * **Undirected**: For undirected graphs `AdjMap` stores each edge twice so it stores |V| + 2*|E| elements.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
pub struct AdjMap<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir = UndirectedEdge> {
    map: HashMap<usize, HashMap<usize, Vec<E>>>,

    reusable_vertex_ids: HashSet<usize>,
    reusable_edge_ids: HashSet<usize>,

    vertex_count: usize,
    max_edge_id: usize,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> AdjMap<W, E, Dir> {
    /// Initializes an empty adjacency map.
    ///
    /// `AdjMap` defines multiple types with different combination of values for generic parameters.
    /// These types are:
    /// * [`Map`](crate::storage::Map): An adjacency map that uses [`undirected`](crate::graph::UndirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    ///
    /// * [`DiMap`](crate::storage::DiMap): An adjacency map that uses [`directed`](crate::graph::DirectedEdge) [`default edges`](crate::graph::DefaultEdge).
    ///
    /// * [`FlowMap`](crate::storage::FlowMap): An adjacency map that uses [`undirected`](crate::graph::UndirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    ///
    /// * [`DiFlowMap`](crate::storage::DiFlowMap): An adjacency map that uses [`directed`](crate::graph::DirectedEdge) [`flow edges`](crate::graph::FlowEdge).
    ///
    /// # Returns
    /// An empty `AdjMap`.
    ///
    /// # Examples
    /// ```
    /// use prepona::prelude::*;
    /// use prepona::storage::{Map, DiMap};
    ///
    /// // To store an undirected graph with usize weights
    /// let map = Map::<usize>::init();
    ///
    /// // To store a directed graph with usize weights
    /// let di_map = DiMap::<usize>::init();
    /// ```
    pub fn init() -> Self {
        AdjMap {
            map: HashMap::new(),

            reusable_vertex_ids: HashSet::new(),
            reusable_edge_ids: HashSet::new(),

            vertex_count: 0,
            max_edge_id: 0,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
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

    ////////// Checked "Map of Edges" Retrieval Functions //////////
    // `get_map` and `get_map_mut` are "checked" alternatives to `index` and `index_mut` to retrieve map of edges from source to destination.
    // It means that if you use `get_map(src_id, dst_id)` and any of the two vertex ids are invalid, It causes the function to return an `Error` instead of panicking.
    //////////////////////////////////////////////

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // * `Ok`: Containing hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VectorNotFound) if `vertex_id` is not a valid vertex id.
    fn get_map(&self, vertex_id: usize) -> Result<&HashMap<usize, Vec<E>>> {
        self.map
            .get(&vertex_id)
            .ok_or(Error::new_vnf(vertex_id).into())
    }

    // # Arguments
    // * `vertex_id`: Id of the source vertex.
    //
    // # Returns
    // * `Ok`: Containing hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VectorNotFound) if `vertex_id` is not a valid vertex id.
    fn get_map_mut(&mut self, vertex_id: usize) -> Result<&mut HashMap<usize, Vec<E>>> {
        self.map
            .get_mut(&vertex_id)
            .ok_or(Error::new_vnf(vertex_id).into())
    }

    ////////// Checked "Vector of Edges" Retrieval Functions //////////
    // `get_edges` and `get_edges_mut` are "checked" alternatives to `index` and `index_mut` to retrieve vector of edges from source to destination.
    // It means that if you use `get_edges(src_id, dst_id)` and any of the two vertex ids are invalid, It causes the function to return an `Error` instead of panicking.
    //////////////////////////////////////////////

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // * `Ok`: Containing hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VectorNotFound) if either `src_id` or `dst_id` is not valid.
    // * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
    fn get_edges(&self, src_id: usize, dst_id: usize) -> Result<&Vec<E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            self.map
                .get(&src_id)
                .unwrap()
                .get(&dst_id)
                .ok_or(Error::new_eel(src_id, dst_id).into())
        }
    }

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // * `Ok`: Containing hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    // * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VectorNotFound) if either `src_id` or `dst_id` is not valid.
    // * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
    fn get_edges_mut(&mut self, src_id: usize, dst_id: usize) -> Result<&mut Vec<E>> {
        if !self.contains_vertex(src_id) {
            Err(Error::new_vnf(src_id))?
        } else if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        } else {
            self.map
                .get_mut(&src_id)
                .unwrap()
                .get_mut(&dst_id)
                .ok_or(Error::new_eel(src_id, dst_id).into())
        }
    }
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjMap<W, E, Dir> {
    /// Adds a vertex to the storage.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    ///
    /// # Complexity
    /// O(|1|)
    ///
    /// # Panics
    /// If number of vertices exceeds the maximum value that `usize` can represent.
    fn add_vertex(&mut self) -> usize {
        let vertex_id = if let Some(reusable_id) = self.next_reusable_vertex_id() {
            reusable_id
        } else {
            self.vertex_count
        };

        self.map.insert(vertex_id, HashMap::new());

        self.vertex_count += 1;

        vertex_id
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
        if let Some(_) = self.map.remove(&vertex_id) {
            // `v_id` comes from `vertices()`. So it's always valid.
            // So it's reasonable to use `index_mut`.
            for v_id in self.vertices() {
                self[v_id].remove(&vertex_id);
            }

            self.reusable_vertex_ids.insert(vertex_id);

            self.vertex_count -= 1;

            Ok(())
        } else {
            Err(Error::new_vnf(vertex_id))?
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
    /// * `Ok`: Containing unique id of the newly added edge.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    ///
    /// # Complexity
    /// O(1)
    fn add_edge(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> Result<usize> {
        if !self.contains_vertex(dst_id) {
            Err(Error::new_vnf(dst_id))?
        }

        let edge_id = if let Some(id) = self.next_reusable_edge_id() {
            id
        } else {
            self.max_edge_id += 1;

            self.max_edge_id - 1
        };

        edge.set_id(edge_id);

        self.get_map_mut(src_id)?
            .entry(dst_id)
            .or_insert(vec![])
            .push(edge.clone());

        if self.is_undirected() && src_id != dst_id {
            // `dst_id` is checked to be valid at the start of this function.
            // So it's reasonable to use `index_mut`.
            self[dst_id].entry(src_id).or_insert(vec![]).push(edge);
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
    /// * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
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
        let edges_vec = self.get_edges_mut(src_id, dst_id)?;

        if let Some(index) = edges_vec.iter().position(|e| e.get_id() == edge_id) {
            edge.set_id(edge_id);

            edges_vec[index] = edge.clone();

            if self.is_undirected() {
                // `dst_id` and `src_id` are both validated at the start of this function.
                // And we are sure we won't encounter EmptyEdgeList because that is also validated at the start of this function.
                // So it's reasonable to use `index_mut`.
                let edges_vec = &mut self[(dst_id, src_id)];

                let index = edges_vec
                    .iter()
                    .position(|e| e.get_id() == edge_id)
                    .unwrap();

                edges_vec[index] = edge;
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
    /// * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        let edges_vec = self.get_edges_mut(src_id, dst_id)?;

        if let Some(index) = edges_vec.iter().position(|e| e.get_id() == edge_id) {
            let edge = edges_vec.swap_remove(index);

            self.reusable_edge_ids.insert(edge_id);

            if self.is_undirected() {
                // `dst_id` and `src_id` are both validated at the start of this function.
                // And we are sure we won't encounter EmptyEdgeList because that is also validated at the start of this function.
                // So it's reasonable to use `index_mut`.
                self[(dst_id, src_id)].retain(|e| e.get_id() != edge_id);
            }

            Ok(edge)
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
        self.map.keys().copied().collect()
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
            .get_map(src_id)?
            .iter()
            .flat_map(|(dst_id, edges)| {
                edges
                    .into_iter()
                    .map(|edge| (*dst_id, edge))
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
    /// O(|N|)   
    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        Ok(self
            .get_map(src_id)?
            .into_iter()
            .filter_map(|(dst_id, edges)| {
                if !edges.is_empty() {
                    Some(*dst_id)
                } else {
                    None
                }
            })
            .collect())
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// * `Ok`: Edges from source vertex to destination vertex.
    /// * `Err`: [`VertexNotFound`](crate::storage::ErrorKind::VertexNotFound) if vertex with either id: `src_id` or `dst_id` does not exist.
    /// * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        Ok(self.get_edges(src_id, dst_id)?.into_iter().collect())
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
    /// * `Err`: [`EmptyEdgeList`](crate::storage::ErrorKind::EmptyEdgeList) if there is no edge from source to destination.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
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

        for src_id in self.map.keys() {
            // `src_id` is a valid key so using self[*src_id] is reasonable.
            for (dst_id, src_to_dst_edges) in self[*src_id].iter() {
                edges.append(
                    &mut src_to_dst_edges
                        .into_iter()
                        .map(|edge| (*src_id, *dst_id, edge))
                        .collect(),
                )
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
        Ok(self.get_map(src_id)?.contains_key(&dst_id))
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
        self.as_directed_edges()
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
    fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.map.contains_key(&vertex_id)
    }

    /// # Arguments
    /// `edge_id`: Id of the edge to search for its existence in the storage.
    ///
    /// # Returns
    /// * `true`: If storage contains the edge with id: `edge_id`.
    /// * `false`: Otherwise.
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
        let mut storage = AdjMap::init();

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

impl<W: Clone + 'static, E: Edge<W> + Clone, Dir: EdgeDir> Clone for AdjMap<W, E, Dir> {
    fn clone(&self) -> Self {
        AdjMap {
            map: self.map.clone(),

            reusable_vertex_ids: self.reusable_vertex_ids.clone(),
            reusable_edge_ids: self.reusable_edge_ids.clone(),

            vertex_count: self.vertex_count,
            max_edge_id: self.max_edge_id,

            phantom_w: PhantomData,
            phantom_e: PhantomData,
            phantom_dir: PhantomData,
        }
    }
}

impl<W: Clone + 'static, E: Edge<W> + Arbitrary, Dir: EdgeDir + 'static> Arbitrary
    for AdjMap<W, E, Dir>
{
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let vertex_count = usize::arbitrary(g).clamp(0, 32);

        let edge_prob = rand::random::<f64>() * rand::random::<f64>();

        let mut storage = AdjMap::init();

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

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> Index<(usize, usize)> for AdjMap<W, E, Dir> {
    type Output = Vec<E>;

    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // Hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    //
    // # Panics
    // * If `src_id` or `dst_id` are not valid vertex ids.
    // * If there is no edge from source to destination.
    fn index(&self, (src_id, dst_id): (usize, usize)) -> &Self::Output {
        &self.map[&src_id][&dst_id]
    }
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> IndexMut<(usize, usize)> for AdjMap<W, E, Dir> {
    // # Arguments
    // * `src_id`: Id of the source vertex.
    // * `dst_id`: Id of the destination vertex.
    //
    // # Returns
    // Hash map mapping each `dst_id` to vector of edges from `vertex_id` to `dst_id`.
    //
    // # Panics
    // * If `src_id` or `dst_id` are not valid vertex ids.
    // * If there is no edge from source to destination.
    fn index_mut(&mut self, (src_id, dst_id): (usize, usize)) -> &mut Self::Output {
        self.map.get_mut(&src_id).unwrap().get_mut(&dst_id).unwrap()
    }
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> Index<usize> for AdjMap<W, E, Dir> {
    type Output = HashMap<usize, Vec<E>>;

    // # Arguments
    // * `src_id`: Id of the source vertex.
    //
    // # Returns
    // Hash map mapping each `dst_id` to vector of edges from `src_id` to `dst_id`.
    //
    // # Panics
    // If `src_id` is not a valid vertex_id.
    fn index(&self, src_id: usize) -> &Self::Output {
        &self.map[&src_id]
    }
}

impl<W: Clone, E: Edge<W> + Clone, Dir: EdgeDir> IndexMut<usize> for AdjMap<W, E, Dir> {
    // # Arguments
    // * `src_id`: Id of the source vertex.
    //
    // # Returns
    // Hash map mapping each `dst_id` to vector of edges from `src_id` to `dst_id`.
    //
    // # Panics
    // If `src_id` is not a valid vertex_id.
    fn index_mut(&mut self, src_id: usize) -> &mut Self::Output {
        self.map.get_mut(&src_id).unwrap()
    }
}

impl<W: Clone + Debug, E: Edge<W> + Clone + Debug, Dir: EdgeDir> Debug for AdjMap<W, E, Dir> {
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
    fn directed_empty_map() {
        // Given: An empty directed map.
        let map = DiMap::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(map.edge_count(), 0);
        assert_eq!(map.vertex_count(), 0);
        assert_eq!(map.reusable_vertex_ids.len(), 0);
        assert_eq!(map.is_directed(), true);
    }

    #[test]
    fn undirected_empty_map() {
        // Given: An empty undirected map.
        let map = Map::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(map.edge_count(), 0);
        assert_eq!(map.vertex_count(), 0);
        assert_eq!(map.reusable_vertex_ids.len(), 0);
        assert_eq!(map.is_directed(), false);
    }

    #[test]
    fn directed_add_vertex() {
        // Given: An empty directed map.
        let mut map = DiMap::<usize>::init();

        // When: Adding 3 vertices.
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // Then:
        assert_eq!(map.vertex_count(), 3);
        assert_eq!(map.reusable_vertex_ids.len(), 0);

        assert_eq!(map.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| map.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !map.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex() {
        // Given: An empty undirected map.
        let mut map = Map::<usize>::init();

        // When: Adding 3 vertices.
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // Then:
        assert_eq!(map.vertex_count(), 3);
        assert_eq!(map.reusable_vertex_ids.len(), 0);

        assert_eq!(map.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| map.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !map.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_delete_vertex() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Removing vertices a and b.
        map.remove_vertex(a).unwrap();
        map.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 1);

        // Vertices a and b must be reusable.
        assert_eq!(map.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| map.reusable_vertex_ids.contains(vertex_id)));

        // map must only contain c.
        assert_eq!(map.vertices().len(), 1);
        assert!(map.vertices().contains(&c));

        assert!(!map.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn undirected_delete_vertex() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Removing vertices a and b.
        map.remove_vertex(a).unwrap();
        map.remove_vertex(b).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 1);

        // Vertices a and b must be reusable.
        assert_eq!(map.reusable_vertex_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| map.reusable_vertex_ids.contains(vertex_id)));

        // map must only contain c.
        assert_eq!(map.vertices().len(), 1);
        assert!(map.vertices().contains(&c));

        assert!(!map.has_any_edge(c, c).unwrap());
    }

    #[test]
    fn directed_add_vertex_after_vertex_deletion() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        map.remove_vertex(a).unwrap();
        map.remove_vertex(b).unwrap();
        let _ = map.add_vertex();
        let _ = map.add_vertex();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        // There must be no reusable id.
        assert_eq!(map.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(map.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| map.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !map.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn undirected_add_vertex_after_vertex_deletion() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        map.remove_vertex(a).unwrap();
        map.remove_vertex(b).unwrap();
        let _ = map.add_vertex();
        let _ = map.add_vertex();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        // There must be no reusable id.
        assert_eq!(map.reusable_vertex_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(map.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| map.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| !map.has_any_edge(src_id, dst_id).unwrap()));
    }

    #[test]
    fn directed_add_edge() {
        // Given: Directed map
        //
        //      a   b   c
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Adding edges
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.edges().len(), 3);
        for (src_id, dst_id, edge) in map.edges() {
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
        // Given: Undirected map
        //
        //      a   b   c
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        // When: Adding edges
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.edges().len(), 3);
        for (src_id, dst_id, edge) in map.edges() {
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
        // Given: Directed map
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(map.has_any_edge(a, b).unwrap());
        assert!(map.has_any_edge(b, c).unwrap());
        assert!(map.has_any_edge(c, a).unwrap());
    }

    #[test]
    fn undirected_has_edge() {
        // Given: Undirected map
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert!(map.has_any_edge(a, b).unwrap());
        assert!(map.has_any_edge(b, a).unwrap());

        assert!(map.has_any_edge(b, c).unwrap());
        assert!(map.has_any_edge(c, b).unwrap());

        assert!(map.has_any_edge(c, a).unwrap());
        assert!(map.has_any_edge(a, c).unwrap());
    }

    #[test]
    fn directed_update_edge() {
        // Given: Directed map
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        let ab = map.add_edge(a, b, 1.into()).unwrap();
        let bc = map.add_edge(b, c, 2.into()).unwrap();
        let ca = map.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing edge of each edge by 1.
        map.update_edge(a, b, ab, 2.into()).unwrap();
        map.update_edge(b, c, bc, 3.into()).unwrap();
        map.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.edges().len(), 3);
        for (src_id, dst_id, edge) in map.edges() {
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
        // Given: Undirected map
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();

        let ab = map.add_edge(a, b, 1.into()).unwrap();
        let bc = map.add_edge(b, c, 2.into()).unwrap();
        let ca = map.add_edge(c, a, 3.into()).unwrap();

        // When: Incrementing edge of each edge by 1.
        map.update_edge(a, b, ab, 2.into()).unwrap();
        map.update_edge(b, c, bc, 3.into()).unwrap();
        map.update_edge(c, a, ca, 4.into()).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.edges().len(), 3);
        for (src_id, dst_id, edge) in map.edges() {
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
        // Given: Directed map
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        let ab = map.add_edge(a, b, 1.into()).unwrap();
        let bc = map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        map.remove_edge(a, b, ab).unwrap();
        map.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);
        assert!(map.edges_from(a).unwrap().is_empty());
        assert!(map.edges_from(b).unwrap().is_empty());
        assert_eq!(map[c].len(), 1);

        assert_eq!(map.edges().len(), 1);
        assert_eq!(map.edges_between(c, a).unwrap()[0].get_weight().unwrap(), 3);
    }

    #[test]
    fn undirected_remove_edge() {
        // Given: Undirected map
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        let ab = map.add_edge(a, b, 1.into()).unwrap();
        let bc = map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        map.remove_edge(a, b, ab).unwrap();
        map.remove_edge(b, c, bc).unwrap();

        // Then:
        assert_eq!(map.vertex_count(), 3);
        assert_eq!(map.edges_from(a).unwrap().len(), 1);
        assert_eq!(map.edges_from(b).unwrap().len(), 0);
        assert_eq!(map.edges_from(c).unwrap().len(), 1);

        assert_eq!(map.edges().len(), 1);
        assert_eq!(map.edges_between(a, c).unwrap()[0].get_weight().unwrap(), 3);
    }

    #[test]
    fn directed_neighbors() {
        // Given: Directed map
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut map = DiMap::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.neighbors(a).unwrap().len(), 1);
        assert_eq!(*map.neighbors(a).unwrap().get(0).unwrap(), b);

        assert_eq!(map.neighbors(b).unwrap().len(), 1);
        assert_eq!(*map.neighbors(b).unwrap().get(0).unwrap(), c);

        assert_eq!(map.neighbors(c).unwrap().len(), 1);
        assert_eq!(*map.neighbors(c).unwrap().get(0).unwrap(), a);
    }

    #[test]
    fn undirected_neighbors() {
        // Given: Undirected map
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut map = Map::<usize>::init();
        let a = map.add_vertex();
        let b = map.add_vertex();
        let c = map.add_vertex();
        map.add_edge(a, b, 1.into()).unwrap();
        map.add_edge(b, c, 2.into()).unwrap();
        map.add_edge(c, a, 3.into()).unwrap();

        // When: Doing nothing.

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.neighbors(a).unwrap().len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| map.neighbors(a).unwrap().contains(vertex_id)));

        assert_eq!(map.neighbors(b).unwrap().len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| map.neighbors(b).unwrap().contains(vertex_id)));

        assert_eq!(map.neighbors(c).unwrap().len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| map.neighbors(c).unwrap().contains(vertex_id)));
    }

    #[test]
    fn edge_count() {
        // Undirected
        // Given: an empty matrix.
        let mut mat = Map::<usize>::init();

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
        let mut di_mat = DiMap::<usize>::init();

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
