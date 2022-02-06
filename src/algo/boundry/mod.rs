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

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::algo::boundry::{edge_boundry, node_boundry};
    use crate::gen::{CompleteGraphGenerator, Generator};
    use crate::provide::Edges;
    use crate::storage::edge::Undirected;
    use crate::storage::AdjMap;

    #[quickcheck]
    fn prop_edge_boundry(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let boundry_edges = edge_boundry(&graph, |vid| vid % 2 == 0, |vid| vid % 2 != 0).unwrap();

        assert!(graph
            .edge_tokens()
            .filter(|eid| boundry_edges.contains(eid))
            .all(|eid| {
                let (sid, did, _) = graph.edge(eid);

                (sid % 2) != (did % 2)
            }))
    }

    #[quickcheck]
    fn prop_edge_boundry_fail(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert!(edge_boundry(&graph, |vid| vid % 2 == 0, |vid| vid % 2 == 0).is_err());
    }

    #[quickcheck]
    fn prop_vertex_boundry(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let boundry_vertices =
            node_boundry(&graph, |vid| vid % 2 == 0, |vid| vid % 2 != 0).unwrap();

        assert!(boundry_vertices.into_iter().all(|vid| vid % 2 != 0))
    }

    #[quickcheck]
    fn prop_vertex_boundry_fail(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert!(node_boundry(&graph, |vid| vid % 2 == 0, |vid| vid % 2 == 0).is_err());
    }
}
