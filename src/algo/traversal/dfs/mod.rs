use std::marker::PhantomData;

use crate::provide::{Direction, NodeId, NodeIdMapProvider, NodeProvider};

const UNKNOWN: usize = usize::MAX;

macro_rules! callback {
    ($cond: expr, $body: block) => {
        match $cond {
            ControlFlow::Prune => continue,
            ControlFlow::Return => return,
            ControlFlow::Continue => $body,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Prune,
    Return,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    ForwardEdge,
    TreeEdge,
    BackEdge,
    CrossEdge,
}

pub enum Event<'a, G: NodeIdMapProvider> {
    Begin(&'a State<G>, NodeId),
    Discover(&'a State<G>, NodeId),
    Finish(&'a State<G>, NodeId),
    End(&'a State<G>),
    VisitEdge(&'a State<G>, EdgeType, NodeId, NodeId),
}

pub struct State<G: NodeIdMapProvider> {
    pub id_map: <G as NodeIdMapProvider>::NodeIdMap,

    pub time: usize,
    pub discover: Vec<usize>,
    pub finished: Vec<usize>,
    pub parent: Vec<usize>,

    pub(crate) phantom_g: PhantomData<G>,
}

pub struct DFS<'a, G>
where
    G: NodeProvider + NodeIdMapProvider,
{
    graph: &'a G,
    starting_nodes: Vec<NodeId>,
    state: State<G>,
}

impl<'a, G> DFS<'a, G>
where
    G: NodeProvider + NodeIdMapProvider,
{
    pub fn init(graph: &'a G) -> Self {
        Self::with_starters(graph, None.into_iter())
    }

    pub fn with_starters(graph: &'a G, starting_nodes: impl Iterator<Item = NodeId>) -> Self {
        let node_count = graph.node_count();

        DFS {
            graph,
            starting_nodes: starting_nodes.collect(),
            state: State {
                id_map: graph.id_map(),

                time: 0,
                discover: vec![UNKNOWN; node_count],
                finished: vec![UNKNOWN; node_count],
                parent: vec![UNKNOWN; node_count],

                phantom_g: PhantomData,
            },
        }
    }

    fn next_start_node(&mut self) -> Option<NodeId> {
        if !self.starting_nodes.is_empty() {
            Some(self.starting_nodes.swap_remove(0))
        } else {
            self.state
                .discover
                .iter()
                .position(|time| *time == UNKNOWN)
                .map(|index| self.state.id_map[index])
        }
    }

    fn edge_type_of(&self, src_vid: usize, dst_vid: usize) -> EdgeType {
        if !self.is_discovered(dst_vid) {
            EdgeType::TreeEdge
        } else if self.is_discovered(dst_vid) && !self.is_finished(dst_vid) {
            eprintln!("Found back edge from {src_vid} to {dst_vid}");
            EdgeType::BackEdge
        } else if self.discover_of(src_vid) < self.discover_of(dst_vid) {
            EdgeType::ForwardEdge
        } else {
            EdgeType::CrossEdge
        }
    }

    fn discover_of(&self, vid: usize) -> usize {
        self.state.discover[vid]
    }

    fn is_discovered(&self, vid: usize) -> bool {
        self.discover_of(vid) != UNKNOWN
    }

    fn finished_of(&self, vid: usize) -> usize {
        self.state.finished[vid]
    }

    fn is_finished(&self, vid: usize) -> bool {
        self.finished_of(vid) != UNKNOWN
    }

    fn parent_of(&self, rid: usize) -> Option<usize> {
        self.state.parent.get(rid).copied()
    }

    pub fn execute(mut self, mut callback: impl FnMut(Event<G>) -> ControlFlow) {
        while let Some(start_node) = self.next_start_node() {
            callback!(callback(Event::Begin(&self.state, start_node)), {
                let start_vid = self.state.id_map[start_node];
                self.state.time += 1;
                self.state.discover[start_vid] = self.state.time;

                callback!(callback(Event::Discover(&self.state, start_node)), {
                    let mut stack =
                        vec![(start_node, start_vid, 0, self.graph.successors(start_node))];

                    while let Some((src_node, src_vid, depth, mut successors)) = stack.pop() {
                        eprintln!("Visiting node {src_vid}");
                        if let Some(dst_node) = successors.next() {
                            let dst_vid = self.state.id_map[dst_node];

                            if G::Dir::is_undirected()
                                && (self.parent_of(src_vid) == Some(dst_vid)
                                    || self.is_finished(dst_vid))
                            {
                                stack.push((src_node, src_vid, depth, successors));
                                continue;
                            }

                            let edge_type = self.edge_type_of(src_vid, dst_vid);

                            match edge_type {
                                EdgeType::TreeEdge => {
                                    eprintln!("Found tree edge from {src_vid} to {dst_vid}");
                                    self.state.parent[dst_vid] = src_vid;

                                    stack.push((src_node, src_vid, depth, successors));

                                    self.state.time += 1;
                                    self.state.discover[dst_vid] = self.state.time;

                                    callback!(callback(Event::Discover(&self.state, dst_node)), {
                                        stack.push((
                                            dst_node,
                                            dst_vid,
                                            depth + 1,
                                            self.graph.successors(dst_node),
                                        ));
                                    });
                                }
                                _ => {
                                    stack.push((src_node, src_vid, depth, successors));
                                }
                            }

                            if callback(Event::VisitEdge(
                                &self.state,
                                edge_type,
                                src_node,
                                dst_node,
                            )) == ControlFlow::Return
                            {
                                return;
                            }
                        } else {
                            self.state.time += 1;
                            self.state.finished[src_vid] = self.state.time;

                            if callback(Event::Finish(&self.state, src_node)) == ControlFlow::Return
                            {
                                return;
                            }
                        }
                    }
                });
            });

            if callback(Event::End(&self.state)) == ControlFlow::Return {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::{
        CompleteGraph, CycleGraph, EmptyGraph, Generator, LadderGraph, LollipopGraph, NullGraph,
        PathGraph, StarGraph,
    };
    use crate::provide::{EdgeProvider, NodeIdMapProvider, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::ControlFlow::Continue;
    use super::{EdgeType, Event, DFS};

    struct EventCounter {
        pub begin_called: usize,
        pub discover_called: usize,
        pub finish_called: usize,
        pub end_called: usize,

        pub tree_edge_found: usize,
        pub forward_edge_found: usize,
        pub cross_edge_found: usize,
        pub back_edge_found: usize,
    }

    impl EventCounter {
        pub fn init() -> Self {
            EventCounter {
                begin_called: 0,
                discover_called: 0,
                finish_called: 0,
                end_called: 0,
                tree_edge_found: 0,
                forward_edge_found: 0,
                cross_edge_found: 0,
                back_edge_found: 0,
            }
        }

        pub fn count<G: NodeIdMapProvider>(&mut self, event: Event<G>) {
            match event {
                Event::Begin(_, _) => self.begin_called += 1,
                Event::Discover(_, _) => self.discover_called += 1,
                Event::Finish(_, _) => self.finish_called += 1,
                Event::End(_) => self.end_called += 1,
                Event::VisitEdge(_, edge_type, _, _) => match edge_type {
                    EdgeType::ForwardEdge => self.forward_edge_found += 1,
                    EdgeType::TreeEdge => self.tree_edge_found += 1,
                    EdgeType::BackEdge => self.back_edge_found += 1,
                    EdgeType::CrossEdge => self.cross_edge_found += 1,
                },
            }
        }
    }

    #[quickcheck]
    fn dfs_on_null_graph(generator: NullGraph) {
        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 0);
        assert_eq!(event_counter.discover_called, 0);
        assert_eq!(event_counter.finish_called, 0);
        assert_eq!(event_counter.end_called, 0);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 0);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_empty_graph(generator: EmptyGraph) {
        let node_count = generator.node_count;

        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, node_count);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, node_count);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 0);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_cycle_graph(generator: CycleGraph) {
        let node_count = generator.node_count;
        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, node_count - 1);
        assert_eq!(event_counter.back_edge_found, 1);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_path_graph(generator: PathGraph) {
        let node_count = generator.node_count;
        if node_count == 0 {
            return;
        }

        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, node_count - 1);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_star_graph(generator: StarGraph) {
        let node_count = generator.node_count;

        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, node_count - 1);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_complete_graph(generator: CompleteGraph) {
        let node_count = generator.node_count;

        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, node_count - 1);
        assert_eq!(
            event_counter.back_edge_found,
            graph.edge_count() - event_counter.tree_edge_found
        );
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_lollipop_graph(generator: LollipopGraph) {
        let complete_graph_size = generator.complete_graph_size;
        let path_graph_size = generator.path_graph_size;
        let total_node_count = complete_graph_size + path_graph_size;

        let graph: AdjMap<Undirected> = generator.generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, total_node_count);
        assert_eq!(event_counter.finish_called, total_node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, total_node_count - 1);
        assert_eq!(
            event_counter.back_edge_found,
            (complete_graph_size * (complete_graph_size - 1)) / 2 - (complete_graph_size - 1)
        );
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[quickcheck]
    fn dfs_on_ladder_graph(generator: LadderGraph) {
        let graph: AdjMap<Undirected> = generator.generate();
        let node_count = graph.node_count();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, node_count);
        assert_eq!(event_counter.finish_called, node_count);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, node_count - 1);
        assert_eq!(event_counter.back_edge_found, (node_count - 2) / 2);
        assert_eq!(event_counter.cross_edge_found, 0);
    }
}
