use crate::provide::{Direction, EdgeProvider, IdMap, NodeId, NodeProvider};

use super::ListType;

// Indicates the absence of a value.
const UNKNOWN: usize = usize::MAX;

// This macro can be used to find the smallest index of an element in any of T1_in, T2_in, T1_out, or T2_out.
// For example it can be called like this: next_index!(in1, core1) to find the smallest index in T1_in. And it will
// return `None` if T1_in is empty.
//
// # Arguments
// * `list`: Can be any of `in1`, `in2`, `out1` or `out2`.
// * `core`: Can be either `core1` or `core2` depending on the value of `list`.
//           If the macro is invoked with `in1` or `out1` as `list`, it must be provided with `core1` as `core`.
//           If the macro is invoked with `in2` or `out2` as `list`, it must be provided with `core2` as `core`.
// * `start`: Indicates where in the `list` must the search begin.
macro_rules! next_index {
    ($list:expr, $core: expr, start = $start: expr) => {
        $list[$start..]
            .iter()
            .enumerate()
            .find(|(index, depth)| **depth != UNKNOWN && $core[*index] == UNKNOWN)
            .map(|(index, _)| index + $start)
    };
    ($list:expr, $map: expr) => {
        next_index!($list, $map, start = 0)
    };
}

// This macro can be used to find the smallest index of an element in either (N1 - `core1`) or (N2 - `core2`).
// Where N1 is the set of nodes of G1 and N2 is the set of nodes of G2.
//
// # Arguments
// * `list`: Can be either `core1` or `core2`.
// * `start`: Indicates where in the `list` must the search begin.
macro_rules! next_index_in_rest {
    ($list: expr, start = $start:expr) => {
        $list[$start..]
            .iter()
            .enumerate()
            .find(|(_, matched)| **matched == UNKNOWN)
            .map(|(index, _)| index + $start)
    };
    ($list: expr) => {
        next_index_in_rest!($list, start = 0)
    };
}

/// Specifies what type of isomorphism should the algorithm try to find.
/// * Graph isomorphism is defined as a matching M between two graphs of the same order(number of nodes) and same size(number of edges).
/// This type of isomorphism is indicated by [Graph](IsomorphismType::Graph).
///
/// * Graph-Subgraph isomorphism is a matching M between a graph like G2 and a subgraph of another graph like G2.
/// This type of isomorphism is indicated by [Subgraph](IsomorphismType::Subgraph).
pub enum IsomorphismType {
    Graph,
    Subgraph,
}

impl IsomorphismType {
    /// There are different values that are compared when checking the feasibility of a partial mapping in the VF2 algorithm.
    /// The comparison and wether it's successful or not depends on the type of isomorphism the algorithm is trying to find.
    ///
    /// # Arguments
    /// * `g1_val`: Value computed from graph G1.
    /// * `g2_val`: Value computed from graph G2.
    ///
    /// # Returns
    /// * `true`: If the two values `g1_val` and `g2_val` don't undermine the feasibility of the current matching.
    /// * `false`: Otherwise.
    pub(crate) fn is_feasible(&self, g1_val: usize, g2_val: usize) -> bool {
        match *self {
            IsomorphismType::Graph => g1_val == g2_val,
            IsomorphismType::Subgraph => g1_val >= g2_val,
        }
    }
}

// Models a particular state in the state space.
//
// # Generic Parameters
// * `Dir`: Specifies wether G1 and G2 are directed or undirected.
// * `G1`: The first.
// * `G2`: The second graph.
pub(crate) struct VF2State<'a, Dir, G1, G2, I>
where
    Dir: Direction,
    G1: NodeProvider<Dir = Dir> + EdgeProvider,
    G2: NodeProvider<Dir = Dir> + EdgeProvider,
    I: IdMap,
{
    pub(crate) g1: &'a G1,
    pub(crate) g2: &'a G2,

    pub(crate) id_map1: I,
    pub(crate) id_map2: I,

    // The type of isomorphism to search for.
    pub(crate) isomorphism_type: IsomorphismType,

    // Number of matches found in the current state.
    pub(crate) match_count: usize,

    // Data structures as stated in the paper. For more info read the doc at the start of this file.
    pub(crate) core1: Vec<usize>,
    pub(crate) core2: Vec<usize>,

    pub(crate) out1: Vec<usize>,
    pub(crate) out2: Vec<usize>,

    // For undirected graphs, `in#` and `out#` will contain the same data.
    // Therefore, we will only store data in these data structures if the graphs are directed.
    pub(crate) in1: Vec<usize>,
    pub(crate) in2: Vec<usize>,

    // During feasibility checking the size of T1_in will be compared to T2_in.
    // Also, the size of T1_out will compared to T2_out.
    // Because we don't store these sets directly and compute them using `core#`, `in#` and `out#` data structures,
    // it's costly to compute their sizes every time we need them. Therefore we store their sizes directly.
    pub(crate) out1_size: usize, // Size of T1_out
    pub(crate) out2_size: usize, // size of T2_out
    pub(crate) in1_size: usize,  // Size of T1_in
    pub(crate) in2_size: usize,  // Size of T2_in
}

