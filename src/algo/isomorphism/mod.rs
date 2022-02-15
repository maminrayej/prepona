use std::collections::HashMap;

use crate::provide::{Storage, Vertices};
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
            });
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

pub struct DiGraphMatcher<'a, G>
where
    G: Storage<Dir = Directed> + Vertices,
{
    graph_1: &'a G,
    graph_2: &'a G,

    maps: Maps,

    state: DiGMState,
}

impl<'a, G> DiGraphMatcher<'a, G>
where
    G: Storage<Dir = Directed> + Vertices,
{
    fn push(&mut self) {
        self.state
            .push_state(&mut self.maps, self.graph_1, self.graph_2, 1, 1);
    }
}
