use super::errors::AlgoError;
use crate::provide::{Edges, Storage, Vertices};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

pub fn edge_boundry<G>(
    graph: &G,
    s_filter: impl Fn(&usize) -> bool,
    t_filter: impl Fn(&usize) -> bool,
) -> Result<Vec<usize>>
where
    G: Storage + Vertices + Edges,
{
    let s: HashSet<usize> = graph.vertex_tokens().filter(s_filter).collect();
    let t: HashSet<usize> = graph.vertex_tokens().filter(t_filter).collect();

    if !s.is_disjoint(&t) {
        return Err(AlgoError::NotDisjointSets.into());
    }

    let boundry_edges = graph
        .edge_tokens()
        .filter(|eid| {
            let (sid, did, _) = graph.edge(*eid);

            (s.contains(&sid) && t.contains(&did)) || (s.contains(&did) && t.contains(&sid))
        })
        .collect_vec();

    Ok(boundry_edges)
}

pub fn node_boundry<G>(
    graph: &G,
    s_filter: impl Fn(&usize) -> bool,
    t_filter: impl Fn(&usize) -> bool,
) -> Result<Vec<usize>>
where
    G: Storage + Vertices + Edges,
{
    let s: HashSet<usize> = graph.vertex_tokens().filter(s_filter).collect();
    let t: HashSet<usize> = graph.vertex_tokens().filter(t_filter).collect();

    if !s.is_disjoint(&t) {
        return Err(AlgoError::NotDisjointSets.into());
    }

    let node_boundries = s
        .iter()
        .flat_map(|sid| {
            graph
                .neighbors(*sid)
                .filter(|nid| !s.contains(nid) && t.contains(nid))
        })
        .collect_vec();

    Ok(node_boundries)
}
