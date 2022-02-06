mod misc;

pub use misc::*;

use crate::common::{RealID, VirtID};
use crate::provide::{Storage, Vertices};
use crate::storage::edge::Direction;
use itertools::Itertools;
use magnitude::Magnitude::{self, PosInfinite};
use std::collections::HashMap;

macro_rules! callback {
    ($e: expr, $b: block) => {
        match $e {
            Prune => continue,
            Return => return,
            Continue => $b,
        }
    };
}

pub struct DFS<'a, G>
where
    G: Storage + Vertices,
{
    graph: &'a G,
    starting_vertex_ids: Vec<usize>,

    state: State,
}

impl<'a, G> DFS<'a, G>
where
    G: Storage + Vertices,
{
    //--- Init functions
    pub fn init(graph: &'a G) -> Self {
        Self::init_with(graph, None.into_iter())
    }

    pub fn init_with(graph: &'a G, starting_vertices: impl Iterator<Item = usize>) -> Self {
        let vertex_count = graph.vertex_count();

        DFS {
            graph,
            starting_vertex_ids: starting_vertices.collect_vec(),
            state: State {
                id_map: graph.id_map(),

                time: 0,
                discover: vec![PosInfinite; vertex_count],
                finished: vec![PosInfinite; vertex_count],
                parent: HashMap::new(),
            },
        }
    }

    //--- Utility functions
    fn next_start_vertex_id(&mut self) -> Option<RealID> {
        if !self.starting_vertex_ids.is_empty() {
            Some(RealID::from(self.starting_vertex_ids.swap_remove(0)))
        } else {
            self.state
                .discover
                .iter()
                .position(|time| time.is_pos_infinite())
                .map(|index| self.state.id_map[VirtID::from(index)])
        }
    }

    fn edge_type_of(&self, src_vid: VirtID, dst_vid: VirtID) -> EdgeType {
        use misc::EdgeType::*;

        let src_rid = self.state.id_map[src_vid];
        let dst_rid = self.state.id_map[dst_vid];

        if !self.is_discovered(dst_vid) {
            TreeEdge(src_rid, dst_rid)
        } else if self.is_discovered(dst_vid) && !self.is_finished(dst_vid) {
            BackEdge(src_rid, dst_rid)
        } else {
            if self.discover_of(src_vid) < self.discover_of(dst_vid) {
                ForwardEdge(src_rid, dst_rid)
            } else {
                CrossEdge(src_rid, dst_rid)
            }
        }
    }

    //--- State query functions
    fn discover_of(&self, vid: VirtID) -> Magnitude<usize> {
        self.state.discover[vid.inner()]
    }

    fn is_discovered(&self, vid: VirtID) -> bool {
        self.discover_of(vid).is_finite()
    }

    fn finished_of(&self, vid: VirtID) -> Magnitude<usize> {
        self.state.finished[vid.inner()]
    }

    fn is_finished(&self, vid: VirtID) -> bool {
        self.finished_of(vid).is_finite()
    }

    fn parent_of(&self, rid: RealID) -> Option<RealID> {
        self.state
            .parent
            .get(&rid.inner())
            .map(|p_rid| RealID::from(*p_rid))
    }

    //--- Execution
    pub fn execute(mut self, mut callback: impl FnMut(Event) -> ControlFlow) {
        use misc::ControlFlow::*;
        use misc::Event::*;

        while let Some(s_rid) = self.next_start_vertex_id() {
            // callback -> onBegin
            callback!(callback(Begin(&self.state, s_rid)), {
                let s_vid = self.state.id_map[s_rid];
                self.state.time = self.state.time + 1;
                self.state.discover[s_vid.inner()] = self.state.time.into();

                // callback -> onDiscover
                callback!(callback(Discover(&self.state, s_rid)), {
                    let mut stack = vec![(s_rid, 0, self.graph.neighbors(s_rid.inner()))];

                    while !stack.is_empty() {
                        let (v_rid, depth, mut neighbors) = stack.pop().unwrap();
                        let v_vid = self.state.id_map[v_rid];

                        if let Some(n_rid) = neighbors.next().map(|n_id| RealID::from(n_id)) {
                            let n_vid = self.state.id_map[n_rid];

                            if G::Dir::is_undirected()
                                && (self.parent_of(v_rid) == Some(n_rid) || self.is_finished(n_vid))
                            {
                                stack.push((v_rid, depth, neighbors));
                                continue;
                            }

                            let edge_type = self.edge_type_of(v_vid, n_vid);

                            match edge_type {
                                EdgeType::TreeEdge(_, _) => {
                                    self.state.parent.insert(n_rid.inner(), v_rid.inner());

                                    stack.push((v_rid, depth, neighbors));

                                    self.state.time = self.state.time + 1;
                                    self.state.discover[n_vid.inner()] = self.state.time.into();

                                    // callback -> onDiscover
                                    callback!(callback(Discover(&self.state, n_rid)), {
                                        stack.push((
                                            n_rid,
                                            depth + 1,
                                            self.graph.neighbors(n_rid.inner()),
                                        ));
                                    });
                                }
                                _ => {
                                    stack.push((v_rid, depth, neighbors));
                                }
                            }

                            // callback -> onEdge
                            if callback(VisitEdge(&self.state, edge_type)) == Return {
                                return;
                            }
                        } else {
                            self.state.time = self.state.time + 1;
                            self.state.finished[v_vid.inner()] = self.state.time.into();

                            // callback -> onFinish
                            if callback(Finish(&self.state, v_rid)) == Return {
                                return;
                            }
                        }
                    }
                });
            });

            // callback -> onEnd
            if callback(End(&self.state)) == Return {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gen::{
        BarbellGraphGenerator, CompleteGraphGenerator, CycleGraphGenerator, EmptyGraphGenerator,
        Generator, LadderGraphGenerator, LollipopGraphGenerator, NullGraphGenerator,
        PathGraphGenerator, StarGraphGenerator,
    };
    use crate::storage::edge::Undirected;
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

        pub fn count(&mut self, event: Event) {
            match event {
                Event::Begin(_, _) => self.begin_called += 1,
                Event::Discover(_, _) => self.discover_called += 1,
                Event::Finish(_, _) => self.finish_called += 1,
                Event::End(_) => self.end_called += 1,
                Event::VisitEdge(_, edge_type) => match edge_type {
                    EdgeType::ForwardEdge(_, _) => self.forward_edge_found += 1,
                    EdgeType::TreeEdge(_, _) => self.tree_edge_found += 1,
                    EdgeType::BackEdge(_, _) => self.back_edge_found += 1,
                    EdgeType::CrossEdge(_, _) => self.cross_edge_found += 1,
                },
            }
        }
    }

    #[test]
    fn dfs_on_null_graph() {
        let graph: AdjMap<(), (), Undirected> = NullGraphGenerator.generate();

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

    #[test]
    fn dfs_on_empty_graph() {
        let graph: AdjMap<(), (), Undirected> = EmptyGraphGenerator::init(5).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 5);
        assert_eq!(event_counter.discover_called, 5);
        assert_eq!(event_counter.finish_called, 5);
        assert_eq!(event_counter.end_called, 5);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 0);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_cycle_graph() {
        let graph: AdjMap<(), (), Undirected> = CycleGraphGenerator::init(3).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 3);
        assert_eq!(event_counter.finish_called, 3);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 2);
        assert_eq!(event_counter.back_edge_found, 1);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_path_graph() {
        let graph: AdjMap<(), (), Undirected> = PathGraphGenerator::init(5).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 5);
        assert_eq!(event_counter.finish_called, 5);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 4);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_star_graph() {
        let graph: AdjMap<(), (), Undirected> = StarGraphGenerator::init(5).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 5);
        assert_eq!(event_counter.finish_called, 5);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 4);
        assert_eq!(event_counter.back_edge_found, 0);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_complete_graph() {
        let graph: AdjMap<(), (), Undirected> = CompleteGraphGenerator::init(4).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 4);
        assert_eq!(event_counter.finish_called, 4);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 3);
        assert_eq!(event_counter.back_edge_found, 3);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_lollipop_graph() {
        let graph: AdjMap<(), (), Undirected> = LollipopGraphGenerator::init(4, 4).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 8);
        assert_eq!(event_counter.finish_called, 8);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 4 + 3);
        assert_eq!(event_counter.back_edge_found, 3);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_barbell_graph() {
        let graph: AdjMap<(), (), Undirected> = BarbellGraphGenerator::init(4, 4).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 12);
        assert_eq!(event_counter.finish_called, 12);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 1 + 3 + 1 + 3 + 3);
        assert_eq!(event_counter.back_edge_found, 6);
        assert_eq!(event_counter.cross_edge_found, 0);
    }

    #[test]
    fn dfs_on_ladder_graph() {
        let graph: AdjMap<(), (), Undirected> = LadderGraphGenerator::init(6).generate();

        let mut event_counter = EventCounter::init();

        DFS::init(&graph).execute(|event| {
            event_counter.count(event);

            Continue
        });

        assert_eq!(event_counter.begin_called, 1);
        assert_eq!(event_counter.discover_called, 6);
        assert_eq!(event_counter.finish_called, 6);
        assert_eq!(event_counter.end_called, 1);

        assert_eq!(event_counter.forward_edge_found, 0);
        assert_eq!(event_counter.tree_edge_found, 5);
        assert_eq!(event_counter.back_edge_found, 2);
        assert_eq!(event_counter.cross_edge_found, 0);
    }
}
