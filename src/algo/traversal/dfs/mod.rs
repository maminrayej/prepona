mod listener;

pub use listener::DfsListener;

use magnitude::Magnitude;
use std::cell::RefCell;

use super::Color;
use crate::provide::{self, IdMap};

pub struct Dfs<'a, L: DfsListener> {
    stack: Vec<usize>,
    colors: Vec<Color>,
    discovered: Vec<Magnitude<usize>>,
    finished: Vec<Magnitude<usize>>,
    time: usize,
    id_map: IdMap,
    start_ids: Vec<usize>,
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

        let id_map = graph.continuos_id_map();

        start_ids = start_ids
            .into_iter()
            .map(|real_id| id_map.virt_id_of(real_id))
            .collect();

        Dfs {
            stack: vec![],
            colors: vec![Color::White; vertex_count],
            discovered: vec![Magnitude::PosInfinite; vertex_count],
            finished: vec![Magnitude::PosInfinite; vertex_count],
            time: 0,
            id_map: graph.continuos_id_map(),
            listener: RefCell::new(listener),
            start_ids,
        }
    }

    fn next_start_id(&self) -> Option<usize> {
        if self.start_ids.is_empty() {
            self.colors.iter().position(|color| *color == Color::White)
        } else {
            self.start_ids
                .iter()
                .find(|virt_id| self.colors[**virt_id] == Color::White)
                .and_then(|virt_id| Some(*virt_id))
        }
    }

    pub fn execute<G>(&mut self, graph: &G)
    where
        G: provide::Vertices + provide::Neighbors,
    {
        while let Some(start_id) = self.next_start_id() {
            self.time += 1;
            self.stack.push(start_id);
            self.listener.borrow_mut().on_start(self, start_id);

            while let Some(virt_id) = self.stack.pop() {
                let color = self.colors[virt_id];

                match color {
                    Color::White => {
                        self.time += 1;
                        self.discovered[virt_id] = self.time.into();
                        self.listener.borrow_mut().on_white(self, virt_id);

                        self.colors[virt_id] = Color::Gray;

                        let real_id = self.id_map.real_id_of(virt_id);

                        let mut neighbors = graph
                            .neighbors(real_id)
                            .into_iter()
                            .map(|real_id| self.id_map.virt_id_of(real_id))
                            .filter(|virt_id| self.colors[*virt_id] == Color::White)
                            .collect();

                        self.stack.push(virt_id);
                        self.stack.append(&mut neighbors);
                    }
                    Color::Gray => {
                        self.listener.borrow_mut().on_gray(self, virt_id);

                        self.colors[virt_id] = Color::Black;
                        self.time += 1;
                        self.finished[virt_id] = self.time.into();
                        self.listener.borrow_mut().on_black(self, virt_id);
                    }
                    Color::Black => {}
                }
            }
            self.listener.borrow_mut().on_finish(self);
        }
    }

    pub fn get_stack(&self) -> &Vec<usize> {
        &self.stack
    }

    pub fn get_colors(&self) -> &Vec<Color> {
        &self.colors
    }

    pub fn get_discovered(&self) -> &Vec<Magnitude<usize>> {
        &self.discovered
    }

    pub fn get_finished(&self) -> &Vec<Magnitude<usize>> {
        &self.finished
    }

    pub fn get_id_map(&self) -> &IdMap {
        &self.id_map
    }

    pub fn dissolve(self) -> (Vec<Magnitude<usize>>, Vec<Magnitude<usize>>, IdMap) {
        (self.discovered, self.finished, self.id_map)
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init(&graph, &mut listener);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init(&graph, &mut listener);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init(&graph, &mut listener);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init(&graph, &mut listener);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a]);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a]);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
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

        // When: Performing Dfs algorithm.
        let mut listener = DefaultListener::init();
        let mut dfs = Dfs::init_with_starts(&graph, &mut listener, vec![a, d]);
        dfs.execute(&graph);

        // Then:
        assert_eq!(listener.on_start_called, 1);
        assert_eq!(listener.on_white_called, 5);
        assert_eq!(listener.on_gray_called, 5);
        assert_eq!(listener.on_black_called, 5);
        assert_eq!(listener.on_finish_called, 1);
    }
}
