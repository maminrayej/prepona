use std::collections::HashSet;
use std::marker::PhantomData;

use anyhow::Result;

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
/// For more info and examples refer to `total_vertex_count` documentation.
/// * |E|: Means number of edges present in the graph.
/// * |E<sup>out</sup>|: Means number of edges exiting a vertex(out degree of the vertex).
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

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> AdjList<W, E, Dir> {
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
    /// Total number of vertices in the storage(|V|).
    ///
    /// # Complexity
    /// O(1)
    pub fn total_vertex_count(&self) -> usize {
        self.edges_of.len()
    }

    fn edges_of(&self, vertex_id: usize) -> Result<&Vec<(usize, E)>> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        }
        {
            Ok(&self.edges_of[vertex_id])
        }
    }

    fn edges_of_mut(&mut self, vertex_id: usize) -> Result<&mut Vec<(usize, E)>> {
        if !self.contains_vertex(vertex_id) {
            Err(Error::new_vnf(vertex_id))?
        }
        {
            Ok(&mut self.edges_of[vertex_id])
        }
    }

    fn edges_of_unsafe(&self, vertex_id: usize) -> &Vec<(usize, E)> {
        &self.edges_of[vertex_id]
    }

    fn edges_of_mut_unsafe(&mut self, vertex_id: usize) -> &mut Vec<(usize, E)> {
        &mut self.edges_of[vertex_id]
    }
}

impl<W: Copy, E: Edge<W> + Copy, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjList<W, E, Dir> {
    /// Adds a vertex to the graph.
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
        self.vertex_count += 1;

        if let Some(reusable_id) = self.next_reusable_vertex_id() {
            reusable_id
        } else {
            self.edges_of.push(vec![]);

            self.edges_of.len() - 1
        }
    }

    fn remove_vertex(&mut self, vertex_id: usize) -> Result<()> {
        self.edges_of_mut(vertex_id)?.clear();

        for src_id in self.vertices() {
            self.edges_of_mut_unsafe(src_id)
                .retain(|(dst_id, _)| *dst_id != vertex_id)
        }

        self.vertex_count -= 1;

        self.reusable_vertex_ids.insert(vertex_id);

        Ok(())
    }

    fn add_edge(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> Result<usize> {
        let edge_id = if let Some(id) = self.next_reusable_edge_id() {
            id
        } else {
            self.max_edge_id += 1;

            self.max_edge_id - 1
        };

        edge.set_id(edge_id);

        self.edges_of_mut(src_id)?.push((dst_id, edge));

        if self.is_undirected() {
            self.edges_of_mut(dst_id)?.push((src_id, edge));
        }

        Ok(edge_id)
    }

    fn update_edge(
        &mut self,
        src_id: usize,
        dst_id: usize,
        edge_id: usize,
        mut edge: E,
    ) -> Result<()> {
        // TODO: implement it without using `remove_edge` and `add_edge` to bypass some unnecessary checks.
        let removed_edge = self.remove_edge(src_id, dst_id, edge_id)?;

        edge.set_id(removed_edge.get_id());

        self.add_edge(src_id, dst_id, edge)?;

        Ok(())
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<E> {
        if let Some(index) = self
            .edges_of(src_id)?
            .iter()
            .position(|(_, edge)| edge.get_id() == edge_id)
        // TODO: check that edge is is indeed belongs to `dst_id` too!
        {
            self.reusable_edge_ids.insert(edge_id);

            if self.is_undirected() {
                self.edges_of_mut(dst_id)?
                    .retain(|(_, edge)| edge.get_id() != edge_id);
            }

            // If `src_id` was not valid `self.edges_of(src_id)?` would have returned error already.
            Ok(self.edges_of_mut_unsafe(src_id).remove(index).1)
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
        (0..self.edges_of.len())
            .filter(|vertex_id| !self.reusable_vertex_ids.contains(vertex_id))
            .collect()
    }

    fn edges_from(&self, src_id: usize) -> Result<Vec<(usize, &E)>> {
        Ok(self
            .edges_of(src_id)?
            .iter()
            .map(|(dst_id, edge)| (*dst_id, edge))
            .collect())
    }

    fn neighbors(&self, src_id: usize) -> Result<Vec<usize>> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .map(|(dst_id, _)| *dst_id)
            .collect())
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Result<Vec<&E>> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .filter_map(|(d_id, edge)| if *d_id == dst_id { Some(edge) } else { None })
            .collect())
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Result<&E> {
        self.edges_between(src_id, dst_id)?
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
            .ok_or(Error::new_iei(src_id, dst_id, edge_id).into())
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        let mut edges = vec![];

        for src_id in self.vertices() {
            let mut out_going_edges = self
                .edges_of_unsafe(src_id)
                .into_iter()
                .map(|(dst_id, edge)| (src_id, *dst_id, edge))
                .collect();

            edges.append(&mut out_going_edges)
        }

        edges
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> Result<bool> {
        Ok(self
            .edges_of(src_id)?
            .into_iter()
            .any(|(d_id, _)| *d_id == dst_id))
    }

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
