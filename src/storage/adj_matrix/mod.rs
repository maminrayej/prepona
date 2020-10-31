mod utils;

use magnitude::Magnitude;
use std::any::Any;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::storage::GraphStorage;
use crate::graph::Edge;

/// For a simple graph with vertex set *V*, the adjacency matrix is a square |V| × |V| matrix *A*
/// such that its element *A<sub>ij</sub>* is the weight when there is an edge from vertex *i* to vertex *j*, and ∞ when there is no edge.
///
/// In an undirected graph, the adjacency matrix is symmetric in the sense that: ∀ i,j *A<sub>ij</sub>* = *A<sub>ji</sub>*.
/// Therefore `AdjMatrix` only stores the lower triangle of the matrix to save space.
///
/// # Conventions:
/// * |V| represents total number of vertices in the adjacency matrix: \
/// number of vertices present in the graph + number of removed vertices that are present in the adjacency matrix.
pub struct AdjMatrix<W, E: Edge<W>> {
    // AdjMatrix uses a flat vector to store the adjacency matrix and uses a mapping function to map the (i,j) tuple to an index.
    // this mapping function depends on wether the matrix is used to store directed or undirected edges.
    // for more info about the mapping function checkout utils module.
    vec: Vec<E>,

    // When a vertex is deleted from the graph, AdjMatrix stores the removed vertex id in this struct to use it later when a vertex needs to be inserted into the graph.
    // Instead of allocation more space for the new vertex, AdjMatrix uses one of the available ids in this struct.
    reusable_ids: HashSet<usize>,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>

}

impl<W, E: Edge<W>> AdjMatrix<W, E> {
    /// Initializes an adjacency matrix.
    ///
    /// # Arguments:
    /// * `edge_type`: indicates wether the adjacency matrix is going to store directed or undirected edges.
    ///
    /// # Returns:
    /// An adjacency matrix.
    pub fn init(is_directed: bool) -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_ids: HashSet::new(),
            vertex_count: 0,
            is_directed,

            phantom_w: PhantomData
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
}

impl<W: Any + Copy, E: Edge<W>> GraphStorage<W, E> for AdjMatrix<W, E> {
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
            reusable_id
        } else {
            let new_size = if self.is_directed() {
                // Has to allocate for a new row(|V|) + a new column(|V|) + one slot for the diagonal: 2 * |V| + 1
                self.vec.len() + 2 * self.total_vertex_count() + 1
            } else {
                // Has to allocate just one row(|V|) + one slot for diagonal: |V| + 1
                self.vec.len() + self.total_vertex_count() + 1
            };

            // Populate these new allocated slots with positive infinity
            self.vec.resize_with(new_size, || E::init(Magnitude::PosInfinite));

            self.vertex_count += 1;

            self.vertex_count - 1
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
            self[(vertex_id, other_id)] = E::init(Magnitude::PosInfinite);
            self[(other_id, vertex_id)] = E::init(Magnitude::PosInfinite);
        }

        // Removed vertex id is now reusable
        self.reusable_ids.insert(vertex_id);

        self.vertex_count -= 1;
    }

    /// Adds an edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    /// * `weight`: Weight of the edge between `src_id` and `dst_id`.
    ///
    /// # Panics:
    /// * If vertex with `src_id` or `dst_id` is not present in the graph.
    /// * If slot [`src_id`][`dst_id`] is out of bounds.
    ///
    /// # Complexity:
    /// O(1).
    fn add_edge(&mut self, src_id: usize, dst_id: usize, edge: E) {
        self[(src_id, dst_id)] = edge;
    }

    /// Removes the edge from vertex with `src_id` to vertex with `dst_id`.
    ///
    /// # Arguments:
    /// * `src_id`: Id of the vertex at the start of the edge.
    /// * `dst_id`: Id of the vertex at the end of the edge.
    ///
    /// # Returns:
    /// The weight of the removed edge.
    ///
    /// # Panics:
    /// * If vertex with `src_id` or `dst_id` is not present in the graph.
    /// * If slot [`src_id`][`dst_id`] is out of bounds.
    ///
    /// # Complexity:
    /// O(1).
    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        let mut edge = E::init(Magnitude::PosInfinite);

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
        // Out of all vertex ids, filter out the ones that are reusable(hence are removed and not present in the graph).
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|v_id| !self.reusable_ids.contains(v_id))
            .collect()
    }

    /// # Returns:
    /// Vector of edges in the format of (`src_id`, `dst_id`, `weight`).
    ///
    /// # Complexity:
    /// O(|V|<sup>2</sup>).
    fn edges(&self) -> Vec<(usize, usize, &E)> {
        let vertices = self.vertices();

        // 1. Produce cartesian product: { vertices } x { vertices }:
        //  1.1: For every vertex v1 produce (v1, v2): ∀v2 ∈ { vertices }.
        //  1.2: Previous step will produce |V| vector of tuples each with length |V|, flat it to a single vector of |V|*|V| tuples.
        // 2. Map each tuple (v1, v2) to (v1, v2, weight of edge between v1 and v2).
        vertices
            .iter()
            .flat_map(|v1| {
                vertices
                    .iter()
                    .map(|v2| (*v1, *v2))
                    .collect::<Vec<(usize, usize)>>()
            })
            .map(|(v1, v2)| (v1, v2, &self[(v1, v2)]))
            .collect()
    }

    /// # Returns:
    /// Vector of edges from vertex with `src_id` in the format of (`dst_id`, `weight`).
    ///
    /// # Arguments:
    /// * `src_id`: Id of the source vertex.
    ///
    /// Complexity:
    /// O(|V|).
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        // 1. Produce tuple (v, weight of edge between src and v): ∀v ∈ { vertices }.
        // 2. Only keep those tuples that their weight is finite(weight with infinite value indicates absence of edge between src and v).
        self.vertices()
            .into_iter()
            .map(|v_id| (v_id, &self[(src_id, v_id)]))
            .filter(|(_, edge)| edge.get_weight().is_finite())
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
        // Of all vertices, only keep those that there exists a edge from vertex with `src_id` to them.
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
impl<W: Copy + Any, E: Edge<W>> Index<(usize, usize)> for AdjMatrix<W, E> {
    type Output = E;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (src_id, dst_id) = index;

