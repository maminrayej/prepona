mod utils;

use std::any::Any;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeType, FlowEdge, UndirectedEdge};
use crate::storage::GraphStorage;

pub type Mat<W, Ty = UndirectedEdge> = AdjMatrix<W, DefaultEdge<W>, Ty>;
pub type DiMat<W> = AdjMatrix<W, DefaultEdge<W>, DirectedEdge>;

pub type FlowMat<W, Ty = UndirectedEdge> = AdjMatrix<W, FlowEdge<W>, Ty>;
pub type DiFlowMat<W> = AdjMatrix<W, FlowEdge<W>, DirectedEdge>;

pub struct AdjMatrix<W, E: Edge<W>, Ty: EdgeType = UndirectedEdge> {
    // AdjMatrix uses a flat vector to store the adjacency matrix and uses a mapping function to map the (i,j) tuple to an index.
    // this mapping function depends on wether the matrix is used to store directed or undirected edges.
    // for more info about the mapping function, checkout utils module.
    vec: Vec<Vec<E>>,

    // When a vertex is deleted from the graph, AdjMatrix stores the removed vertex id in this struct to use it later when a vertex needs to be inserted into the graph.
    // Instead of allocation more space for the new vertex, AdjMatrix uses one of the available ids in this struct.
    reusable_ids: HashSet<usize>,

    edge_id: usize,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>,
    phantom_ty: PhantomData<Ty>,
}

impl<W, E: Edge<W>, Ty: EdgeType> AdjMatrix<W, E, Ty> {
    pub fn init() -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_ids: HashSet::new(),

            edge_id: 0,

            vertex_count: 0,
            is_directed: Ty::is_directed(),

            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }

    fn next_reusable_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_ids.iter().take(1).next().copied() {
            self.reusable_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    pub fn total_vertex_count(&self) -> usize {
        self.vertex_count + self.reusable_ids.len()
    }
}

impl<W: Any, E: Edge<W>, Ty: EdgeType> GraphStorage<W, E, Ty> for AdjMatrix<W, E, Ty> {
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_id() {
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

    fn remove_vertex(&mut self, vertex_id: usize) {
        for other_id in 0..self.total_vertex_count() {
            self[(vertex_id, other_id)].clear();
            self[(other_id, vertex_id)].clear();
        }

        self.reusable_ids.insert(vertex_id);

        self.vertex_count -= 1;
    }

    fn add_edge(&mut self, src_id: usize, dst_id: usize, mut edge: E) -> usize {
        edge.set_id(self.edge_id);

        self[(src_id, dst_id)].push(edge);

        self.edge_id += 1;

        self.edge_id - 1
    }

    fn update_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize, mut edge: E) {
        if let Some(index) = self[(src_id, dst_id)]
            .iter()
            .position(|edge| edge.get_id() == edge_id)
        {
            edge.set_id(edge_id);
            self[(src_id, dst_id)][index] = edge;
        }
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<E> {
        if let Some(index) = self[(src_id, dst_id)]
            .iter()
            .position(|edge| edge.get_id() == edge_id)
        {
            Some(self[(src_id, dst_id)].remove(index))
        } else {
            None
        }
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|v_id| !self.reusable_ids.contains(v_id))
            .collect()
    }

    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        (0..self.total_vertex_count())
            .into_iter()
            .flat_map(|dst_id| {
                self.edges_between(src_id, dst_id)
                    .into_iter()
                    .map(|edge| (dst_id, edge))
                    .collect::<Vec<(usize, &E)>>()
            })
            .collect()
    }

    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        (0..self.total_vertex_count())
            .into_iter()
            .filter(|dst_id| !self[(src_id, *dst_id)].is_empty())
            .collect()
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self[(src_id, dst_id)].iter().collect()
    }
}

use std::ops::{Index, IndexMut};
impl<W: Any, E: Edge<W>, Ty: EdgeType> Index<(usize, usize)> for AdjMatrix<W, E, Ty> {
    type Output = Vec<E>;

    fn index(&self, (src_id, dst_id): (usize, usize)) -> &Self::Output {
        let index = utils::from_ij(src_id, dst_id, self.is_directed);

        &self.vec[index]
    }
}

