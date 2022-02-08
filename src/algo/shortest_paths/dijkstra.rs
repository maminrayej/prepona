use std::collections::{BinaryHeap, HashMap};

use magnitude::Magnitude;

use crate::common::{RealID, VirtID};
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;
use crate::view::GenericView;

#[derive(PartialEq, Eq)]
struct VertexCost(RealID, Magnitude<usize>);

impl PartialOrd for VertexCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Magnitude doesn't allow two infinite values to be compared to each other since it's
        // undefined. So as long as at least one of the values is finite, we can compare them.
        // If both are infinite values, we declare them equal since we don't care about the
        // ordering of infinite costs.
        if self.1.is_finite() || other.1.is_finite() {
            self.1.partial_cmp(&other.1)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for VertexCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn dijkstra<G, F>(
    graph: &G,
    src_rid: RealID,
    target_id: Option<usize>,
    cost_of: F,
) -> (GenericView<'_, G>, HashMap<usize, usize>)
where
    G: Storage<Dir = Directed> + Vertices + Edges,
    F: Fn(usize) -> usize,
{
    let vertex_count = graph.vertex_count();
    let id_map = graph.id_map();
    let src_vid = id_map[src_rid];

    let mut visited = vec![false; vertex_count];
    let mut costs = vec![Magnitude::PosInfinite; vertex_count];
    let mut used_edge = vec![None; vertex_count];
    let mut heap = BinaryHeap::new();

    costs[src_vid.inner()] = 0.into();
    heap.push(VertexCost(src_rid, 0.into()));

    while let Some(VertexCost(u_rid, u_cost)) = heap.pop() {
        let u_vid = id_map[u_rid];

        // A vertex has been visited when it had been poped from the heap already. Also, all of its
        // neighbors had been updated. So there is no point in visiting the vertex again
        // because its cost can not improve as it was poped from the stack (it's already at its
        // minimum) and it can not improve the cost of its neighbors either for the same reason.
        if visited[u_vid.inner()] {
            continue;
        }

        // When the potential target vertex is poped from the heap, we terminate the algorithm.
        // Because the cost of reaching the target can not get any lower.
        if Some(u_rid.inner()) == target_id {
            break;
        }

        for eid in graph.outgoing_edges(u_rid.inner()) {
            let (_, v_id, _) = graph.edge(eid);
            let v_rid = RealID::from(v_id);
            let v_vid = id_map[v_rid];

            // When a neighbor is visited, there is no point in recalculating its cost again.
            // Because it was already at its minimum when it had been poped.
            if visited[v_vid.inner()] {
                continue;
            }

            let new_cost = u_cost + cost_of(eid).into();
            let current_cost = costs[v_vid.inner()];

            if new_cost < current_cost {
                costs[v_vid.inner()] = new_cost;
                used_edge[v_vid.inner()] = Some(eid);
                heap.push(VertexCost(v_rid, new_cost));
            }
        }

        visited[u_vid.inner()] = true;
    }

    let tree_view = GenericView::init(
        graph,
        |vid| {
            let v_vid = id_map[RealID::from(vid)];
            visited[v_vid.inner()]
        },
        |eid| used_edge.contains(&Some(eid)),
    );

    let cost_map = costs
        .into_iter()
        .enumerate()
        .filter_map(|(index, cost)| {
            let v_vid = VirtID::from(index);
            let v_rid = id_map[v_vid];

            if visited[index] {
                Some((v_rid.inner(), cost.unwrap()))
            } else {
                None
            }
        })
        .collect();

    (tree_view, cost_map)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::algo::shortest_paths::dijkstra;
    use crate::common::RealID;
    use crate::gen::{
        CompleteGraphGenerator, CycleGraphGenerator, EmptyGraphGenerator, Generator,
        PathGraphGenerator,
    };
    use crate::provide::{Edges, Vertices};
    use crate::storage::edge::Directed;
    use crate::storage::AdjMap;

    #[quickcheck]
    fn prop_dijkstra_on_empty_graph(generator: EmptyGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        if graph.vertex_count() > 0 {
            let (tree_view, costs) = dijkstra(&graph, RealID::from(0), None, |_| 1);

            assert_eq!(tree_view.vertex_count(), 1);
            assert_eq!(tree_view.edge_count(), 0);
            assert_eq!(costs.len(), 1);
            assert_eq!(costs[&0], 0);
        }
    }

    #[quickcheck]
    fn prop_dijkstra_on_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let (tree_view, costs) = dijkstra(&graph, RealID::from(0), None, |_| 1);

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
    fn prop_dijkstra_on_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        // Find the vertex with in degree of zero.
        // Every other vertex must be accessible from this vertex.
        let start_id = graph
            .vertex_tokens()
            .find(|vid| graph.ingoing_edges(*vid).count() == 0)
            .unwrap();

        let (tree_view, costs) = dijkstra(&graph, RealID::from(start_id), None, |_| 1);

        assert_eq!(tree_view.vertex_count(), graph.vertex_count());
        assert_eq!(tree_view.edge_count(), tree_view.vertex_count() - 1);
        assert_eq!(tree_view.edge_count(), graph.edge_count());

        let mut sorted_costs = costs.values().collect_vec();
        sorted_costs.sort();

        sorted_costs.iter().enumerate().for_each(|(index, cost)| {
            assert_eq!(index, **cost);
        });
    }

    #[quickcheck]
    fn prop_dijkstra_on_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        // Because the graph is circular, it doesn't matter from which vertex to start the
        // dijkstra.
        let (tree_view, costs) = dijkstra(&graph, RealID::from(0), None, |_| 1);

        assert_eq!(tree_view.vertex_count(), graph.vertex_count());
        assert_eq!(tree_view.edge_count(), tree_view.vertex_count() - 1);
        assert_eq!(tree_view.edge_count(), graph.edge_count() - 1);

        let mut sorted_costs = costs.values().collect_vec();
        sorted_costs.sort();

        sorted_costs.iter().enumerate().for_each(|(index, cost)| {
            assert_eq!(index, **cost);
        });
    }
}