impl<'a, Dir, G1, G2, I> VF2State<'a, Dir, G1, G2, I>
where
    Dir: Direction,
    G1: NodeProvider<Dir = Dir> + EdgeProvider,
    G2: NodeProvider<Dir = Dir> + EdgeProvider,
    I: IdMap,
{
    // # Arguments
    // * `g1`: First graph.
    // * `g2`: Second graph.
    // * `isomorphism_type`: The type of isomorphism to search for.
    pub(crate) fn init(
        g1: &'a G1,
        g2: &'a G2,
        id_map1: I,
        id_map2: I,
        isomorphism_type: IsomorphismType,
    ) -> Self {
        let g1_node_count = g1.node_count();
        let g2_node_count = g2.node_count();

        VF2State {
            g1,
            g2,

            id_map1,
            id_map2,

            isomorphism_type,

            match_count: 0,

            core1: vec![UNKNOWN; g1_node_count],
            core2: vec![UNKNOWN; g2_node_count],

            in1: vec![UNKNOWN; if Dir::is_directed() { g1_node_count } else { 0 }],
            in2: vec![UNKNOWN; if Dir::is_directed() { g2_node_count } else { 0 }],

            in1_size: 0,
            in2_size: 0,

            out1: vec![UNKNOWN; g1_node_count],
            out2: vec![UNKNOWN; g2_node_count],

            out1_size: 0,
            out2_size: 0,
        }
    }

    // At any given time, `match_count` and depth have the same value.
    // Because we add a matching any time we go deeper in the state space.
    // And we remove a matching any time we go up a level in the state space.
    // This method is just here to give context to the value that `match_count` represents.
    #[inline]
    fn depth(&self) -> usize {
        self.match_count
    }

    // Pushes the candidate nodes specified by the `nodes` argument.
    // Any time a pair of nodes are added to the state, we must update all `core#`, `in#`, and `out#` data structures.
    // After this method is finished, `self` will represent a new state in the state space.
    //
    // # Argument
    // * `nodes`: A tuple containing the pair of candidate nodes.
    fn push_state(&mut self, nodes: (usize, usize)) {
        let (node1, node2) = nodes;

        // Add the mapping to the state
        self.core1[node1] = node2;
        self.core2[node2] = node1;
        self.match_count += 1;

        // At this point we don't have any use for virtual ids of the candidate nodes.
        // So we shadow them and replace them with their real ids.
        let node1 = self.id_map1[node1];
        let node2 = self.id_map2[node2];

        // Updates the `out1` data structure.
        for successor in self.g1.successors(node1) {
            let successor = self.id_map1[successor];

            if self.out1[successor] == UNKNOWN {
                self.out1[successor] = self.depth();
                self.out1_size += 1;
            }
        }

        // Updates the `out2` data structure.
        for successor in self.g2.successors(node2) {
            let successor = self.id_map2[successor];

            if self.out2[successor] == UNKNOWN {
                self.out2[successor] = self.depth();
                self.out2_size += 1;
            }
        }

        if Dir::is_directed() {
            // Updates the `in1` data structure.
            for predecessor in self.g1.predecessors(node1) {
                let predecessor = self.id_map1[predecessor];

                if self.in1[predecessor] == UNKNOWN {
                    self.in1[predecessor] = self.depth();
                    self.in1_size += 1;
                }
            }

            // Updates the `in2` data structure.
            for predecessor in self.g2.predecessors(node2) {
                let predecessor = self.id_map2[predecessor];

                if self.in2[predecessor] == UNKNOWN {
                    self.in2[predecessor] = self.depth();
                    self.in2_size += 1;
                }
            }
        }
    }

    // Removes the candidate nodes specified by the `nodes` argument.
    // Any time a pair of nodes are removed from the state, we must update all `core#`, `in#`, and `out#` data structures.
    // After this method is finished, `self` will represent a new state in the state space.
    //
    // # Argument
    // * `nodes`: A tuple containing the pair of candidate nodes.
    pub(crate) fn pop_state(&mut self, nodes: (usize, usize)) {
        let (node1, node2) = nodes;

        self.core1[node1] = UNKNOWN;
        self.core2[node2] = UNKNOWN;

        // At this point we don't have any use for the virtual ids of the nodes.
        // So we shadow them and replace them with their real ids.
        let node1 = self.id_map1[node1];
        let node2 = self.id_map2[node2];

        // Update the `out1` data structure.
        for successor in self.g1.successors(node1) {
            let successor = self.id_map1[successor];

            if self.out1[successor] == self.depth() {
                self.out1[successor] = UNKNOWN;
                self.out1_size -= 1;
            }
        }

        // Update the `out2` data structure.
        for successor in self.g2.successors(node2) {
            let successor = self.id_map2[successor];

            if self.out2[successor] == self.depth() {
                self.out2[successor] = UNKNOWN;
                self.out2_size -= 1;
            }
        }

        if Dir::is_directed() {
            // Update the `in1` data structure.
            for predecessor in self.g1.predecessors(node1) {
                let predecessor = self.id_map1[predecessor];

                if self.in1[predecessor] == self.depth() {
                    self.in1[predecessor] = UNKNOWN;
                    self.in1_size -= 1;
                }
            }

            // Update the `in2` data structure.
            for predecessor in self.g2.predecessors(node2) {
                let predecessor = self.id_map2[predecessor];

                if self.in2[predecessor] == self.depth() {
                    self.in2[predecessor] = UNKNOWN;
                    self.in2_size -= 1;
                }
            }
        }

        self.match_count -= 1;
    }

    // Tries to find the next candidate of pairs in the current state in SSR.
    //
    // # Returns
    // * `Some`: Containing a triple:
    //              ( (node1, node2), list_type )
    // In which node1 is the candidate pair in G1 and node2 is the candidate pair in G2.
    // list_type indicates from what list the pairs are coming from.
    // * `None`: If there are no candidate pairs to be found.
    pub(crate) fn next_candidates(&self) -> Option<((usize, usize), ListType)> {
        // Following the order stated by the VF2 paper, we search these sets to find the candidate pairs.
        //
        // We begin with:                   1. T1_out(s)    x {min T2_out(s)}
        // If T1_out and T2_out were empty: 2. T1_in(s)     x {min T2_in(s)}
        // If T1_in and T2_in were empty:   3. (N1 - M1(s)) x {min (N2 - M2(s))}
        if let (Some(node1), Some(node2)) = (
            next_index!(self.out1, self.core1), // T1_out
            next_index!(self.out2, self.core2), // T2_out
        ) {
            Some(((node1, node2), ListType::Out))
        } else if let (Some(node1), Some(node2)) = (
            next_index!(self.in1, self.core1), // T1_in
            next_index!(self.in2, self.core2), // T2_in
        ) {
            Some(((node1, node2), ListType::In))
        } else if let (Some(node1), Some(node2)) = (
            next_index_in_rest!(self.core1), // N1 - M1(s)
            next_index_in_rest!(self.core2), // N2 - M2(s)
        ) {
            Some(((node1, node2), ListType::Other))
        } else {
            // If all of the sets were empty, there are no candidate pairs to be found.
            // Also, if one set like T1_out is empty and T2_out is non-empty, this block will be executed.
            // But, as stated in the paper, If this happens we could show that the current state is not a feasible state.
            // So returning None is correct.
            None
        }
    }

    // Imagine we find a candidate pair like:
    //      (T1_out[ix1]   , T2_out[ix2])
    // If this candidate pair fails to satisfy the feasibility checking, the next pair must be:
    //      (T1_out[ix + 1], T2_out[ix2])
    //
    // This method does exactly this. For example if we want to find T1_out[ix + 1], we should call this method as below:
    //      next_from_list(ix + 1, ListType::Out)
    //
    // # Arguments
    // * `start`: The smallest index from which to start searching.
    // * `list_type`: Indicates in what list we must search.
    //
    // # Returns
    // `Some`: Containing the smallest index in the given set.
    // `None`: If there are no more elements in the given set.
    pub(crate) fn next_from_list(&self, start: usize, list_type: ListType) -> Option<usize> {
        match list_type {
            ListType::Out => next_index!(self.out1, self.core1, start = start),
            ListType::In => next_index!(self.in1, self.core1, start = start),
            ListType::Other => next_index_in_rest!(self.core1, start = start),
        }
    }

    // Pushes the candidate nodes specified by argument `nodes` if they satisfy all feasibility checks.
    //
    // Look ahead 1
    // 1.1 If a successor of `node1` has a match, its match must be a successor of `node2` as well.
    // 1.2 If a successor of `node2` has a match, its match must be a successor of `node1` as well.
    // 1.3 The number of successors of the two nodes must pass a feasibility test depending on the type of isomorphism requested.
    // 1.4 The number of successors of the two nodes that are not in `core#` but are in `out#`,
    //     must pass a feasibility test depending on the type of isomorphism requested.
    //
    // Look ahead 2 (Only for directed graphs)
    // 2.1 If a predecessor of `node1` has a match, its match must be a predecessor of `node2` as well.
    // 2.2 If a predecessor of `node2` has a match, its match must be a predecessor of `node1` as well.
    // 2.3 Also, the number of predecessors must pass a feasibility test depending on the type of isomorphism requested.
    // 2.4 The number of predecessor of the two nodes that are not in `core#` but are in `in#`,
    //     must pass a feasibility test depending on the type of isomorphism requested.
    //
    // Look ahead 3
    // 3.1 |T1_in| and |T2_in| must pass the feasibility check depending on the type of isomorphism requested.
    // 3.2 |T1_out| and |T2_out| must pass the feasibility check depending on the type of isomorphism requested.
    pub(crate) fn push_if_feasible(
        &mut self,
        nodes: (usize, usize),

        // FIXME: Implement semantic matching
        _node_matcher: &impl FnMut(NodeId, NodeId) -> bool,
        _edge_matcher: &impl FnMut(NodeId, NodeId) -> bool,
    ) -> bool {
        let (node1, node2) = nodes;
        let node1 = self.id_map1[node1];
        let node2 = self.id_map2[node2];

        //--- Look ahead 1 ---//
        let mut successors_count1 = 0; // Number of successors of `node1`
        let mut successors_out1 = 0; // Number of successors of `node1` that are not in `core1` but are in `out1`
        for successor in self.g1.successors(node1) {
            let successor = self.id_map1[successor];

            if self.core1[successor] != UNKNOWN {
                let successor2 = self.id_map2[self.core1[successor]];

                // Look ahead 1.1
                if !self.g2.is_successor(node2, successor2) {
                    return false;
                }
            } else if self.out1[successor] != UNKNOWN {
                successors_out1 += 1;
            }

            successors_count1 += 1;
        }

        let mut successors_count2 = 0; // Number of successors of `node2`
        let mut successors_out2 = 0; // Number of successors of `node2` that are not in `core2` but are in `out2`
        for successor in self.g2.successors(node2) {
            let successor = self.id_map2[successor];

            if self.core2[successor] != UNKNOWN {
                let successor1 = self.id_map1[self.core2[successor]];

                // Look ahead 1.2
                if !self.g1.is_successor(node1, successor1) {
                    return false;
                }
            } else if self.out2[successor] != UNKNOWN {
                successors_out2 += 1;
            }

            successors_count2 += 1;
        }

        // Look ahead 1.3
        if !self
            .isomorphism_type
            .is_feasible(successors_count1, successors_count2)
        {
            return false;
        }

        // Look ahead 1.4
        if !self
            .isomorphism_type
            .is_feasible(successors_out1, successors_out2)
        {
            return false;
        }

        if Dir::is_directed() {
            //--- Look ahead 2 ---//
            let mut predecessors_count1 = 0; // Number of predecessors of `node1`
            let mut predecessors_in1 = 0; // Number of predecessors of `node1` that are not in `core1` but are in `in1`
            for predecessor in self.g1.predecessors(node1) {
                let predecessor = self.id_map1[predecessor];

                if self.core1[predecessor] != UNKNOWN {
                    let predecessor2 = self.id_map2[self.core1[predecessor]];

                    // Look ahead 2.1
                    if !self.g2.is_predecessor(node2, predecessor2) {
                        return false;
                    }
                } else if self.in1[predecessor] != UNKNOWN {
                    predecessors_in1 += 1;
                }

                predecessors_count1 += 1;
            }

            let mut predecessors_count2 = 0; // Number of predecessors of `node2`
            let mut predecessors_in2 = 0; // Number of predecessors of `node2` that are not in `core1` but are in `in2`
            for predecessor in self.g2.predecessors(node2) {
                let predecessor = self.id_map2[predecessor];

                if self.core2[predecessor] != UNKNOWN {
                    let predecessor1 = self.id_map1[self.core2[predecessor]];

                    // Look ahead 2.2
                    if !self.g1.is_predecessor(node1, predecessor1) {
                        return false;
                    }
                } else if self.in2[predecessor] != UNKNOWN {
                    predecessors_in2 += 1;
                }

                predecessors_count2 += 1;
            }

            // Look ahead 2.3
            if !self
                .isomorphism_type
                .is_feasible(predecessors_count1, predecessors_count2)
            {
                return false;
            }

            // Look ahead 2.4
            if !self
                .isomorphism_type
                .is_feasible(predecessors_in1, predecessors_in2)
            {
                return false;
            }
        }

        //--- Look ahead 3 ---//
        self.push_state(nodes);
        if !self
            .isomorphism_type
            .is_feasible(self.in1_size, self.in2_size) // Look ahead 3.1
            || !self
                .isomorphism_type
                .is_feasible(self.out1_size, self.out2_size)
        // Look ahead 3.2
        {
            self.pop_state(nodes);
            return false;
        }

        true
    }
}
