use std::collections::{BinaryHeap, HashMap, HashSet};

use magnitude::Magnitude;

use crate::common::{RealID, VirtID};
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::Directed;
use crate::view::GenericView;

#[derive(PartialEq, Eq)]
struct VertexScore(RealID, Magnitude<usize>);

impl PartialOrd for VertexScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for VertexScore {
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
    let mut visited_edges = HashSet::new();
    let mut heap = BinaryHeap::new();

    visited[src_vid.inner()] = true;
    costs[src_vid.inner()] = 0.into();
    heap.push(VertexScore(src_rid, 0.into()));

    while let Some(VertexScore(u_rid, u_cost)) = heap.pop() {
        let u_vid = id_map[u_rid];

        if visited[u_vid.inner()] {
            continue;
        }

        if Some(u_rid.inner()) == target_id {
            visited[u_vid.inner()] = true;
            break;
        }

        for eid in graph.outgoing_edges(u_rid.inner()) {
            let (_, v_id, _) = graph.edge(eid);
            let v_rid = RealID::from(v_id);
            let v_vid = id_map[v_rid];

            if visited[v_vid.inner()] {
                continue;
            }

            let cost = u_cost + cost_of(eid).into();
            let current_cost = costs[v_vid.inner()];

            if cost < current_cost {
                costs[v_vid.inner()] = cost;
                visited_edges.insert(eid);
                heap.push(VertexScore(v_rid, cost));
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
        |eid| visited_edges.contains(&eid),
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
