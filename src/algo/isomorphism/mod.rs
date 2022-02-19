use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::DynIter;
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;

macro_rules! fill_map {
    ($core: expr, $graph:expr, $func: tt, $target_map: expr, $val: expr ) => {
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

    core_1: HashMap<usize, usize>,
    core_2: HashMap<usize, usize>,

    in_1: HashMap<usize, usize>,
    in_2: HashMap<usize, usize>,

    out_1: HashMap<usize, usize>,
    out_2: HashMap<usize, usize>,
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
            core_1: HashMap::new(),
            core_2: HashMap::new(),

            in_1: HashMap::new(),
            in_2: HashMap::new(),

            out_1: HashMap::new(),
            out_2: HashMap::new(),
        }
    }

    fn candidate_pairs_iter(&self) -> DynIter<'_, (usize, usize)> {
        let t1_out = diff!(self.out_1.keys(), self.core_1).collect_vec();
        let t2_out = diff!(self.out_2.keys(), self.core_2).collect_vec();

        if !t1_out.is_empty() && !t2_out.is_empty() {
            let min_2 = **t2_out.iter().min().unwrap();
            DynIter::init(t1_out.into_iter().map(move |node_1| (*node_1, min_2)))
        } else {
            let t1_in = diff!(self.in_1.keys(), self.core_1).collect_vec();
            let t2_in = diff!(self.in_2.keys(), self.core_2).collect_vec();

            if !t1_in.is_empty() && !t2_in.is_empty() {
                let min_2 = **t2_in.iter().min().unwrap();
                DynIter::init(t1_in.into_iter().map(move |node_id| (*node_id, min_2)))
            } else {
                let m = diff!(self.graph_2.vertex_tokens(), self.core_2);
                let min_2 = m.min().unwrap();

                let n = diff!(self.graph_1.vertex_tokens(), self.core_1);
                DynIter::init(n.map(move |node_id| (node_id, min_2)))
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
            if let Some(mapped_node) = self.core_1.get(&pred_id) {
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
            if let Some(mapped_node) = self.core_2.get(&pred_id) {
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
            if let Some(mapped_node) = self.core_1.get(&succ_id) {
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
            if let Some(mapped_node) = self.core_2.get(&succ_id) {
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
        let num_1 = filter!(pred_1, self.in_1, !self.core_1).count();
        let num_2 = filter!(pred_2, self.in_2, !self.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, self.in_1, !self.core_1).count();
        let num_2 = filter!(succ_2, self.in_2, !self.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        // Look ahead 2
        let num_1 = filter!(pred_1, self.out_1, !self.core_1).count();
        let num_2 = filter!(pred_2, self.out_2, !self.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, self.out_1, !self.core_1).count();
        let num_2 = filter!(succ_2, self.out_2, !self.core_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        // Look ahead 3
        let num_1 = filter!(pred_1, !self.in_1, !self.out_1).count();
        let num_2 = filter!(pred_2, !self.in_2, !self.out_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        let num_1 = filter!(succ_1, !self.in_1, !self.out_1).count();
        let num_2 = filter!(succ_2, !self.in_2, !self.out_2).count();

        if !self.match_type.is_match(num_1, num_2) {
            return false;
        }

        true
    }

    pub fn pop_state(&mut self, g1_node: usize, g2_node: usize, depth: usize) {
        println!(
            "Poping state: ({:?}, {:?}) with depth: {}",
            g1_node, g2_node, depth
        );

        self.core_1.remove(&g1_node);
        self.core_2.remove(&g2_node);

        self.in_1.retain(|_, val| *val != depth);
        self.in_2.retain(|_, val| *val != depth);

        self.out_1.retain(|_, val| *val != depth);
        self.out_2.retain(|_, val| *val != depth);
    }

    pub fn push_state(&mut self, g1_node: usize, g2_node: usize, depth: usize) {
        self.core_1.insert(g1_node, g2_node);
        self.core_2.insert(g2_node, g1_node);

        self.in_1.entry(g1_node).or_insert(depth);
        self.in_2.entry(g2_node).or_insert(depth);

        self.out_1.entry(g1_node).or_insert(depth);
        self.out_2.entry(g2_node).or_insert(depth);

        fill_map!(self.core_1, self.graph_1, predecessors, self.in_1, depth);
        fill_map!(self.core_2, self.graph_2, predecessors, self.in_2, depth);

        fill_map!(self.core_1, self.graph_1, successors, self.out_1, depth);
        fill_map!(self.core_2, self.graph_2, successors, self.out_2, depth);
    }

    pub fn execute(&mut self) -> Option<HashMap<usize, usize>> {
        if self.core_1.len() == self.graph_2.vertex_count() {
            Some(self.core_1.clone())
        } else {
            let candidate_pairs = self.candidate_pairs_iter().collect_vec();

            for (g1_node, g2_node) in candidate_pairs {
                println!("candidates: g1_node: {}, g2_node: {}", g1_node, g2_node);
                if self.syntactic_feasibility(g1_node, g2_node) {
                    println!("Feasible!");
                    let depth = self.core_1.len() + 1;

                    self.push_state(g1_node, g2_node, depth);

                    let result = self.execute();

                    if result.is_some() {
                        return result;
                    }

                    self.pop_state(g1_node, g2_node, depth);
                }
            }

            return None;
        }
    }
}

#[cfg(test)]
mod tests {

    use quickcheck_macros::quickcheck;

    use crate::gen::{CompleteGraphGenerator, CycleGraphGenerator, Generator, PathGraphGenerator};
    use crate::provide::Vertices;
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;

    use super::{DiGraphMatcher, MatchType};

    #[quickcheck]
    fn prop_isomorphism_on_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let matching = DiGraphMatcher::init(&graph, &graph, MatchType::Graph)
            .execute()
            .unwrap();

        assert_eq!(matching.len(), graph.vertex_count());
    }

    #[quickcheck]
    fn prop_isomorphism_on_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let matching = DiGraphMatcher::init(&graph, &graph, MatchType::Graph)
            .execute()
            .unwrap();

        assert_eq!(matching.len(), graph.vertex_count());
    }

    #[quickcheck]
    fn prop_isomorphism_on_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let matching = DiGraphMatcher::init(&graph, &graph, MatchType::Graph)
            .execute()
            .unwrap();

        assert_eq!(matching.len(), graph.vertex_count());
    }

    #[quickcheck]
    fn prop_subgraph_isomorphism_on_complete_graph(graph_size: usize, diff_size: usize) {
        let diff_size = diff_size % 2 + 1;
        let graph_size = graph_size % 16 + 4;

        let subgraph_size = graph_size - diff_size;

        let graph: AdjMap<(), (), Directed> = CompleteGraphGenerator::init(graph_size).generate();
        let subgraph: AdjMap<(), (), Directed> =
            CompleteGraphGenerator::init(subgraph_size).generate();

        let matching = DiGraphMatcher::init(&graph, &subgraph, MatchType::Subgraph)
            .execute()
            .unwrap();

        assert_eq!(matching.len(), subgraph.vertex_count());
    }

    #[quickcheck]
    fn prop_subgraph_isomorphism_on_path_graph(graph_size: usize, diff_size: usize) {
        let diff_size = diff_size % 2 + 1;
        let graph_size = graph_size % 16 + 4;
        let subgraph_size = graph_size - diff_size;

        let graph: AdjMap<(), (), Directed> = PathGraphGenerator::init(graph_size).generate();
        let subgraph: AdjMap<(), (), Directed> = PathGraphGenerator::init(subgraph_size).generate();

        let matching = DiGraphMatcher::init(&graph, &subgraph, MatchType::Subgraph)
            .execute()
            .unwrap();

        assert_eq!(matching.len(), subgraph.vertex_count());
    }
}
