mod utils;

use magnitude::Magnitude;
use std::any::Any;
use std::collections::HashSet;

use crate::graph::EdgeType;
use crate::storage::GraphStorage;

pub struct AdjMatrix<W> {
    vec: Vec<Magnitude<W>>,
    reusable_ids: HashSet<usize>,
    vertex_count: usize,
    edge_type: EdgeType,
}

impl<W> AdjMatrix<W> {
    pub fn init(edge_type: EdgeType) -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_ids: HashSet::new(),
            vertex_count: 0,
            edge_type,
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

impl<W: Any + Copy> GraphStorage<W> for AdjMatrix<W> {
    fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_id() {
            reusable_id
        } else {
            let new_size = if self.is_directed() {
                self.vec.len() + 2 * self.total_vertex_count() + 1
            } else {
                self.vec.len() + self.total_vertex_count() + 1
            };

            self.vec.resize_with(new_size, || Magnitude::PosInfinite);

            self.vertex_count += 1;

            self.vertex_count - 1
        }
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        for other_id in self.vertices() {
            self[(vertex_id, other_id)] = Magnitude::PosInfinite;
            self[(other_id, vertex_id)] = Magnitude::PosInfinite;
        }

        self.reusable_ids.insert(vertex_id);

        self.vertex_count -= 1;
    }

    fn add_edge(&mut self, vertex1: usize, vertex2: usize, edge: Magnitude<W>) {
        self[(vertex1, vertex2)] = edge;
    }

    fn remove_edge(&mut self, vertex1: usize, vertex2: usize) -> Magnitude<W> {
        let mut edge = Magnitude::PosInfinite;

        std::mem::swap(&mut self[(vertex1, vertex2)], &mut edge);

        edge
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

    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)> {
        let vertices = self.vertices();

        vertices
            .iter()
            .flat_map(|v1| {
                vertices
                    .iter()
                    .map(|v2| (*v1, *v2))
                    .collect::<Vec<(usize, usize)>>()
            })
            .map(|(v1, v2)| (v1, v2, self[(v1, v2)]))
            .collect()
    }

    fn edges_from(&self, src_index: usize) -> Vec<(usize, Magnitude<W>)> {
        self.vertices()
            .into_iter()
            .map(|dst_index| (dst_index, self[(src_index, dst_index)]))
            .filter(|(_, weight)| weight.is_finite())
            .collect()
    }

    fn neighbors(&self, src_index: usize) -> Vec<usize> {
        self.vertices()
            .into_iter()
            .filter(|dst_index| self[(src_index, *dst_index)].is_finite())
            .collect()
    }

    fn is_directed(&self) -> bool {
        self.edge_type.is_directed()
    }
}

use std::ops::{Index, IndexMut};
impl<W: Copy + std::any::Any> Index<(usize, usize)> for AdjMatrix<W> {
    type Output = Magnitude<W>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;

        // make sure both vertices denoted by `i` and `j` are present in graph
        match (
            self.reusable_ids.contains(&i),
            self.reusable_ids.contains(&j),
        ) {
            (true, _) => panic!(format!("Vertex with id: {} does not exist", i)),
            (_, true) => panic!(format!("Vertex with id: {} does not exist", j)),
            _ => (),
        }

        let index = utils::from_ij(i, j, self.is_directed());

        if index < self.vec.len() {
            &self.vec[index]
        } else {
            panic!(format!(
                "Index out of bounds: ({i},{j}) does not exist",
                i = i,
                j = j
            ))
        }
    }
}

