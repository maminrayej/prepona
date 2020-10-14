mod utils;

use magnitude::Magnitude;
use std::collections::HashSet;

pub struct AdjMatrix<W> {
    vec: Vec<Magnitude<W>>,
    reusable_ids: HashSet<usize>,
    vertex_count: usize,
}

impl<W> AdjMatrix<W> {
    pub fn init() -> Self {
        AdjMatrix {
            vec: vec![],
            reusable_ids: HashSet::new(),
            vertex_count: 0,
        }
    }

    pub fn add_vertex(&mut self) -> usize {
        if let Some(reusable_id) = self.next_reusable_id() {
            reusable_id
        } else {
            let new_size = self.vec.len() + 2 * self.total_vertex_count() + 1;

            self.vec.resize_with(new_size, || Magnitude::PosInfinite);

            self.vertex_count += 1;

            self.vertex_count - 1
        }
    }

    pub fn remove_vertex(&mut self, vertex_id: usize) {
        self.reusable_ids.insert(vertex_id);

        for other_id in 0..self.total_vertex_count() {
            self[(vertex_id, other_id)] = Magnitude::PosInfinite;
            self[(other_id, vertex_id)] = Magnitude::PosInfinite;
        }

        self.vertex_count -= 1;
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn total_vertex_count(&self) -> usize {
        self.vertex_count() + self.reusable_ids.len()
    }

    fn next_reusable_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_ids.iter().take(1).next().copied() {
            self.reusable_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }
}

use std::ops::{Index, IndexMut};
impl<W> Index<(usize, usize)> for AdjMatrix<W> {
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

        let index = utils::from_ij(i, j);

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

impl<W> IndexMut<(usize, usize)> for AdjMatrix<W> {
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

        let index = utils::from_ij(i, j);

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
    fn add_vertex() {
        let mut adj_matrix = AdjMatrix::<usize>::init();

        for i in 0usize..10 {
            let vertex_id = adj_matrix.add_vertex();
            assert_eq!(i, vertex_id);
        }

        assert_eq!(adj_matrix.vertex_count(), 10);
        assert_eq!(adj_matrix.vec.len(), 100);
    }

    #[test]
    fn access_using_index() {
        let mut adj_matrix = AdjMatrix::<usize>::init();

        for _ in 0usize..10 {
            let _ = adj_matrix.add_vertex();
        }

        for i in 0..adj_matrix.vertex_count() {
            for j in 0..adj_matrix.vertex_count() {
                assert!(adj_matrix[(i, j)].is_pos_infinite())
            }
        }
    }
}