impl<W: Any, E: Edge<W>, Ty: EdgeType> IndexMut<(usize, usize)> for AdjMatrix<W, E, Ty> {
    fn index_mut(&mut self, (src_id, dst_id): (usize, usize)) -> &mut Self::Output {
        let index = utils::from_ij(src_id, dst_id, self.is_directed);

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
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id)));
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id)));
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

        assert!(!matrix.has_any_edge(c, c));
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

        assert!(!matrix.has_any_edge(c, c));
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id)));
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
            .all(|(src_id, dst_id)| !matrix.has_any_edge(src_id, dst_id)));
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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge(a, b));
        assert!(matrix.has_any_edge(b, c));
        assert!(matrix.has_any_edge(c, a));
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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

        // When: Doing nothing.

        // Then:
        assert!(matrix.has_any_edge(a, b));
        assert!(matrix.has_any_edge(b, a));

        assert!(matrix.has_any_edge(b, c));
        assert!(matrix.has_any_edge(c, b));

        assert!(matrix.has_any_edge(c, a));
        assert!(matrix.has_any_edge(a, c));
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
        let ab = matrix.add_edge(a, b, 1.into());
        let bc = matrix.add_edge(b, c, 2.into());
        let ca = matrix.add_edge(c, a, 3.into());

        // When: Incrementing edge of each edge by 1.
        matrix.update_edge(a, b, ab, 2.into());
        matrix.update_edge(b, c, bc, 3.into());
        matrix.update_edge(c, a, ca, 4.into());

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
        let ab = matrix.add_edge(a, b, 1.into());
        let bc = matrix.add_edge(b, c, 2.into());
        let ca = matrix.add_edge(c, a, 3.into());

        // When: Incrementing edge of each edge by 1.
        matrix.update_edge(a, b, ab, 2.into());
        matrix.update_edge(b, c, bc, 3.into());
        matrix.update_edge(c, a, ca, 4.into());

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
        let ab = matrix.add_edge(a, b, 1.into());
        let bc = matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        matrix.remove_edge(a, b, ab);
        matrix.remove_edge(b, c, bc);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 9);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(matrix.edges_between(c, a)[0].get_weight().unwrap(), 3);
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
        let ab = matrix.add_edge(a, b, 1.into());
        let bc = matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        matrix.remove_edge(a, b, ab);
        matrix.remove_edge(b, c, bc);

        // Then:
        assert_eq!(matrix.vertex_count(), 3);
        assert_eq!(matrix.total_vertex_count(), 3);
        assert_eq!(matrix.vec.len(), 6);

        assert_eq!(matrix.edges().len(), 1);
        assert_eq!(matrix.edges_between(a, c)[0].get_weight().unwrap(), 3);
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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

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
        matrix.add_edge(a, b, 1.into());
        matrix.add_edge(b, c, 2.into());
        matrix.add_edge(c, a, 3.into());

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

    // #[test]
    // #[should_panic(expected = "Vertex with id: 0 is not present in the graph")]
    // fn first_vertex_not_present() {
    //     // Given: Matrix
    //     //
    //     //      a
    //     //
    //     let mut matrix = Mat::<usize>::init();
    //     let a = matrix.add_vertex();
    //     let b = matrix.add_vertex();

    //     // When: Removing vertex a and try to pass it as valid vertex id.
    //     matrix.remove_vertex(a);
    //     matrix.edge(a, b);

    //     // Then: Code should panic.
    // }

    // #[test]
    // #[should_panic(expected = "Vertex with id: 1 is not present in the graph")]
    // fn second_vertex_not_present() {
    //     // Given: Matrix
    //     //
    //     //      a
    //     //
    //     let mut matrix = Mat::<usize>::init();
    //     let a = matrix.add_vertex();
    //     let b = matrix.add_vertex();

    //     // When: Removing vertex b and try to pass it as valid vertex id.
    //     matrix.remove_vertex(b);
    //     matrix.edge(a, b);

    //     // Then: Code should panic.
    // }

    // #[test]
    // #[should_panic(expected = "Vertices with id: 0 and 1 are not present in the graph")]
    // fn both_vertices_are_not_present() {
    //     // Given: An empty matrix.
    //     let mut matrix = Mat::<usize>::init();
    //     let a = matrix.add_vertex();
    //     let b = matrix.add_vertex();

    //     // When: Removing both vertices a and b and trying to pass them as valid ids.
    //     matrix.remove_vertex(a);
    //     matrix.remove_vertex(b);
    //     matrix.edge(a, b);

    //     // Then: Code should panic.
    // }

    // #[test]
    // #[should_panic(expected = "Index out of bounds: (0,1) does not exist")]
    // fn index_out_of_bounds() {
    //     // Given: Matrix
    //     //
    //     //      a
    //     //
    //     let mut matrix = Mat::<usize>::init();
    //     let a = matrix.add_vertex();
    //     let b = 1;

    //     // When: Trying to access b which is never add to graph.
    //     matrix.edge(a, b);

    //     // Then: Code should panic.
    // }
}
