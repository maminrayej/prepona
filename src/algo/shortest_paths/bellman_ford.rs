use std::collections::HashMap;

use anyhow::Result;
use magnitude::Magnitude;

use crate::algo::errors::AlgoError;
use crate::common::{RealID, VirtID};
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;
use crate::view::GenericView;

pub fn bellman_ford<G, F>(
    graph: &G,
    src_rid: RealID,
    cost_of: F,
) -> Result<(GenericView<'_, G>, HashMap<usize, isize>)>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
    F: Fn(usize) -> isize,
{
    let vertex_count = graph.vertex_count();
    let id_map = graph.id_map();
    let src_vid = id_map[src_rid];

    let mut costs = vec![Magnitude::PosInfinite; vertex_count];
    let mut used_edges = vec![None; vertex_count];

    costs[src_vid.inner()] = 0.into();

    for _ in 1..vertex_count {
        for eid in graph.edge_tokens() {
            let (sid, did, _) = graph.edge(eid);

            let s_vid = id_map[RealID::from(sid)];
            let d_vid = id_map[RealID::from(did)];

            let s_cost = costs[s_vid.inner()];
            let d_cost = costs[d_vid.inner()];

            let new_cost = s_cost + cost_of(eid).into();
            if new_cost.is_finite() && new_cost < d_cost {
                costs[d_vid.inner()] = new_cost;
                used_edges[d_vid.inner()] = Some(eid);
            }
        }
    }

    for eid in graph.edge_tokens() {
        let (sid, did, _) = graph.edge(eid);

        let s_vid = id_map[RealID::from(sid)];
        let d_vid = id_map[RealID::from(did)];

        let s_cost = costs[s_vid.inner()];
        let d_cost = costs[d_vid.inner()];

        let new_cost = s_cost + cost_of(eid).into();
        if new_cost.is_finite() && new_cost < d_cost {
            return Err(
                AlgoError::InvalidArgument("Graph contains negative cycle".to_string()).into(),
            );
        }
    }

    let tree_view = GenericView::init(
        graph,
        |vid| {
            let v_vid = id_map[RealID::from(vid)];
            costs[v_vid.inner()].is_finite()
        },
        |eid| used_edges.contains(&Some(eid)),
    );

    let cost_map = costs
        .iter()
        .enumerate()
        .filter_map(|(index, cost)| {
            let v_vid = VirtID::from(index);
            let v_rid = id_map[v_vid];

            if costs[v_vid.inner()].is_finite() {
                Some((v_rid.inner(), cost.unwrap()))
            } else {
                None
            }
        })
        .collect();

    Ok((tree_view, cost_map))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::algo::shortest_paths::bellman_ford;
    use crate::common::RealID;
    use crate::gen::{
        CompleteGraphGenerator, CycleGraphGenerator, EmptyGraphGenerator, Generator,
        PathGraphGenerator,
    };
    use crate::provide::{Edges, Vertices};
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;
    #[quickcheck]
    fn prop_bellman_ford_on_empty_graph(generator: EmptyGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        if graph.vertex_count() > 0 {
            let (tree_view, costs) = bellman_ford(&graph, RealID::from(0), |_| 1).unwrap();

            assert_eq!(tree_view.vertex_count(), 1);
            assert_eq!(tree_view.edge_count(), 0);
            assert_eq!(costs.len(), 1);
            assert_eq!(costs[&0], 0);
        }
    }

    #[quickcheck]
    fn prop_bellman_ford_on_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let (tree_view, costs) = bellman_ford(&graph, RealID::from(0), |_| 1).unwrap();

        assert_eq!(tree_view.vertex_count(), graph.vertex_count());
        assert_eq!(tree_view.edge_count(), tree_view.vertex_count() - 1);
        tree_view.vertex_tokens().for_each(|vid| {
            let cost = costs[&vid];

            if vid == 0 {
                assert_eq!(cost, 0);
            } else {
                assert_eq!(cost, 1);
            }
        });
    }

    #[quickcheck]
    fn prop_bellman_ford_on_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        // Find the vertex with in degree of zero.
        // Every other vertex must be accessible from this vertex.
        let start_id = graph
            .vertex_tokens()
            .find(|vid| graph.ingoing_edges(*vid).count() == 0)
            .unwrap();

        let (tree_view, costs) = bellman_ford(&graph, RealID::from(start_id), |_| 1).unwrap();

        assert_eq!(tree_view.vertex_count(), graph.vertex_count());
        assert_eq!(tree_view.edge_count(), tree_view.vertex_count() - 1);
        assert_eq!(tree_view.edge_count(), graph.edge_count());

        let mut sorted_costs = costs.values().collect_vec();
        sorted_costs.sort();

        sorted_costs.iter().enumerate().for_each(|(index, cost)| {
            assert_eq!(index as isize, **cost);
        });
    }

    #[quickcheck]
    fn prop_bellman_ford_on_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        // Because the graph is circular, it doesn't matter from which vertex to start the
        // bellman_ford.
        let (tree_view, costs) = bellman_ford(&graph, RealID::from(0), |_| 1).unwrap();

        assert_eq!(tree_view.vertex_count(), graph.vertex_count());
        assert_eq!(tree_view.edge_count(), tree_view.vertex_count() - 1);
        assert_eq!(tree_view.edge_count(), graph.edge_count() - 1);

        let mut sorted_costs = costs.values().collect_vec();
        sorted_costs.sort();

        sorted_costs.iter().enumerate().for_each(|(index, cost)| {
            assert_eq!(index as isize, **cost);
        });
    }

    #[quickcheck]
    fn prop_bellman_ford_on_graph_with_negative_cycle(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        assert!(bellman_ford(&graph, RealID::from(0), |_| -1).is_err())
    }
}
