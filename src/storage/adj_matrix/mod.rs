mod utils;

use magnitude::Magnitude;
use std::any::Any;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeType, FlowEdge, UndirectedEdge};
use crate::storage::GraphStorage;

/// An `AdjMatrix` that stores edges of type `DefaultEdge`.
pub type Mat<W, Ty = UndirectedEdge> = AdjMatrix<W, DefaultEdge<W>, Ty>;
pub type DiMat<W> = AdjMatrix<W, DefaultEdge<W>, DirectedEdge>;

/// An `AdjMatrix` that stores edges of type `FlowEdge`.
pub type FlowMat<W> = AdjMatrix<W, FlowEdge<W>, UndirectedEdge>;
pub type DiFlowMat<W> = AdjMatrix<W, FlowEdge<W>, DirectedEdge>;

/// For a simple graph with vertex set *V*, the adjacency matrix is a square |V| × |V| matrix *A*
/// such that its element *A<sub>ij</sub>* is the weight when there is an edge from vertex *i* to vertex *j*, and ∞ when there is no edge.
///
/// In an undirected graph, the adjacency matrix is symmetric in the sense that: ∀ i,j *A<sub>ij</sub>* = *A<sub>ji</sub>*.
/// Therefore `AdjMatrix` only stores the lower triangle of the matrix to save space.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
/// * `E`: Edge of the graph.
///
/// # Conventions:
/// * |V| represents total number of vertices in the adjacency matrix: \
/// number of vertices present in the graph + number of removed vertices that are present in the adjacency matrix.
pub struct AdjMatrix<W, E: Edge<W>, Ty: EdgeType = UndirectedEdge> {
    // AdjMatrix uses a flat vector to store the adjacency matrix and uses a mapping function to map the (i,j) tuple to an index.
    // this mapping function depends on wether the matrix is used to store directed or undirected edges.
    // for more info about the mapping function, checkout utils module.
    vec: Vec<E>,

    // When a vertex is deleted from the graph, AdjMatrix stores the removed vertex id in this struct to use it later when a vertex needs to be inserted into the graph.
    // Instead of allocation more space for the new vertex, AdjMatrix uses one of the available ids in this struct.
    reusable_ids: HashSet<usize>,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>,
    phantom_ty: PhantomData<Ty>,
}

impl<W, E: Edge<W>, Ty: EdgeType> AdjMatrix<W, E, Ty> {
    /// Initializes an adjacency matrix.
    ///
    /// # Arguments:
    /// * `is_directed`: indicates wether the adjacency matrix is going to store directed or undirected edges.
    ///
    /// # Returns:
    /// An adjacency matrix.
    pub fn init() -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_ids: HashSet::new(),
            vertex_count: 0,
            is_directed: Ty::is_directed(),

            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }

    // If there exists a reusable id, this method returns it and removes the id from the reusable_ids struct.
    // If there is no reusable id, this method returns None.
    //
    // Complexity:
    // O(1)
    fn next_reusable_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_ids.iter().take(1).next().copied() {
            self.reusable_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    /// Returns number of both vertices that are present in the graph and the vertices that are removed but are ready to be reused again. \
    /// In other words this method return the number of rows/columns of the adjacency matrix.
    ///
    /// # Returns:
    /// Total number of vertices present in the adjacency matrix: |V|.
    ///
    /// # Complexity:
    /// O(1)
    pub fn total_vertex_count(&self) -> usize {
        self.vertex_count + self.reusable_ids.len()
    }

    fn index_of(&self, src_id: usize, dst_id: usize) -> Result<usize, String> {
        // Make sure both src and dst vertices are present in the graph.
        match (
            self.reusable_ids.contains(&src_id),
            self.reusable_ids.contains(&dst_id),
        ) {
            (true, true) => Err(format!(
                "Vertices with id: {} and {} are not present in the graph",
                src_id, dst_id
            )),
            (true, false) => Err(format!(
                "Vertex with id: {} is not present in the graph",
                src_id
            )),
            (false, true) => Err(format!(
                "Vertex with id: {} is not present in the graph",
                dst_id
            )),
            (false, false) => {
                let index = utils::from_ij(src_id, dst_id, self.is_directed);

                if index < self.vec.len() {
                    Ok(index)
                } else {
                    Err(format!(
                        "Index out of bounds: ({src_id},{dst_id}) does not exist",
                        src_id = src_id,
                        dst_id = dst_id
                    ))
                }
            }
        }
    }
}

