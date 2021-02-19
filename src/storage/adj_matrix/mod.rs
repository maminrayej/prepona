mod utils;

use std::any::Any;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeDir, FlowEdge, UndirectedEdge};
use crate::storage::GraphStorage;

pub type Mat<W, Dir = UndirectedEdge> = AdjMatrix<W, DefaultEdge<W>, Dir>;
pub type DiMat<W> = AdjMatrix<W, DefaultEdge<W>, DirectedEdge>;

pub type FlowMat<W, Dir = UndirectedEdge> = AdjMatrix<W, FlowEdge<W>, Dir>;
pub type DiFlowMat<W> = AdjMatrix<W, FlowEdge<W>, DirectedEdge>;

/// Is a matrix used to represent a finite graph.
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
/// * |E<sup>*</sup>|: Means number of edges exiting a vertex(out degree of the vertex).
///
/// ## Space complexity
/// Space complexity of `AdjMatrix` depends on wether `Dir` is [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge). \
/// * **Directed**: For directed graphs `AdjMatrix` stores matrix with |V|<sup>2</sup> + |E| elements.
/// * **Undirected**: For undirected graphs `AdjMatrix` stores a lower triangle matrix with (|V|<sup>2</sup> + |V|)/2 + |E| elements.
///
/// ## Generic Parameters
/// * `W`: **W**eight type associated with edges.
/// * `E`: **E**dge type that graph uses.
/// * `Dir`: **Dir**ection of edges: [`Directed`](crate::graph::DirectedEdge) or [`Undirected`](crate::graph::UndirectedEdge).
pub struct AdjMatrix<W, E: Edge<W>, Dir: EdgeDir = UndirectedEdge> {
    vec: Vec<Vec<E>>,

    reusable_vertex_ids: HashSet<usize>,

    max_edge_id: usize,
    reusable_edge_ids: HashSet<usize>,

    vertex_count: usize,

    phantom_w: PhantomData<W>,
    phantom_dir: PhantomData<Dir>,
}

impl<W, E: Edge<W>, Dir: EdgeDir> AdjMatrix<W, E, Dir> {
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

    fn next_reusable_vertex_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_vertex_ids.iter().take(1).next().copied() {
            self.reusable_vertex_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

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
}

impl<W: Any, E: Edge<W>, Dir: EdgeDir> GraphStorage<W, E, Dir> for AdjMatrix<W, E, Dir> {
    /// Adds a vertex to the graph.
    ///
    /// # Returns
    /// Unique id of the newly added vertex.
    ///
    /// # Complexity
    /// O(|V|)
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_vertex_id() {
            self.vertex_count += 1;

            reusable_id
        } else {
            let new_size = if self.is_directed() {
                self.vec.len() + 2 * self.total_vertex_count() + 1
            } else {
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
    /// If `vertex_id` is not in range 0..|V|.
    fn remove_vertex_unchecked(&mut self, vertex_id: usize) {
        for other_id in 0..self.total_vertex_count() {
            self[(vertex_id, other_id)].clear();
            self[(other_id, vertex_id)].clear();
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

        self[(src_id, dst_id)].push(edge);

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
    /// O(E<sup>*</sup>)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    fn update_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        if let Some(index) = self[(src_id, dst_id)]
            .iter()
            .position(|edge| edge.get_id() == edge_id)
        {
            edge.set_id(edge_id);
            self[(src_id, dst_id)][index] = edge;
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
    /// O(E<sup>*</sup>)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    /// * If there is no edge with id: `edge_id` from `src_id` to `dst_id`.
    fn remove_edge_unchecked(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> E {
        let index = self[(src_id, dst_id)]
            .iter()
            .position(|edge| edge.get_id() == edge_id)
            .unwrap();

        self.reusable_edge_ids.insert(edge_id);

        self[(src_id, dst_id)].swap_remove(index)
    }

    /// # Returns
    /// Number of vertices in the graph.
    ///
    /// # Complexity
    /// O(1)
    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

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
    /// O(|V|*|E<sup>\*</sup>|)
    ///
    /// # Panics
    /// If `src_id` is not in range 0..|V|.
    fn edges_from_unchecked(&self, src_id: usize) -> Vec<(usize, &E)> {
        (0..self.total_vertex_count())
            .into_iter()
            .flat_map(|dst_id| {
                self.edges_between_unchecked(src_id, dst_id)
                    .into_iter()
                    .map(|edge| (dst_id, edge))
                    .collect::<Vec<(usize, &E)>>()
            })
            .collect()
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
    /// If `src_id` is not in range 0..|V|.
    fn neighbors_unchecked(&self, src_id: usize) -> Vec<usize> {
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|dst_id| !self[(src_id, *dst_id)].is_empty())
            .collect()
    }

    /// # Arguments
    /// * `src_id`: Id of source vertex.
    /// * `dst_id`: Id of destination vertex.
    ///
    /// # Returns
    /// Edges from source vertex to destination vertex.
    ///
    /// # Complexity
    /// O(|E<sup>*</sup>|)
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    fn edges_between_unchecked(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self[(src_id, dst_id)].iter().collect()
    }

    fn contains_vertex(&self, vertex_id: usize) -> bool {
        vertex_id < self.total_vertex_count() && !self.reusable_vertex_ids.contains(&vertex_id)
    }

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
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    fn index(&self, (src_id, dst_id): (usize, usize)) -> &Self::Output {
        let index = utils::from_ij(src_id, dst_id, self.is_directed());

        &self.vec[index]
    }
}

impl<W: Any, E: Edge<W>, Dir: EdgeDir> IndexMut<(usize, usize)> for AdjMatrix<W, E, Dir> {
    /// # Arguments
    /// * (`src_id`, `dst_id`): (Id of the source vertex, Id of the destination vertex).
    ///
    /// # Returns
    /// Edges from vertex with id: `src_id` to vertex with id: `dst_id`.
    ///
    /// # Panics
    /// * If `src_id` or `dst_id` is not in range 0..|V|.
    fn index_mut(&mut self, (src_id, dst_id): (usize, usize)) -> &mut Self::Output {
        let index = utils::from_ij(src_id, dst_id, self.is_directed());

        &mut self.vec[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_empty_matrix() {
        // Given: An empty directed matrix.
        let matrix = DiMat::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.edges().len(), 0);
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

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.edges().len(), 0);
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
        assert_eq!(matrix.edges().len(), 0);
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
        assert_eq!(matrix.edges().len(), 0);
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
        assert_eq!(matrix.edges().len(), 0);
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
        assert_eq!(matrix.edges().len(), 0);
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
        assert_eq!(matrix.edges().len(), 0);
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

        // When: Incrementing edge of each edge by 1.
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

        // When: Incrementing edge of each edge by 1.
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
}
