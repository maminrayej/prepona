use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::DynIter;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;

macro_rules! fill_map {
    ($core: expr, $graph: ident, $func: tt, $target_map: expr, $val: expr ) => {
        $core
            .keys()
            .flat_map(|node_id| {
                $graph
                    .$func(*node_id)
                    .filter(|pred_id| !$core.contains_key(pred_id))
            })
            .for_each(|pred_id| {
                $target_map.entry(pred_id).or_insert($val);
            })
    };
}

macro_rules! diff {
    ($values: expr, $filter_set: expr) => {
        $values.filter(|node_id| !$filter_set.contains_key(node_id))
    };
}

macro_rules! filter {
    ($set: ident, !$filter_1: expr, !$filter_2: expr) => {
        $set.iter()
            .filter(|val| !$filter_1.contains_key(val) && !$filter_2.contains_key(val))
    };

    ($set: ident, $filter_1: expr, !$filter_2: expr) => {
        $set.iter()
            .filter(|val| $filter_1.contains_key(val) && !$filter_2.contains_key(val))
    };
}

struct DiGMState {
    g1_node: Option<usize>,
    g2_node: Option<usize>,
    depth: usize,
}

impl DiGMState {
    pub fn init() -> Self {
        DiGMState {
            g1_node: None,
            g2_node: None,
            depth: 0,
        }
    }

    pub fn push_state<'a, G>(
        &mut self,
        maps: &mut Maps,
        graph_1: &'a G,
        graph_2: &'a G,
        g1_node: usize,
        g2_node: usize,
    ) where
        G: Storage<Dir = Directed> + Vertices,
    {
        maps.core_1.insert(g1_node, g2_node);
        maps.core_2.insert(g2_node, g1_node);

        self.g1_node = Some(g1_node);
        self.g2_node = Some(g2_node);

        self.depth = maps.core_1.len();

        maps.in_1.entry(g1_node).or_insert(self.depth);
        maps.in_2.entry(g2_node).or_insert(self.depth);

        maps.out_1.entry(g1_node).or_insert(self.depth);
        maps.out_2.entry(g2_node).or_insert(self.depth);

        fill_map!(maps.core_1, graph_1, predecessors, maps.in_1, self.depth);
        fill_map!(maps.core_2, graph_2, predecessors, maps.in_2, self.depth);

        fill_map!(maps.core_1, graph_1, successors, maps.out_1, self.depth);
        fill_map!(maps.core_2, graph_2, successors, maps.out_2, self.depth);
    }

    pub fn pop_state(&mut self, maps: &mut Maps) {
        if let (Some(g1_node), Some(g2_node)) = (self.g1_node, self.g2_node) {
            maps.core_1.remove(&g1_node);
            maps.core_2.remove(&g2_node);
        }

        maps.in_1.retain(|_, val| *val != self.depth);
        maps.in_2.retain(|_, val| *val != self.depth);

        maps.out_1.retain(|_, val| *val != self.depth);
        maps.out_2.retain(|_, val| *val != self.depth);
    }
}

struct Maps {
    core_1: HashMap<usize, usize>,
    core_2: HashMap<usize, usize>,

    in_1: HashMap<usize, usize>,
    in_2: HashMap<usize, usize>,

    out_1: HashMap<usize, usize>,
    out_2: HashMap<usize, usize>,
}

pub enum MatchType {
    Graph,
    Subgraph,
}

impl MatchType {
    fn is_match(&self, val_1: usize, val_2: usize) -> bool {
        if matches!(self, MatchType::Graph) {
            val_1 == val_2
        } else {
            val_1 >= val_2
        }
    }
}

pub struct DiGraphMatcher<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    graph_1: &'a G,
    graph_2: &'a G,

    match_type: MatchType,

    maps: Maps,

    state: DiGMState,
}