impl<W: Any, E: Edge<W>, Ty: EdgeType> GraphStorage<W, E, Ty> for AdjMatrix<W, E, Ty> {
    /// Adds a vertex into the adjacency matrix.
    ///
    /// # Returns:
    /// Id of the newly inserted vertex.
    ///
    /// # Complexity:
    /// * If there exists a reusable id: O(1).
    /// * Else: O(|V|).
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_id() {
            self.vertex_count += 1;

            reusable_id
        } else {
            let new_size = if self.is_directed() {
                // Has to allocate for a new row(|V|) + a new column(|V|) + one slot for the diagonal: 2 * |V| + 1.
                self.vec.len() + 2 * self.total_vertex_count() + 1
            } else {
                // Has to allocate just one row(|V|) + one slot for diagonal: |V| + 1.
                self.vec.len() + self.total_vertex_count() + 1
            };

            // Populate these new allocated slots with positive infinity.
            let vertex_id = self.vertex_count();

            self.vec.resize_with(new_size, || {
                Edge::init(vertex_id, vertex_id, Magnitude::PosInfinite)
            });

            (0..self.total_vertex_count()).for_each(|v_id| {
                self[(vertex_id, v_id)] = Edge::init(vertex_id, v_id, Magnitude::PosInfinite);
                self[(v_id, vertex_id)] = Edge::init(v_id, vertex_id, Magnitude::PosInfinite);
            });

            self.vertex_count += 1;

            vertex_id
        }
    }

    /// Removes a vertex from the adjacency matrix.
    ///
    /// # Arguments:
    /// * `vertex_id`: id of the vertex to be removed.
    ///
    /// # Complexity:
    /// O(|V|).
    fn remove_vertex(&mut self, vertex_id: usize) {
        // When a vertex is removed, row and column corresponding to that vertex must be filled with positive infinity.
        // ex: if vertex with id: 1 got removed
        //  ___________
        // |   | ∞ |   |
        // | ∞ | ∞ | ∞ |
        // |   | ∞ |   |
        //  -----------
        for other_id in self.vertices() {
            self[(vertex_id, other_id)] = Edge::init(vertex_id, other_id, Magnitude::PosInfinite);
            self[(other_id, vertex_id)] = Edge::init(other_id, vertex_id, Magnitude::PosInfinite);
        }

        // Removed vertex id is now reusable.
        self.reusable_ids.insert(vertex_id);

        self.vertex_count -= 1;
    }

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    /// * `edge`: The edge between `src_id` and `dst_id`.
    ///
    /// # Panics:
    /// * If vertex with `src_id` or `dst_id` is not present in the graph.
    /// * If slot [`src_id`][`dst_id`] is out of bounds.
    ///
    /// # Complexity:
    /// O(1).
    fn add_edge(&mut self, edge: E) {
        let (src_id, dst_id) = (edge.get_src_id(), edge.get_dst_id());

        self[(src_id, dst_id)] = edge;
    }

    fn update_edge(&mut self, edge: E) {
        self.add_edge(edge);
    }

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The removed edge.
    ///
    /// # Panics:
    /// * If vertex with `src_id` or `dst_id` is not present in the graph.
    /// * If slot [`src_id`][`dst_id`] is out of bounds.
    ///
    /// # Complexity:
    /// O(1).
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        let mut edge = E::init(src_id, dst_id, Magnitude::PosInfinite);

        std::mem::swap(&mut self[(src_id, dst_id)], &mut edge);

        edge
    }

    /// # Returns:
    /// Number of vertices present in the graph.
    ///
    /// # Complexity:
    /// O(1).
    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// # Returns:
    /// Vector of vertex ids that are present in the graph.
    ///
    /// # Complexity:
    /// O(|V|).
    fn vertices(&self) -> Vec<usize> {
        // Out of all vertex ids, remove ids that are reusable(hence are removed and not present in the graph).
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|v_id| !self.reusable_ids.contains(v_id))
            .collect()
    }

    fn edge(&self, src_id: usize, dst_id: usize) -> Option<&E> {
        let edge = &self[(src_id, dst_id)];

        if edge.get_weight().is_finite() {
            Some(edge)
        } else {
            None
        }
    }

    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `edge`).
    ///
    /// # Complexity:
    /// O(|V|<sup>2</sup>).
    fn edges(&self) -> Vec<&E> {
        let vertices = self.vertices();

        // 1. Produce cartesian product: { vertices } x { vertices }:
        //  1.1: For every vertex v1 produce (v1, v2): ∀v2 ∈ { vertices }.
        //  1.2: Previous step will produce |V| vector of tuples each with length |V|, flat it to a single vector of |V|*|V| tuples.
        // 2. Map each tuple (v1, v2) to (v1, v2, edge between v1 and v2).
        vertices
            .iter()
            .flat_map(|v1| self.edges_from(*v1))
            .filter(|edge| edge.get_weight().is_finite())
            .collect()
    }

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `edge`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// Complexity:
    /// O(|V|).
    fn edges_from(&self, src_id: usize) -> Vec<&E> {
        // 1. Produce tuple (v, edge between src and v): ∀v ∈ { vertices }.
        // 2. Only keep those tuples that weight of their edge is finite(weight with infinite value indicates absence of edge between src and v).
        self.vertices()
            .into_iter()
            .map(|v_id| &self[(src_id, v_id)])
            .filter(|edge| edge.get_weight().is_finite())
            .collect()
    }

    /// # Returns:
    /// Id of neighbors of the vertex with `src_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// # Complexity:
    /// O(|V|).
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        // Of all vertices, only keep those that there exists an edge from vertex with `src_id` to them.
        self.vertices()
            .into_iter()
            .filter(|dst_id| self[(src_id, *dst_id)].get_weight().is_finite())
            .collect()
    }

    /// # Returns:
    /// `true` If edges stored in the matrix is directed `false` otherwise.
    ///
    /// # Complexity:
    /// O(1).
    fn is_directed(&self) -> bool {
        self.is_directed
    }
}