        // Make sure both src and dst vertices are present in the graph.
        match (
            self.reusable_ids.contains(&src_id),
            self.reusable_ids.contains(&dst_id),
        ) {
            (true, true) => panic!(
                "Vertices with id: {} and {} are not present in the graph",
                src_id, dst_id
            ),
            (true, false) => panic!(format!(
                "Vertex with id: {} is not present in the graph",
                src_id
            )),
            (false, true) => panic!(format!(
                "Vertex with id: {} is not present in the graph",
                dst_id
            )),
            (false, false) => (),
        }

        // Map (src_id, dst_id) to the corresponding index in the flat vector.
        let index = utils::from_ij(src_id, dst_id, self.is_directed());

        if index < self.vec.len() {
            &self.vec[index]
        } else {
            panic!(format!(
                "Index out of bounds: ({src_id},{dst_id}) does not exist",
                src_id = src_id,
                dst_id = dst_id
            ))
        }
    }
}

impl<W: Copy + Any, E: Edge<W>> IndexMut<(usize, usize)> for AdjMatrix<W, E> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (src_id, dst_id) = index;

        // Make sure both src and dst vertices are present in the graph.
        match (
            self.reusable_ids.contains(&src_id),
            self.reusable_ids.contains(&dst_id),
        ) {
            (true, true) => panic!(
                "Vertices with id: {} and {} are not present in the graph",
                src_id, dst_id
            ),
            (true, false) => panic!(format!(
                "Vertex with id: {} is not present in the graph",
                src_id
            )),
            (false, true) => panic!(format!(
                "Vertex with id: {} is not present in the graph",
                dst_id
            )),
            (false, false) => (),
        }

        // Map (src_id, dst_id) to the corresponding index in the flat vector.
        let index = utils::from_ij(src_id, dst_id, self.is_directed());

        if index < self.vec.len() {
            &mut self.vec[index]
        } else {
            panic!(format!(
                "Index out of bounds: ({src_id},{dst_id}) does not exist",
                src_id = src_id,
                dst_id = dst_id
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::DefaultEdge;

    #[test]
    fn directed_add_vertex() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for i in 0..10 {
            let v_id = adj_matrix.add_vertex();
            assert!(v_id == i);
        }

        assert!(adj_matrix.vertex_count == 10);
        assert!(adj_matrix.vec.len() == 100);
    }

    #[test]
    fn undirected_add_vertex() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for i in 0..10 {
            let v_id = adj_matrix.add_vertex();
            assert!(v_id == i);
        }

        assert!(adj_matrix.vertex_count == 10);
        assert!(adj_matrix.vec.len() == 55);
    }

    #[test]
    fn directed_remove_vertex() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in 0..10 {
            adj_matrix.remove_vertex(i);
            assert!(adj_matrix.vertex_count() == 10 - (i + 1))
        }

        assert!(adj_matrix.vec.iter().all(|edge| edge.get_weight().is_pos_infinite()));
    }

    #[test]
    fn undirected_remove_vertex() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in 0..10 {
            adj_matrix.remove_vertex(i);
            assert!(adj_matrix.vertex_count() == 10 - (i + 1))
        }

        assert!(adj_matrix.vec.iter().all(|edge| edge.get_weight().is_pos_infinite()));
    }

    #[test]
    fn directed_add_edge() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix[(v1, v2)] = DefaultEdge::init((v1 * 2 + v2).into());
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                assert_eq!(adj_matrix[(v1, v2)].get_weight(), &(v1 * 2 + v2).into());
            }
        }
    }

    #[test]
    fn undirected_add_edge() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix[(v1, v2)] = Edge::init((v1 * 2 + v2).into());
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                assert_eq!(adj_matrix[(v1, v2)].get_weight(), &(v1 * 2 + v2).into());
                assert_eq!(adj_matrix[(v2, v1)].get_weight(), &(v1 * 2 + v2).into());
            }
        }
    }

    #[test]
    fn directed_remove_edge() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix[(v1, v2)] = Edge::init((v1 * 2 + v2).into());
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix.remove_edge(v1, v2);
                assert!(adj_matrix[(v1, v2)].get_weight().is_pos_infinite());
            }
        }
    }

    #[test]
    fn undirected_remove_edge() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix[(v1, v2)] = Edge::init((v1 * 2 + v2).into());
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix.remove_edge(v1, v2);
                assert!(adj_matrix[(v1, v2)].get_weight().is_pos_infinite());
                assert!(adj_matrix[(v2, v1)].get_weight().is_pos_infinite());
            }
        }
    }

    #[test]
    fn vertices() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in (0..10).step_by(2) {
            adj_matrix.remove_vertex(i);
        }

        let vertices = adj_matrix.vertices();
        assert_eq!(vertices.len(), 5);
        assert!(vec![1, 3, 5, 7, 9].iter().all(|v| vertices.contains(v)));

        let reusable_ids = &adj_matrix.reusable_ids;
        assert_eq!(reusable_ids.len(), 5);
        assert!(vec![0, 2, 4, 6, 8].iter().all(|v| reusable_ids.contains(v)));
    }

    #[test]
    fn edges_directed() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        for i in (0..5).step_by(2) {
            adj_matrix.remove_vertex(i);
        }

        adj_matrix[(1, 3)] = Edge::init(3.into());
        adj_matrix[(3, 1)] = Edge::init(4.into());

        for (v1, v2, edge) in adj_matrix.edges() {
            match (v1, v2) {
                (1, 1) => assert!(edge.get_weight().is_pos_infinite()),
                (1, 3) => assert_eq!(edge.get_weight(), &3.into()),
                (3, 1) => assert_eq!(edge.get_weight(), &4.into()),
                (3, 3) => assert!(edge.get_weight().is_pos_infinite()),
                _ => panic!("not valid vertices"),
            }
        }
    }

    #[test]
    fn edges_undirected() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        for i in (0..5).step_by(2) {
            adj_matrix.remove_vertex(i);
        }

        adj_matrix[(1, 3)] = Edge::init(3.into());

        for (v1, v2, edge) in adj_matrix.edges() {
            match (v1, v2) {
                (1, 1) => assert!(edge.get_weight().is_pos_infinite()),
                (1, 3) => assert_eq!(edge.get_weight(), &3.into()),
                (3, 1) => assert_eq!(edge.get_weight(), &3.into()),
                (3, 3) => assert!(edge.get_weight().is_pos_infinite()),
                _ => panic!("not valid vertices"),
            }
        }
    }

    #[test]
    fn neighbors_directed() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(true);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        adj_matrix[(1, 3)] = Edge::init(3.into());
        adj_matrix[(1, 2)] = Edge::init(2.into());
        adj_matrix[(4, 0)] = Edge::init(1.into());

        let one_neighbors = adj_matrix.neighbors(1);

        assert_eq!(one_neighbors.len(), 2);
        assert!(one_neighbors.contains(&3));
        assert!(one_neighbors.contains(&2));

        let four_neighbors = adj_matrix.neighbors(4);
        assert_eq!(four_neighbors.len(), 1);
        assert!(four_neighbors.contains(&0));

        assert!(adj_matrix.neighbors(0).is_empty());
        assert!(adj_matrix.neighbors(2).is_empty());
        assert!(adj_matrix.neighbors(3).is_empty());
    }

    #[test]
    fn neighbors_undirected() {
        let mut adj_matrix = AdjMatrix::<usize, DefaultEdge<usize>>::init(false);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        adj_matrix[(1, 3)] = Edge::init(3.into());
        adj_matrix[(4, 0)] = Edge::init(1.into());

        let one_neighbors = adj_matrix.neighbors(1);
        assert_eq!(one_neighbors.len(), 1);
        assert!(one_neighbors.contains(&3));

        let three_neighbors = adj_matrix.neighbors(3);
        assert_eq!(three_neighbors.len(), 1);
        assert!(three_neighbors.contains(&1));

        let four_neighbors = adj_matrix.neighbors(4);
        assert_eq!(four_neighbors.len(), 1);
        assert!(four_neighbors.contains(&0));

        let zero_neighbors = adj_matrix.neighbors(0);
        assert_eq!(zero_neighbors.len(), 1);
        assert!(zero_neighbors.contains(&4));

        assert!(adj_matrix.neighbors(2).is_empty());
    }
}