impl<W: Copy + std::any::Any> IndexMut<(usize, usize)> for AdjMatrix<W> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;

        // make sure both vertices denoted by `i` and `j` are present in graph
        match (
            self.reusable_ids.contains(&i),
            self.reusable_ids.contains(&j),
        ) {
            (true, _) => panic!(format!("Vertex with id: {} does not exist", i)),
            (_, true) => panic!(format!("Vertex with id: {} does not exist", j)),
            _ => (),
        }

        let index = utils::from_ij(i, j, self.is_directed());

        if index < self.vec.len() {
            &mut self.vec[index]
        } else {
            panic!(format!(
                "Index out of bounds: ({i},{j}) does not exist",
                i = i,
                j = j
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_add_vertex() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for i in 0..10 {
            let v_id = adj_matrix.add_vertex();
            assert!(v_id == i);
        }

        assert!(adj_matrix.vertex_count == 10);
        assert!(adj_matrix.vec.len() == 100);
    }

    #[test]
    fn undirected_add_vertex() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for i in 0..10 {
            let v_id = adj_matrix.add_vertex();
            assert!(v_id == i);
        }

        assert!(adj_matrix.vertex_count == 10);
        assert!(adj_matrix.vec.len() == 55);
    }

    #[test]
    fn directed_remove_vertex() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in 0..10 {
            adj_matrix.remove_vertex(i);
            assert!(adj_matrix.vertex_count() == 10 - (i + 1))
        }

        assert!(adj_matrix.vec.iter().all(|weight| weight.is_pos_infinite()));
    }

    #[test]
    fn undirected_remove_vertex() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in 0..10 {
            adj_matrix.remove_vertex(i);
            assert!(adj_matrix.vertex_count() == 10 - (i + 1))
        }

        assert!(adj_matrix.vec.iter().all(|weight| weight.is_pos_infinite()));
    }

    #[test]
    fn directed_add_edge() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix[(v1, v2)] = (v1 * 2 + v2).into();
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                assert_eq!(adj_matrix[(v1, v2)], (v1 * 2 + v2).into());
            }
        }
    }

    #[test]
    fn undirected_add_edge() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix[(v1, v2)] = (v1 * 2 + v2).into();
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                assert_eq!(adj_matrix[(v1, v2)], (v1 * 2 + v2).into());
                assert_eq!(adj_matrix[(v2, v1)], (v1 * 2 + v2).into());
            }
        }
    }

    #[test]
    fn directed_remove_edge() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix[(v1, v2)] = (v1 * 2 + v2).into();
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in adj_matrix.vertices() {
                adj_matrix.remove_edge(v1, v2);
                assert!(adj_matrix[(v1, v2)].is_pos_infinite());
            }
        }
    }

    #[test]
    fn undirected_remove_edge() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for _ in 0..10 {
            let _ = adj_matrix.add_vertex();
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix[(v1, v2)] = (v1 * 2 + v2).into();
            }
        }

        for v1 in adj_matrix.vertices() {
            for v2 in 0..=v1 {
                adj_matrix.remove_edge(v1, v2);
                assert!(adj_matrix[(v1, v2)].is_pos_infinite());
                assert!(adj_matrix[(v2, v1)].is_pos_infinite());
            }
        }
    }

    #[test]
    fn vertices() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

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
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        for i in (0..5).step_by(2) {
            adj_matrix.remove_vertex(i);
        }

        adj_matrix[(1, 3)] = 3.into();
        adj_matrix[(3, 1)] = 4.into();

        for (v1, v2, weight) in adj_matrix.edges() {
            match (v1, v2) {
                (1, 1) => assert!(weight.is_pos_infinite()),
                (1, 3) => assert_eq!(weight, 3.into()),
                (3, 1) => assert_eq!(weight, 4.into()),
                (3, 3) => assert!(weight.is_pos_infinite()),
                _ => panic!("not valid vertices"),
            }
        }
    }

    #[test]
    fn edges_undirected() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        for i in (0..5).step_by(2) {
            adj_matrix.remove_vertex(i);
        }

        adj_matrix[(1, 3)] = 3.into();

        for (v1, v2, weight) in adj_matrix.edges() {
            match (v1, v2) {
                (1, 1) => assert!(weight.is_pos_infinite()),
                (1, 3) => assert_eq!(weight, 3.into()),
                (3, 1) => assert_eq!(weight, 3.into()),
                (3, 3) => assert!(weight.is_pos_infinite()),
                _ => panic!("not valid vertices"),
            }
        }
    }

    #[test]
    fn neighbors_directed() {
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Directed);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        adj_matrix[(1, 3)] = 3.into();
        adj_matrix[(1, 2)] = 2.into();
        adj_matrix[(4, 0)] = 1.into();

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
        let mut adj_matrix = AdjMatrix::<usize>::init(EdgeType::Undirected);

        for _ in 0..5 {
            let _ = adj_matrix.add_vertex();
        }

        adj_matrix[(1, 3)] = 3.into();
        adj_matrix[(4, 0)] = 1.into();

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