use std::ops::{Index, IndexMut};
impl<W: Any, E: Edge<W>, Ty: EdgeType> Index<(usize, usize)> for AdjMatrix<W, E, Ty> {
    type Output = E;

    fn index(&self, src_dst: (usize, usize)) -> &Self::Output {
        let (src_id, dst_id) = src_dst;

        let index_result = self.index_of(src_id, dst_id);

        // The error message is already in the Err, no need to specify a message.
        &self.vec[index_result.expect("")]
    }
}

impl<W: Any, E: Edge<W>, Ty: EdgeType> IndexMut<(usize, usize)> for AdjMatrix<W, E, Ty> {
    fn index_mut(&mut self, src_dst: (usize, usize)) -> &mut Self::Output {
        let (src_id, dst_id) = src_dst;

        let index_result = self.index_of(src_id, dst_id);

        // The error message is already in the Err, no need to specify a message.
        &mut self.vec[index_result.expect("")]
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
        assert_eq!(matrix.reusable_ids.len(), 0);
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
        assert_eq!(matrix.reusable_ids.len(), 0);
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
        assert_eq!(matrix.reusable_ids.len(), 0);

        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| matrix.edge(src_id, dst_id).is_none()));
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
        assert_eq!(matrix.reusable_ids.len(), 0);

        assert_eq!(matrix.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| matrix.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| matrix.edge(src_id, dst_id).is_none()));
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
        matrix.remove_vertex(a);
        matrix.remove_vertex(b);

        // Then:
        assert_eq!(matrix.edges().len(), 0);
        assert_eq!(matrix.vertex_count(), 1);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        // Vertices a and b must be reusable.
        assert_eq!(matrix.reusable_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.reusable_ids.contains(vertex_id)));

        // Matrix must only contain c.
        assert_eq!(matrix.vertices().len(), 1);
        assert!(matrix.vertices().contains(&c));

        assert!(matrix.edge(c, c).is_none());
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
        matrix.remove_vertex(a);
        matrix.remove_vertex(b);

        // Then:
        assert_eq!(matrix.edges().len(), 0);
        assert_eq!(matrix.vertex_count(), 1);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        // Vertices a and b must be reusable.
        assert_eq!(matrix.reusable_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.reusable_ids.contains(vertex_id)));

        // Matrix must only contain c.
        assert_eq!(matrix.vertices().len(), 1);
        assert!(matrix.vertices().contains(&c));

        assert!(matrix.edge(c, c).is_none());
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
        matrix.remove_vertex(a);
        matrix.remove_vertex(b);
        let _ = matrix.add_vertex();
        let _ = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.edges().len(), 0);
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        // There must be no reusable id.
        assert_eq!(matrix.reusable_ids.len(), 0);

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
            .all(|(src_id, dst_id)| matrix.edge(src_id, dst_id).is_none()));
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
        matrix.remove_vertex(a);
        matrix.remove_vertex(b);
        let _ = matrix.add_vertex();
        let _ = matrix.add_vertex();

        // Then:
        assert_eq!(matrix.edges().len(), 0);
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        // There must be no reusable id.
        assert_eq!(matrix.reusable_ids.len(), 0);

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
            .all(|(src_id, dst_id)| matrix.edge(src_id, dst_id).is_none()));
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 3);
        for edge in matrix.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 6);
        for edge in matrix.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_edge(a, b));
        assert!(matrix.has_edge(b, c));
        assert!(matrix.has_edge(c, a));
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_edge(a, b));
        assert!(matrix.has_edge(b, a));

        assert!(matrix.has_edge(b, c));
        assert!(matrix.has_edge(c, b));
        
        assert!(matrix.has_edge(c, a));
        assert!(matrix.has_edge(a, c));
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Incrementing edge of each edge by 1.
        matrix.update_edge((a, b, 2).into());
        matrix.update_edge((b, c, 3).into());
        matrix.update_edge((c, a, 4).into());

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 3);
        for edge in matrix.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Incrementing edge of each edge by 1.
        matrix.update_edge((a, b, 2).into());
        matrix.update_edge((b, c, 3).into());
        matrix.update_edge((c, a, 4).into());

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 6);
        for edge in matrix.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        matrix.remove_edge(a, b);
        matrix.remove_edge(b, c);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(matrix.edge(c, a).unwrap().get_weight().unwrap(), 3);
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        matrix.remove_edge(a, b);
        matrix.remove_edge(b, c);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 2);
        assert_eq!(matrix.edge(a, c).unwrap().get_weight().unwrap(), 3);
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.neighbors(a).len(), 1);
        assert_eq!(*matrix.neighbors(a).get(0).unwrap(), b);

        assert_eq!(matrix.neighbors(b).len(), 1);
        assert_eq!(*matrix.neighbors(b).get(0).unwrap(), c);

        assert_eq!(matrix.neighbors(c).len(), 1);
        assert_eq!(*matrix.neighbors(c).get(0).unwrap(), a);
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
        matrix.add_edge((a, b, 1).into());
        matrix.add_edge((b, c, 2).into());
        matrix.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.neighbors(a).len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| matrix.neighbors(a).contains(vertex_id)));

        assert_eq!(matrix.neighbors(b).len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| matrix.neighbors(b).contains(vertex_id)));

        assert_eq!(matrix.neighbors(c).len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| matrix.neighbors(c).contains(vertex_id)));
    }

    #[test]
    #[should_panic(expected = "Vertex with id: 0 is not present in the graph")]
    fn first_vertex_not_present() {
        // Given: Matrix
        //
        //      a
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();

        // When: Removing vertex a and try to pass it as valid vertex id.
        matrix.remove_vertex(a);
        matrix.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Vertex with id: 1 is not present in the graph")]
    fn second_vertex_not_present() {
        // Given: Matrix
        //
        //      a
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();

        // When: Removing vertex b and try to pass it as valid vertex id.
        matrix.remove_vertex(b);
        matrix.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Vertices with id: 0 and 1 are not present in the graph")]
    fn both_vertices_are_not_present() {
        // Given: An empty matrix.
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = matrix.add_vertex();

        // When: Removing both vertices a and b and trying to pass them as valid ids.
        matrix.remove_vertex(a);
        matrix.remove_vertex(b);
        matrix.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: (0,1) does not exist")]
    fn index_out_of_bounds() {
        // Given: Matrix
        //
        //      a
        //
        let mut matrix = Mat::<usize>::init();
        let a = matrix.add_vertex();
        let b = 1;

        // When: Trying to access b which is never add to graph.
        matrix.edge(a, b);

        // Then: Code should panic.
    }
}
