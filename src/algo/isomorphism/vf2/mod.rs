mod state;

pub use state::*;

use crate::provide::{DefaultIdMap, Direction, EdgeProvider, IdMap, NodeId, NodeProvider};

/// VF2 isomorphism algorithm.
///
/// # Generic Parameters
/// * `Dir`: Indicates wether the provided graphs (`G1` and `G2`) are directed or not.
/// * `G1`: Type of the first graph.
/// * `G2`: Type of the second graph.
pub struct VF2Isomorphism<'a, Dir, G1, G2, I = DefaultIdMap>
where
    Dir: Direction,
    G1: NodeProvider<Dir = Dir> + EdgeProvider,
    G2: NodeProvider<Dir = Dir> + EdgeProvider,
    I: IdMap,
{
    state: VF2State<'a, Dir, G1, G2, I>,
}

// Indicates the source from which the candidates pair are chosen.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ListType {
    // Indicates that the candidate pairs are chosen from the `out#` data structures.
    Out,

    // Indicates that the candidate pairs are chosen from the `in#` data structures.
    In,

    // Indicates that the candidate pairs are chosen from the rest of the nodes (that are not present in `in#` nor `out#`).
    Other,
}

// Indicates the action that must be carried out by the algorithm.
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum Action {
    // This action indicates that the algorithm must find a pair of candidates and try to go to the next state.
    GoNextState,

    // This action indicates that the algorithm must verify the feasibility
    // of the state if `nodes` were to be added to the matching.
    VerifyState {
        nodes: (usize, usize),
        list_type: ListType,
    },

    // This action indicates that the algorithm must remove the `nodes` that caused the current state to be created,
    // try to find a new pair of candidates, and go to the next state.
    UnwindState {
        nodes: (usize, usize),
        list_type: ListType,
    },
}

impl<'a, Dir, G1, G2> VF2Isomorphism<'a, Dir, G1, G2>
where
    Dir: Direction,
    G1: NodeProvider<Dir = Dir> + EdgeProvider,
    G2: NodeProvider<Dir = Dir> + EdgeProvider,
{
    pub fn init(g1: &'a G1, g2: &'a G2, isomorphism_type: IsomorphismType) -> Self {
        let id_map1 = DefaultIdMap::new(g1);
        let id_map2 = DefaultIdMap::new(g2);
        Self::with_ids(g1, g2, id_map1, id_map2, isomorphism_type)
    }
}

