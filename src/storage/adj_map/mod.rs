use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::{
    graph::{Edge, EdgeDir, FlowEdge},
    prelude::{DefaultEdge, DirectedEdge, UndirectedEdge},
};

use super::GraphStorage;

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
///
pub struct AdjMap<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir = UndirectedEdge> {
    map: HashMap<usize, HashMap<usize, Vec<E>>>,

    reusable_vertex_ids: HashSet<usize>,
    reusable_edge_ids: HashSet<usize>,

    vertex_count: usize,
    max_edge_id: usize,

    phantom_w: PhantomData<W>,
    phantom_e: PhantomData<E>,
    phantom_dir: PhantomData<Dir>,
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> AdjMap<W, E, Dir> {
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
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjMap<W, E, Dir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    ///
    /// # Complexity
    /// O(|1|)
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_vertex_id() {
            self.vertex_count += 1;

            self.map.insert(reusable_id, HashMap::new());

            reusable_id
        } else {
            let vertex_id = self.vertex_count;

            self.map.insert(vertex_id, HashMap::new());

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
    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        self.map.remove(&vertex_id);

        for v_id in self.vertices() {
            self[v_id].remove(&vertex_id);
        }

        self.reusable_vertex_ids.insert(vertex_id);

        self.vertex_count -= 1;
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
    ///
    /// # Complexity
    /// O(1)
    ///
    /// # Panics
    /// If `src_id` or `dst_id` is not in 0..|V| range.
    fn add_edge_unchecked(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> usize {
        let edge_id = if let Some(id) = self.next_reusable_edge_id() {
            id
        } else {
            self.max_edge_id += 1;

            self.max_edge_id - 1
        };

        edge.set_id(edge_id);

        self[src_id].entry(dst_id).or_insert(vec![]).push(edge);

        if Dir::is_undirected() {
            self[dst_id].entry(src_id).or_insert(vec![]).push(edge);
        }

        edge_id
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
    /// O(|E|)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    /// * If there is no edge with id: `edge_id` from `src_id` to `dst_id`.
    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        let removed_edge = self.remove_edge_unchecked(src_id, dst_id, edge_id);

        edge.set_id(removed_edge.get_id());

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
    /// * `Some`: Containing the removed edge.
    /// * `None`: If edge with `edge_id` does not exist in the graph.
    ///
    /// # Complexity
    /// O(|E<sub>src->dst</sub>|)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    /// * If there is no edge with id: `edge_id` from `src_id` to `dst_id`.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E {
        let edge_vec = &mut self[(src_id, dst_id)];

        let index = edge_vec.iter().position(|e| e.get_id() == edge_id).unwrap();

        let edge = edge_vec.swap_remove(index);

        if edge_vec.is_empty() {
            self[src_id].remove(&dst_id);
        }

        self.reusable_edge_ids.insert(edge_id);

        if Dir::is_undirected() {
            let edge_vec = &mut self[(dst_id, src_id)];
            edge_vec.retain(|e| e.get_id() != edge_id);

            if edge_vec.is_empty() {
                self[dst_id].remove(&src_id);
            }
        }

        edge
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
        self.map.keys().copied().collect()
    }

    /// # Arguments
    /// `src_id`: Id of the source vertex.
    ///
    /// # Returns
    /// * All edges from the source vertex in the format of: (`dst_id`, `edge`)
    ///
    /// # Complexity
    /// O(|E|)
    ///
    /// # Panics
    /// If `src_id` is not in range 0..|V|.
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        let edge_map = &self.map[&src_id];

        let edges = edge_map
            .iter()
            .flat_map(|(dst_id, edge_vec)| {
                edge_vec
                    .iter()
                    .map(|edge| (*dst_id, edge))
                    .collect::<Vec<(usize, &E)>>()
            })
            .collect::<Vec<(usize, &E)>>();

        edges
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
        edge_id < self.max_edge_id && !self.reusable_edge_ids.contains(&edge_id)
    }
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> Index<(usize, usize)> for AdjMap<W, E, Dir> {
    type Output = Vec<E>;

    fn index(&self, (src_id, dst_id): (usize, usize)) -> &Self::Output {
        &self.map[&src_id][&dst_id]
    }
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> IndexMut<(usize, usize)> for AdjMap<W, E, Dir> {
    fn index_mut(&mut self, (src_id, dst_id): (usize, usize)) -> &mut Self::Output {
        self.map.get_mut(&src_id).unwrap().get_mut(&dst_id).unwrap()
    }
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> Index<usize> for AdjMap<W, E, Dir> {
    type Output = HashMap<usize, Vec<E>>;

    fn index(&self, src_id: usize) -> &Self::Output {
        &self.map[&src_id]
    }
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> IndexMut<usize> for AdjMap<W, E, Dir> {
    fn index_mut(&mut self, src_id: usize) -> &mut Self::Output {
        self.map.get_mut(&src_id).unwrap()
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
            .all(|(src_id, dst_id)| !map.has_any_edge_unchecked(src_id, dst_id)));
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
            .all(|(src_id, dst_id)| !map.has_any_edge_unchecked(src_id, dst_id)));
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
        map.remove_vertex_unchecked(a);
        map.remove_vertex_unchecked(b);

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

        assert!(!map.has_any_edge_unchecked(c, c));
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
        map.remove_vertex_unchecked(a);
        map.remove_vertex_unchecked(b);

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

        assert!(!map.has_any_edge_unchecked(c, c));
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
        map.remove_vertex_unchecked(a);
        map.remove_vertex_unchecked(b);
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
            .all(|(src_id, dst_id)| !map.has_any_edge_unchecked(src_id, dst_id)));
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
        map.remove_vertex_unchecked(a);
        map.remove_vertex_unchecked(b);
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
            .all(|(src_id, dst_id)| !map.has_any_edge_unchecked(src_id, dst_id)));
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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(map.has_any_edge_unchecked(a, b));
        assert!(map.has_any_edge_unchecked(b, c));
        assert!(map.has_any_edge_unchecked(c, a));
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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(map.has_any_edge_unchecked(a, b));
        assert!(map.has_any_edge_unchecked(b, a));

        assert!(map.has_any_edge_unchecked(b, c));
        assert!(map.has_any_edge_unchecked(c, b));

        assert!(map.has_any_edge_unchecked(c, a));
        assert!(map.has_any_edge_unchecked(a, c));
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

        let ab = map.add_edge_unchecked(a, b, 1.into());
        let bc = map.add_edge_unchecked(b, c, 2.into());
        let ca = map.add_edge_unchecked(c, a, 3.into());

        // When: Incrementing edge of each edge by 1.
        map.update_edge_unchecked(a, b, ab, 2.into());
        map.update_edge_unchecked(b, c, bc, 3.into());
        map.update_edge_unchecked(c, a, ca, 4.into());

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

        let ab = map.add_edge_unchecked(a, b, 1.into());
        let bc = map.add_edge_unchecked(b, c, 2.into());
        let ca = map.add_edge_unchecked(c, a, 3.into());

        // When: Incrementing edge of each edge by 1.
        map.update_edge_unchecked(a, b, ab, 2.into());
        map.update_edge_unchecked(b, c, bc, 3.into());
        map.update_edge_unchecked(c, a, ca, 4.into());

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
        let ab = map.add_edge_unchecked(a, b, 1.into());
        let bc = map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        map.remove_edge_unchecked(a, b, ab);
        map.remove_edge_unchecked(b, c, bc);

        // Then:
        assert_eq!(map.vertex_count(), 3);
        println!("size: {:?}", map[a]);
        assert!(map[a].is_empty());
        assert!(map[b].is_empty());
        assert_eq!(map[c].len(), 1);

        assert_eq!(map.edges().len(), 1);
        assert_eq!(
            map.edges_between_unchecked(c, a)[0].get_weight().unwrap(),
            3
        );
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
        let ab = map.add_edge_unchecked(a, b, 1.into());
        let bc = map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        map.remove_edge_unchecked(a, b, ab);
        map.remove_edge_unchecked(b, c, bc);

        // Then:
        assert_eq!(map.vertex_count(), 3);
        assert_eq!(map[a].len(), 1);
        assert!(map[b].is_empty());
        assert_eq!(map[c].len(), 1);

        assert_eq!(map.edges().len(), 1);
        assert_eq!(
            map.edges_between_unchecked(a, c)[0].get_weight().unwrap(),
            3
        );
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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.neighbors_unchecked(a).len(), 1);
        assert_eq!(*map.neighbors_unchecked(a).get(0).unwrap(), b);

        assert_eq!(map.neighbors_unchecked(b).len(), 1);
        assert_eq!(*map.neighbors_unchecked(b).get(0).unwrap(), c);

        assert_eq!(map.neighbors_unchecked(c).len(), 1);
        assert_eq!(*map.neighbors_unchecked(c).get(0).unwrap(), a);
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
        map.add_edge_unchecked(a, b, 1.into());
        map.add_edge_unchecked(b, c, 2.into());
        map.add_edge_unchecked(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert_eq!(map.vertex_count(), 3);

        assert_eq!(map.neighbors_unchecked(a).len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| map.neighbors_unchecked(a).contains(vertex_id)));

        assert_eq!(map.neighbors_unchecked(b).len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| map.neighbors_unchecked(b).contains(vertex_id)));

        assert_eq!(map.neighbors_unchecked(c).len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| map.neighbors_unchecked(c).contains(vertex_id)));
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
        let mut di_mat = DiMap::<usize>::init();

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
}
