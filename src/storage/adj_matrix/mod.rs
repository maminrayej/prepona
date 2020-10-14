mod utils;

use magnitude::Magnitude;
pub struct AdjMatrix<W> {
    vec: Vec<Magnitude<W>>,
    vertex_count: usize,
}

impl<W> AdjMatrix<W> {
    pub fn init() -> Self {
        AdjMatrix {
            vec: vec![],
            vertex_count: 0,
        }
    }

    pub fn add_node(&mut self) -> usize {
        let new_size = self.vec.len() + 2 * self.vertex_count + 1;

        self.vec.resize_with(new_size, || Magnitude::PosInfinite);

        self.vertex_count += 1;

        self.vertex_count - 1
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }
}

use std::ops::Index;
impl<W> Index<(usize, usize)> for AdjMatrix<W> {
    type Output = Magnitude<W>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node() {
        let mut adj_matrix = AdjMatrix::<usize>::init();

        for i in 0usize..10 {
            let node_id = adj_matrix.add_node();
            assert_eq!(i, node_id);
        }

        assert_eq!(adj_matrix.vertex_count(), 10);
        assert_eq!(adj_matrix.vec.len(), 100);
    }

    #[test]
    fn access_using_index() {
        let mut adj_matrix = AdjMatrix::<usize>::init();

        for _ in 0usize..10 {
            let _ = adj_matrix.add_node();
        }

        for i in 0..adj_matrix.vertex_count() {
            for j in 0..adj_matrix.vertex_count() {
                assert!(adj_matrix[(i, j)].is_pos_infinite())
            }
        }
    }
}
