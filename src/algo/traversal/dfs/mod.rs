mod listener;

pub use listener::DfsListener;

use magnitude::Magnitude;
use std::cell::{Ref, RefCell};

use super::Color;
use crate::provide;

pub struct Dfs<'a, L: DfsListener> {
    stack: RefCell<Vec<usize>>,
    colors: RefCell<Vec<Color>>,
    discovered: RefCell<Vec<Magnitude<usize>>>,
    finished: RefCell<Vec<Magnitude<usize>>>,
    time: RefCell<usize>,
    id_map: RefCell<provide::IdMap>,
    start_ids: RefCell<Vec<usize>>,
    listener: RefCell<&'a mut L>,
}

impl<'a, L: DfsListener> Dfs<'a, L> {
    pub fn init<G>(graph: &G, listener: &'a mut L) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        Dfs::init_with_starts(graph, listener, vec![])
    }

    pub fn init_with_starts<G>(graph: &G, listener: &'a mut L, mut start_ids: Vec<usize>) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        let vertex_count = graph.vertex_count();

        start_ids.reverse();

        Dfs {
            stack: RefCell::new(vec![]),
            colors: RefCell::new(vec![Color::White; vertex_count]),
            discovered: RefCell::new(vec![Magnitude::PosInfinite; vertex_count]),
            finished: RefCell::new(vec![Magnitude::PosInfinite; vertex_count]),
            time: RefCell::new(0),
            id_map: RefCell::new(graph.continuos_id_map()),
            start_ids: RefCell::new(start_ids),
            listener: RefCell::new(listener),
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

    pub fn execute<G>(&self, graph: &G)
    where
        G: provide::Vertices + provide::Neighbors,
    {
        while let Some(virt_start_id) = self.next_start_id() {
            if self.colors.borrow()[virt_start_id] != Color::White {
                continue;
            }

            // On start.
            *self.time.borrow_mut() = 0;
            self.stack.borrow_mut().push(virt_start_id);
            self.listener.borrow_mut().on_start(&self, virt_start_id);

            while let Some(virt_id) = self.next_virt_id() {
                let color = self.colors.borrow()[virt_id];
                match color {
                    Color::White => {
                        *self.time.borrow_mut() += 1;
                        self.discovered.borrow_mut()[virt_id] = (*self.time.borrow()).into();

                        // On white.
                        self.listener.borrow_mut().on_white(&self, virt_id);

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
                        self.listener.borrow_mut().on_gray(&self, virt_id);

                        // On black.
                        self.colors.borrow_mut()[virt_id] = Color::Black;
                        *self.time.borrow_mut() += 1;
                        self.finished.borrow_mut()[virt_id] = (*self.time.borrow()).into();
                        self.listener.borrow_mut().on_black(&self, virt_id);
                    }
                    Color::Black => {}
                };
            }

            self.listener.borrow_mut().on_finish(&self);
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

    pub fn id_map(self) -> provide::IdMap {
        self.id_map.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::{DiMat, Mat};

    struct DefaultListener {
        pub on_start_called: usize,
        pub on_white_called: usize,
        pub on_gray_called: usize,
        pub on_black_called: usize,
        pub on_finish_called: usize,
    }

    impl DefaultListener {
        fn init() -> Self {
            DefaultListener {
                on_start_called: 0,
                on_white_called: 0,
                on_gray_called: 0,
                on_black_called: 0,
                on_finish_called: 0,
            }
        }
    }

    impl DfsListener for DefaultListener {
        fn on_start(&mut self, _: &Dfs<Self>, _: usize) {
            self.on_start_called += 1;
        }

        fn on_white(&mut self, _: &Dfs<Self>, _: usize) {
            self.on_white_called += 1;
        }

        fn on_gray(&mut self, _: &Dfs<Self>, _: usize) {
            self.on_gray_called += 1;
        }

        fn on_black(&mut self, _: &Dfs<Self>, _: usize) {
            self.on_black_called += 1;
        }

        fn on_finish(&mut self, _: &Dfs<Self>) {
            self.on_finish_called += 1;
        }
    }

    #[test]
    fn empty_directed_graph() {
        // Given: An empty directed graph.
        let graph = MatGraph::init(DiMat::<usize>::init());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init(&graph, &mut listener);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 0);
        assert_eq!(listener.on_white_called, 0);
        assert_eq!(listener.on_gray_called, 0);
        assert_eq!(listener.on_black_called, 0);
        assert_eq!(listener.on_finish_called, 0);
    }

    #[test]
    fn empty_undirected_graph() {
        // Given: An empty undirected graph.
        let graph = MatGraph::init(Mat::<usize>::init());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init(&graph, &mut listener);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 0);
        assert_eq!(listener.on_white_called, 0);
        assert_eq!(listener.on_gray_called, 0);
        assert_eq!(listener.on_black_called, 0);
        assert_eq!(listener.on_finish_called, 0);
    }

    #[test]
    fn single_vertex_directed_graph() {
        // Given: A graph with single vertex
        //
        //      a
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        graph.add_vertex();

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init(&graph, &mut listener);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 1);
        assert_eq!(listener.on_gray_called, 1);
        assert_eq!(listener.on_black_called, 1);
        assert_eq!(listener.on_finish_called, 1);
    }

    #[test]
    fn single_vertex_undirected_graph() {
        // Given: A graph with single vertex
        //
        //      a
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        graph.add_vertex();

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init(&graph, &mut listener);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 1);
        assert_eq!(listener.on_gray_called, 1);
        assert_eq!(listener.on_black_called, 1);
        assert_eq!(listener.on_finish_called, 1);
    }

    #[test]
    fn trivial_directed_graph() {
        // Given: Graph
        //
        //      a  -->  b  -->  d  -->  e
        //      ^       |               |
        //      |       v               v
        //      '______ c               f
        //
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());
        graph.add_edge(b, d, 1.into());
        graph.add_edge(d, e, 1.into());
        graph.add_edge(e, f, 1.into());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 6);
        assert_eq!(listener.on_gray_called, 6);
        assert_eq!(listener.on_black_called, 6);
        assert_eq!(listener.on_finish_called, 1);
    }

    #[test]
    fn trivial_undirected_graph() {
        // Given: Graph
        //
        //      a  ---  b  ---  d  ---  e
        //      |       |               |
        //      '______ c               f
        //
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(c, a, 1.into());
        graph.add_edge(b, d, 1.into());
        graph.add_edge(d, e, 1.into());
        graph.add_edge(e, f, 1.into());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 6);
        assert_eq!(listener.on_gray_called, 6);
        assert_eq!(listener.on_black_called, 6);
        assert_eq!(listener.on_finish_called, 1);
    }

    #[test]
    fn not_strongly_connected_directed_graph() {
        // Given: Graph
        //
        //      a  -->  b       d  -->  e
        //              |       |
        //              v       v
        //              c       f
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(d, f, 1.into());
        graph.add_edge(d, e, 1.into());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 2);
        assert_eq!(listener.on_white_called, 6);
        assert_eq!(listener.on_gray_called, 6);
        assert_eq!(listener.on_black_called, 6);
        assert_eq!(listener.on_finish_called, 2);
    }

    #[test]
    fn not_connected_undirected_graph() {
        // Given: Graph
        //
        //      a  ---  b       d  ---  e
        //              |       |
        //              c       f
        let mut graph = MatGraph::init(Mat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex();
        let e = graph.add_vertex();
        let f = graph.add_vertex();

        graph.add_edge(a, b, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(d, f, 1.into());
        graph.add_edge(d, e, 1.into());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 2);
        assert_eq!(listener.on_white_called, 6);
        assert_eq!(listener.on_gray_called, 6);
        assert_eq!(listener.on_black_called, 6);
        assert_eq!(listener.on_finish_called, 2);
    }

    #[test]
    fn trivial_directed_graph_2() {
        // Given: Graph
        //
        //      a  -->  b  -->  c
        //      |       |
        //      v       v
        //      d  -->  e
        let mut graph = MatGraph::init(DiMat::<usize>::init());
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        let d = graph.add_vertex(); // 3
        let e = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, d, 1.into());
        graph.add_edge(b, c, 1.into());
        graph.add_edge(b, e, 1.into());
        graph.add_edge(d, e, 1.into());

        // When: Performing DFS algorithm.
        let mut listener = DefaultListener::init();
        let dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 5);
        assert_eq!(listener.on_gray_called, 5);
        assert_eq!(listener.on_black_called, 5);
        assert_eq!(listener.on_finish_called, 1);
    }
}