impl<'a, Dir, G1, G2, I> VF2Isomorphism<'a, Dir, G1, G2, I>
where
    Dir: Direction,
    G1: NodeProvider<Dir = Dir> + EdgeProvider,
    G2: NodeProvider<Dir = Dir> + EdgeProvider,
    I: IdMap,
{
    pub fn with_ids(
        g1: &'a G1,
        g2: &'a G2,
        id_map1: I,
        id_map2: I,
        isomorphism_type: IsomorphismType,
    ) -> Self {
        VF2Isomorphism {
            state: VF2State::init(g1, g2, id_map1, id_map2, isomorphism_type),
        }
    }

    pub fn execute_with_matchers(
        &mut self,
        node_matcher: impl FnMut(NodeId, NodeId) -> bool,
        edge_matcher: impl FnMut(NodeId, NodeId) -> bool,
    ) -> bool {
        if !self
            .state
            .isomorphism_type
            .is_feasible(self.state.g1.node_count(), self.state.g2.node_count())
            || !self
                .state
                .isomorphism_type
                .is_feasible(self.state.g1.edge_count(), self.state.g2.edge_count())
        {
            return false;
        }

        // The method pops an action from the actions stack. Processes the data,
        // and potentially pushes back some other actions onto the stack.
        // This process will continue until there are no more actions to be done (stack is empty).

        // First action to be executed is to find a candidate pair.
        let mut actions = vec![Action::GoNextState];

        while let Some(action) = actions.pop() {
            match action {
                Action::GoNextState => {
                    // If we are at a state that maps all nodes from G2 to G1, the algorithm can halt.
                    // We assume here that |G1| >= |G2|.
                    if self.state.match_count == self.state.g2.node_count() {
                        return true;
                    }

                    match self.state.next_candidates() {
                        Some((nodes, list_type)) => {
                            // If a pair of candidates were found, it must be verified first before getting added to the partial matching.
                            // So we create a VerifyState and push it onto the stack to get executed next.
                            let verify_state = Action::VerifyState { nodes, list_type };
                            actions.push(verify_state);
                        }
                        None => continue,
                    }
                }
                Action::VerifyState { nodes, list_type } => {
                    if self
                        .state
                        .push_if_feasible(nodes, &node_matcher, &edge_matcher)
                    {
                        // If the candidates were feasible and got added to the state,
                        // we must go deeper in the state space and try to find another pair in order to grow the partial mapping.
                        // But, if the path we've taken by adding `nodes` were to fail further ahead, we must unwind it to
                        // try other candidates. Therefore we first push an `UnwindState` action and then a `GoNextState` action.
                        let unwind_state = Action::UnwindState { nodes, list_type };
                        let go_next_state = Action::GoNextState;

                        actions.push(unwind_state);
                        actions.push(go_next_state);
                    } else {
                        // If the candidates were not feasible, we must look for the next pair of candidates.
                        // The search will keep the chosen node from G2 (that has the minimum index) and selects
                        // another node from G1.
                        match self.state.next_from_list(nodes.0 + 1, list_type) {
                            Some(node0) => {
                                // The next pair of candidates must be verified before getting added to the partial matching.
                                let verify_state = Action::VerifyState {
                                    nodes: (node0, nodes.1),
                                    list_type,
                                };
                                actions.push(verify_state);
                            }
                            None => continue,
                        }
                    }
                }
                Action::UnwindState { nodes, list_type } => {
                    self.state.pop_state(nodes);

                    // After unwinding the current state, we must look for the next pair of candidates.
                    // The search will keep the chosen node from G2 (that has the minimum index) and selects
                    // another node from G1.
                    match self.state.next_from_list(nodes.0 + 1, list_type) {
                        Some(node0) => {
                            // The next pair of candidates must be verified before getting added to the partial matching.
                            let verify_state = Action::VerifyState {
                                nodes: (node0, nodes.1),
                                list_type,
                            };
                            actions.push(verify_state);
                        }
                        None => continue,
                    }
                }
            }
        }

        false
    }

    pub fn execute_with_edge_matcher(
        &mut self,
        edge_matcher: impl FnMut(NodeId, NodeId) -> bool,
    ) -> bool {
        self.execute_with_matchers(|_, _| true, edge_matcher)
    }

    pub fn execute_with_node_matcher(
        &mut self,
        node_matcher: impl FnMut(NodeId, NodeId) -> bool,
    ) -> bool {
        self.execute_with_matchers(node_matcher, |_, _| true)
    }

    pub fn execute(&mut self) -> bool {
        self.execute_with_matchers(|_, _| true, |_, _| true)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::gen::{
        CircularLadderGraph, CompleteGraph, CycleGraph, Generator, LadderGraph, LollipopGraph,
        PathGraph, StarGraph, WheelGraph,
    };
    use crate::provide::{Directed, NodeProvider, Undirected};
    use crate::storage::AdjMap;

    use super::{IsomorphismType, VF2Isomorphism};

    //--- Isomorphism tests ---//

    #[quickcheck]
    fn vf2_isomorphism_on_directed_complete_graph(generator: CompleteGraph) {
        let g1: AdjMap<Directed> = generator.generate();
        let g2: AdjMap<Directed> = generator.generate();

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Graph).execute();

        assert!(are_isomorph);
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_complete_graph(generator: CompleteGraph) {
        let g1: AdjMap<Undirected> = generator.generate();
        let g2: AdjMap<Undirected> = generator.generate();

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Graph).execute();

        assert!(are_isomorph);
    }

    #[quickcheck]
    fn vf2_isomorphism_on_directed_path_graph(generator: PathGraph) {
        let g1: AdjMap<Directed> = generator.generate();
        let g2: AdjMap<Directed> = generator.generate();

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_wheel_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let wheel: AdjMap<Undirected> = WheelGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&wheel, &wheel, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_star_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let star: AdjMap<Undirected> = StarGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&star, &star, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_lollipop_graph(
        complete_graph_size: usize,
        path_graph_size: usize,
    ) {
        let complete_graph_size = complete_graph_size % 6 + 4;
        let path_graph_size = path_graph_size % 9 + 1;

        let lollipop: AdjMap<Undirected> =
            LollipopGraph::init(complete_graph_size, path_graph_size).generate();

        let are_isomorph =
            VF2Isomorphism::init(&lollipop, &lollipop, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_cycle_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let cycle: AdjMap<Undirected> = CycleGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&cycle, &cycle, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_ladder_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let ladder: AdjMap<Undirected> = LadderGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&ladder, &ladder, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_circular_ladder_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let circular_ladder: AdjMap<Undirected> = CircularLadderGraph::init(node_count).generate();

        let are_isomorph =
            VF2Isomorphism::init(&circular_ladder, &circular_ladder, IsomorphismType::Graph)
                .execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_directed_cycle_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;

        let cycle: AdjMap<Directed> = CycleGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&cycle, &cycle, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    //--- Subgraph isomorphism tests ---//

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_directed_complete_graph(
        generator1: CompleteGraph,
        generator2: CompleteGraph,
    ) {
        let mut g1: AdjMap<Directed> = generator1.generate();
        let mut g2: AdjMap<Directed> = generator2.generate();

        if g1.node_count() < g2.node_count() {
            std::mem::swap(&mut g1, &mut g2);
        }

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_complete_graph(
        generator1: CompleteGraph,
        generator2: CompleteGraph,
    ) {
        let mut g1: AdjMap<Undirected> = generator1.generate();
        let mut g2: AdjMap<Undirected> = generator2.generate();

        if g1.node_count() < g2.node_count() {
            std::mem::swap(&mut g1, &mut g2);
        }

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_isomorphism_on_undirected_path_graph(generator: PathGraph) {
        let g1: AdjMap<Undirected> = generator.generate();
        let g2: AdjMap<Undirected> = generator.generate();

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Graph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_directed_path_graph(
        generator1: PathGraph,
        generator2: PathGraph,
    ) {
        let mut g1: AdjMap<Directed> = generator1.generate();
        let mut g2: AdjMap<Directed> = generator2.generate();

        if g1.node_count() < g2.node_count() {
            std::mem::swap(&mut g1, &mut g2);
        }

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_path_graph(
        generator1: PathGraph,
        generator2: PathGraph,
    ) {
        let mut g1: AdjMap<Undirected> = generator1.generate();
        let mut g2: AdjMap<Undirected> = generator2.generate();

        if g1.node_count() < g2.node_count() {
            std::mem::swap(&mut g1, &mut g2);
        }

        let are_isomorph = VF2Isomorphism::init(&g1, &g2, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_wheel_graph(node_count: usize) {
        let node_count = node_count % 28 + 4;
        let star: AdjMap<Undirected> = StarGraph::init(node_count).generate();
        let wheel: AdjMap<Undirected> = WheelGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(&wheel, &star, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_lollipop_graph(
        complete_graph_size: usize,
        path_graph_size: usize,
    ) {
        let complete_graph_size = complete_graph_size % 6 + 4;
        let path_graph_size = path_graph_size % 9 + 2;

        let path_graph: AdjMap<Undirected> = PathGraph::init(path_graph_size).generate();

        let lollipop_graph: AdjMap<Undirected> =
            LollipopGraph::init(complete_graph_size, path_graph_size).generate();

        let are_isomorph =
            VF2Isomorphism::init(&lollipop_graph, &path_graph, IsomorphismType::Subgraph).execute();

        assert!(are_isomorph)
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_cycle_graph(node_count: usize) {
        let node_count = node_count % 7 + 4;

        let cycle_graph: AdjMap<Undirected> = CycleGraph::init(node_count).generate();

        for i in 4..node_count {
            let path_graph: AdjMap<Undirected> = PathGraph::init(i).generate();

            let are_isomorph =
                VF2Isomorphism::init(&cycle_graph, &path_graph, IsomorphismType::Subgraph)
                    .execute();

            assert!(are_isomorph)
        }
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_directed_cycle_graph(node_count: usize) {
        let node_count = node_count % 7 + 4;

        let cycle_graph: AdjMap<Directed> = CycleGraph::init(node_count).generate();

        for i in 4..node_count {
            let path_graph: AdjMap<Directed> = PathGraph::init(i).generate();

            let are_isomorph =
                VF2Isomorphism::init(&cycle_graph, &path_graph, IsomorphismType::Subgraph)
                    .execute();

            assert!(are_isomorph)
        }
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_ladder_graph(node_count: usize) {
        let node_count = node_count % 7 + 4;

        let ladder_graph: AdjMap<Undirected> = LadderGraph::init(node_count).generate();

        for i in 2..node_count {
            let path_graph: AdjMap<Undirected> = PathGraph::init(i).generate();

            let are_isomorph =
                VF2Isomorphism::init(&ladder_graph, &path_graph, IsomorphismType::Subgraph)
                    .execute();

            assert!(are_isomorph)
        }
    }

    #[quickcheck]
    fn vf2_subgraph_isomorphism_on_undirected_circular_ladder_graph(node_count: usize) {
        let node_count = node_count % 7 + 4;

        let circular_ladder_graph: AdjMap<Undirected> =
            CircularLadderGraph::init(node_count).generate();
        let cycle_graph: AdjMap<Undirected> = CycleGraph::init(node_count).generate();

        let are_isomorph = VF2Isomorphism::init(
            &circular_ladder_graph,
            &cycle_graph,
            IsomorphismType::Subgraph,
        )
        .execute();

        assert!(are_isomorph)
    }
}
