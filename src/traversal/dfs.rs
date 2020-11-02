use magnitude::Magnitude;
use std::cell::{Ref, RefCell};

use crate::provide;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Gray,
    Black,
}

pub struct Dfs {
    stack: RefCell<Vec<usize>>,
    colors: RefCell<Vec<Color>>,
    discovered: RefCell<Vec<Magnitude<usize>>>,
    finished: RefCell<Vec<Magnitude<usize>>>,
    time: RefCell<usize>,
    id_map: RefCell<provide::IdMap>,
    start_ids: RefCell<Vec<usize>>,
}

impl Dfs {
    pub fn init<G>(graph: &G) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        Dfs::init_with(graph, vec![])
    }

    pub fn init_with<G>(graph: &G, start_ids: Vec<usize>) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        let vertex_count = graph.vertex_count();

        Dfs {
            stack: RefCell::new(vec![]),
            colors: RefCell::new(vec![Color::White; vertex_count]),
            discovered: RefCell::new(vec![Magnitude::PosInfinite; vertex_count]),
            finished: RefCell::new(vec![Magnitude::PosInfinite; vertex_count]),
            time: RefCell::new(0),
            id_map: RefCell::new(graph.continuos_id_map()),
            start_ids: RefCell::new(start_ids),
        }
    }

    fn next_start_id(&self) -> Option<usize> {
        if self.start_ids.borrow().is_empty() {
            for (virt_start_id, &color) in self.colors.borrow().iter().enumerate() {
                if color == Color::White {
                    return Some(virt_start_id);
                }
            }

            None
        } else {
            self.start_ids.borrow_mut().pop()
        }
    }

    fn next_virt_id(&self) -> Option<usize> {
        self.stack.borrow_mut().pop()
    }

    pub fn execute<G>(
        &self,
        graph: &G,
        mut on_start: impl FnMut(usize),
        mut on_white: impl FnMut(usize),
        mut on_gray: impl FnMut(usize),
        mut on_black: impl FnMut(usize),
    ) where
        G: provide::Vertices + provide::Neighbors,
    {
        while let Some(virt_start_id) = self.next_start_id() {
            if self.colors.borrow()[virt_start_id] != Color::White {
                continue;
            }

            // On start.
            *self.time.borrow_mut() = 0;
            self.stack.borrow_mut().push(virt_start_id);
            on_start(virt_start_id);

            while let Some(virt_id) = self.next_virt_id() {
                let color = self.colors.borrow()[virt_id];
                match color {
                    Color::White => {
                        println!(
                            "vertex {} with color: {:?}",
                            self.id_map.borrow().get_virt_to_real(virt_id).unwrap(),
                            color
                        );
                        *self.time.borrow_mut() += 1;
                        self.discovered.borrow_mut()[virt_id] = (*self.time.borrow()).into();

                        // On white.
                        on_white(virt_id);

                        self.colors.borrow_mut()[virt_id] = Color::Gray;

                        let real_id = self.id_map.borrow().get_virt_to_real(virt_id).unwrap();

                        let mut undiscovered_neighbors = graph
                            .neighbors(real_id)
                            .into_iter()
                            .filter(|n_id| self.colors.borrow()[*n_id] == Color::White)
                            .map(|real_id| self.id_map.borrow().get_real_to_virt(real_id).unwrap())
                            .collect::<Vec<usize>>();

                        self.stack.borrow_mut().push(virt_id);
                        self.stack.borrow_mut().append(&mut undiscovered_neighbors);
                    }
                    Color::Gray => {
                        // On gray.
                        on_gray(virt_id);

                        // On black.
                        self.colors.borrow_mut()[virt_id] = Color::Black;
                        *self.time.borrow_mut() += 1;
                        self.finished.borrow_mut()[virt_id] = (*self.time.borrow()).into();
                        on_black(virt_id);
                    }
                    Color::Black => {}
                };
            }
        }
    }

    pub fn get_stack(&self) -> Ref<'_, Vec<usize>> {
        self.stack.borrow()
    }

    pub fn get_colors(&self) -> Ref<'_, Vec<Color>> {
        self.colors.borrow()
    }

    pub fn get_discovered(&self) -> Ref<'_, Vec<Magnitude<usize>>> {
        self.discovered.borrow()
    }

    pub fn get_finished(&self) -> Ref<'_, Vec<Magnitude<usize>>> {
        self.finished.borrow()
    }

    pub fn get_id_map(&self) -> Ref<'_, provide::IdMap> {
        self.id_map.borrow()
    }

    pub fn get_time(&self) -> usize {
        *self.time.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn empty_directed_graph() {
        // Given: An empty directed graph.
        let graph = MatGraph::init(Mat::<usize>::init(true));

        // When: traversing the graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| on_start_called += 1,
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 0);
        assert_eq!(on_white_called, 0);
        assert_eq!(on_gray_called, 0);
        assert_eq!(on_black_called, 0);
    }

    #[test]
    fn empty_undirected_graph() {
        // Given: An empty undirected graph.
        let graph = MatGraph::init(Mat::<usize>::init(false));

        // When: traversing the graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| on_start_called += 1,
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 0);
        assert_eq!(on_white_called, 0);
        assert_eq!(on_gray_called, 0);
        assert_eq!(on_black_called, 0);
    }

    #[test]
    fn directed_graph_with_one_vertex() {
        // Given: A directed graph: a.
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let _ = graph.add_vertex();

        // When: traversing graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 1);
        assert_eq!(on_gray_called, 1);
        assert_eq!(on_black_called, 1);
    }

    #[test]
    fn undirected_graph_with_one_vertex() {
        // Given: An undirected graph: a.
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let _ = graph.add_vertex();

        // When: traversing graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 1);
        assert_eq!(on_gray_called, 1);
        assert_eq!(on_black_called, 1);
    }

    #[test]
    fn directed_graph_with_two_separate_vertices() {
        // Given: A directed graph: a  b.
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let _ = graph.add_vertex();
        let _ = graph.add_vertex();

        // When: traversing graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 2);
        assert_eq!(on_white_called, 2);
        assert_eq!(on_gray_called, 2);
        assert_eq!(on_black_called, 2);
    }
    #[test]
    fn undirected_graph_with_two_separate_vertices() {
        // Given: An undirected graph: a  b.
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let _ = graph.add_vertex();
        let _ = graph.add_vertex();

        // When: traversing graph.
        let dfs = Dfs::init(&graph);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 2);
        assert_eq!(on_white_called, 2);
        assert_eq!(on_gray_called, 2);
        assert_eq!(on_black_called, 2);
    }

    #[test]
    fn directed_graph_with_three_vertices_in_a_line() {
        // Given: A directed graph: a -> b -> c.
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 3);
        assert_eq!(on_gray_called, 3);
        assert_eq!(on_black_called, 3);
    }

    #[test]
    fn undirected_graph_with_three_vertices_in_a_line() {
        // Given: An undirected graph: a -- b -- c.
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![c]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 3);
        assert_eq!(on_gray_called, 3);
        assert_eq!(on_black_called, 3);
    }

    #[test]
    fn directed_graph_with_cycle() {
        // Given: A directed graph: a -> b -> c.
        //                          ^         |
        //                          '---------
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 3);
        assert_eq!(on_gray_called, 3);
        assert_eq!(on_black_called, 3);
    }

    #[test]
    fn undirected_graph_with_cycle() {
        // Given: An undirected graph: a -- b -- c.
        //                             |         |
        //                             -----------
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 3);
        assert_eq!(on_gray_called, 3);
        assert_eq!(on_black_called, 3);
    }

    #[test]
    fn trivial_directed_graph() {
        // Given: A directed graph:
        //                           _____________________
        //                          |                    |
        //                          |                    v
        //                          a -> b -> c -> d -> f
        //                          ^         |     |
        //                          '---------      v
        //                                          e
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, f, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());
        graph.add_edge(c, d, 1.into());
        graph.add_edge(d, e, 1.into());
        graph.add_edge(d, f, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 6);
        assert_eq!(on_gray_called, 6);
        assert_eq!(on_black_called, 6);
    }

    #[test]
    fn trivial_undirected_graph() {
        // Given: An undirected graph:
        //                          ----------------------
        //                          |                    |
        //                          a -- b -- c -- d -- f
        //                          |         |    |
        //                          '---------     e
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, f, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());
        graph.add_edge(c, d, 1.into());
        graph.add_edge(d, e, 1.into());
        graph.add_edge(d, f, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 1);
        assert_eq!(on_white_called, 6);
        assert_eq!(on_gray_called, 6);
        assert_eq!(on_black_called, 6);
    }

    #[test]
    fn directed_graph_of_two_forests() {
        // Given: A directed graph:
        //                          a -> b -> c     d -> f
        //                          ^         |     |
        //                          '---------      v
        //                                          e
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        graph.add_edge(d, e, 1.into());
        graph.add_edge(d, f, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a, d]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 2);
        assert_eq!(on_white_called, 6);
        assert_eq!(on_gray_called, 6);
        assert_eq!(on_black_called, 6);
    }

    #[test]
    fn undirected_graph_of_two_forests() {
        // Given: An undirected graph:
        //                          a -- b -- c    d -- f
        //                          |         |    |
        //                          '---------     e
        let mut graph = MatGraph::init(Mat::<usize>::init(false));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());

        graph.add_edge(d, e, 1.into());
        graph.add_edge(d, f, 1.into());

        // When: traversing graph.
        let dfs = Dfs::init_with(&graph, vec![a, d]);

        let mut on_start_called = 0;
        let mut on_white_called = 0;
        let mut on_gray_called = 0;
        let mut on_black_called = 0;

        dfs.execute(
            &graph,
            |_| {
                println!("start");
                on_start_called += 1;
                assert!(dfs.get_time() == 0);
            },
            |_| on_white_called += 1,
            |_| on_gray_called += 1,
            |_| on_black_called += 1,
        );

        // Then:
        assert_eq!(on_start_called, 2);
        assert_eq!(on_white_called, 6);
        assert_eq!(on_gray_called, 6);
        assert_eq!(on_black_called, 6);
    }
}