impl<'a, G> DiGraphMatcher<'a, G>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    pub fn init(graph_1: &'a G, graph_2: &'a G, match_type: MatchType) -> Self {
        DiGraphMatcher {
            graph_1,
            graph_2,
            match_type,
            maps: Maps {
                core_1: HashMap::new(),
                core_2: HashMap::new(),

                in_1: HashMap::new(),
                in_2: HashMap::new(),

                out_1: HashMap::new(),
                out_2: HashMap::new(),
            },
            state: DiGMState::init(),
        }
    }

    fn candidate_pairs_iter(&self) -> DynIter<'_, (usize, usize)> {
        let t1_out = diff!(self.maps.out_1.keys(), self.maps.core_1).collect_vec();
        let t2_out = diff!(self.maps.out_2.keys(), self.maps.core_2).collect_vec();

        if !t1_out.is_empty() && !t2_out.is_empty() {
            let min_2 = **t2_out.iter().min().unwrap();
            DynIter::init(t1_out.into_iter().map(move |node_1| (*node_1, min_2)))
        } else {
            let t1_in = diff!(self.maps.in_1.keys(), self.maps.core_1).collect_vec();
            let t2_in = diff!(self.maps.in_2.keys(), self.maps.core_2).collect_vec();

            if !t1_in.is_empty() && !t2_in.is_empty() {
                let min_2 = **t2_in.iter().min().unwrap();
                DynIter::init(t1_in.into_iter().map(move |node_id| (*node_id, min_2)))
            } else {
                let min_2 = diff!(self.graph_2.vertex_tokens(), self.maps.core_2)
                    .min()
                    .unwrap();

                DynIter::init(
                    diff!(self.graph_1.vertex_tokens(), self.maps.core_1)
                        .map(move |node_id| (node_id, min_2)),
                )
            }
        }
    }

    fn syntactic_feasibility(&self, g1_node: usize, g2_node: usize) -> bool {
        // Checking self loops
        if !self.match_type.is_match(
            self.graph_1.edges_between(g1_node, g1_node).count(),
            self.graph_2.edges_between(g2_node, g2_node).count(),
        ) {
            return false;
        }

        let pred_1: HashSet<usize> = self.graph_1.predecessors(g1_node).collect();
        let succ_1: HashSet<usize> = self.graph_1.successors(g1_node).collect();

        let pred_2: HashSet<usize> = self.graph_2.predecessors(g2_node).collect();
        let succ_2: HashSet<usize> = self.graph_2.successors(g2_node).collect();

        // Check neighbors in partial mapping
        for pred_id in pred_1.iter().copied() {
            if let Some(mapped_node) = self.maps.core_1.get(&pred_id) {
                if !pred_2.contains(mapped_node)
                    || !self.match_type.is_match(
                        self.graph_1.edges_between(pred_id, g1_node).count(),
                        self.graph_2.edges_between(*mapped_node, g2_node).count(),
                    )
                {
                    return false;
                }
            }
        }

        for pred_id in pred_2.iter().copied() {
            if let Some(mapped_node) = self.maps.core_2.get(&pred_id) {
                if !pred_1.contains(mapped_node)
                    || !self.match_type.is_match(
                        self.graph_1.edges_between(*mapped_node, g1_node).count(),
                        self.graph_2.edges_between(pred_id, g2_node).count(),
                    )
                {
                    return false;
                }
            }
        }

        for succ_id in succ_1.iter().copied() {
            if let Some(mapped_node) = self.maps.core_1.get(&succ_id) {
                if !succ_2.contains(mapped_node)
                    || !self.match_type.is_match(
                        self.graph_1.edges_between(g1_node, succ_id).count(),
                        self.graph_2.edges_between(g2_node, *mapped_node).count(),
                    )
                {
                    return false;
                }
            }
        }

        for succ_id in succ_2.iter().copied() {
            if let Some(mapped_node) = self.maps.core_2.get(&succ_id) {
                if !succ_1.contains(mapped_node)
                    || !self.match_type.is_match(
                        self.graph_1.edges_between(g1_node, *mapped_node).count(),
                        self.graph_2.edges_between(g2_node, succ_id).count(),
                    )
                {
                    return false;
                }
            }
        }

        // Look ahead 1
        let num_1 = filter!(pred_1, self.maps.in_1, !self.maps.core_1).count();
        let num_2 = filter!(pred_2, self.maps.in_2, !self.maps.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, self.maps.in_1, !self.maps.core_1).count();
        let num_2 = filter!(succ_2, self.maps.in_2, !self.maps.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        // Look ahead 2
        let num_1 = filter!(pred_1, self.maps.out_1, !self.maps.core_1).count();
        let num_2 = filter!(pred_2, self.maps.out_2, !self.maps.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, self.maps.out_1, !self.maps.core_1).count();
        let num_2 = filter!(succ_2, self.maps.out_2, !self.maps.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        // Look ahead 3
        let num_1 = filter!(pred_1, !self.maps.in_1, !self.maps.out_1).count();
        let num_2 = filter!(pred_2, !self.maps.in_2, !self.maps.out_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, !self.maps.in_1, !self.maps.out_1).count();
        let num_2 = filter!(succ_2, !self.maps.in_2, !self.maps.out_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        true
    }

    pub fn execute(&mut self) -> Option<HashMap<usize, usize>> {
        if self.maps.core_1.len() == self.graph_2.vertex_count() {
            Some(self.maps.core_1.clone())
        } else {
            let candidate_pairs = self.candidate_pairs_iter().collect_vec();

            for (g1_node, g2_node) in candidate_pairs {
                if self.syntactic_feasibility(g1_node, g2_node) {
                    self.state.push_state(
                        &mut self.maps,
                        self.graph_1,
                        self.graph_2,
                        g1_node,
                        g2_node,
                    );
                    let result = self.execute();

                    if result.is_some() {
                        return result;
                    }

                    self.state.pop_state(&mut self.maps);
                }
            }

            return None;
        }
    }
}
