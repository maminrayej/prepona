use std::collections::HashMap;

use itertools::Itertools;

use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::{Directed, Direction};

pub fn degree_centrality<G>(graph: &G) -> HashMap<usize, usize>
where
    G: Storage + Vertices + Edges,
{
    graph
        .vertex_tokens()
        .map(|vid| {
            let mut d = graph.ingoing_edges(vid).count();

            if G::Dir::is_directed() {
                d += graph.outgoing_edges(vid).count();
            }

            // FIXME: it assumes there is only one loop. what if there is more?
            if graph.neighbors(vid).contains(&vid) {
                d += 1;
            }

            (vid, d)
        })
        .collect()
}

pub fn in_degree_centrality<G>(graph: &G) -> HashMap<usize, usize>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    // FIXME: How loops and multi edges affect this?
    graph
        .vertex_tokens()
        .map(|vid| {
            let d = graph.ingoing_edges(vid).count();

            (vid, d)
        })
        .collect()
}

pub fn out_degree_centrality<G>(graph: &G) -> HashMap<usize, usize>
where
    G: Storage<Dir = Directed> + Vertices + Edges,
{
    // FIXME: How loops and multi edges affect this?
    graph
        .vertex_tokens()
        .map(|vid| {
            let d = graph.outgoing_edges(vid).count();

            (vid, d)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::algo::centrality::{degree_centrality, in_degree_centrality, out_degree_centrality};
    use crate::gen::{CompleteGraphGenerator, Generator, LadderGraphGenerator, CycleGraphGenerator};
    use crate::provide::Vertices;
    use crate::storage::edge::{Directed, Undirected};
    use crate::storage::AdjMap;

    #[quickcheck]
    fn prop_degree_centrality_of_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let degree_map = degree_centrality(&graph);

        assert!(degree_map
            .into_iter()
            .all(|(_, degree)| degree == graph.vertex_count() - 1));
    }

    #[quickcheck]
    fn prop_degree_centrality_of_complete_graph_directed(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let degree_map = degree_centrality(&graph);
        let out_degree_map = out_degree_centrality(&graph);
        let in_degree_map = in_degree_centrality(&graph);

        assert!(degree_map
            .into_iter()
            .all(|(_, degree)| degree == 2 * (graph.vertex_count() - 1)));

        assert!(out_degree_map
            .into_iter()
            .all(|(_, degree)| degree == graph.vertex_count() - 1));

        assert!(in_degree_map
            .into_iter()
            .all(|(_, degree)| degree ==  graph.vertex_count() - 1));
    }

    #[quickcheck]
    fn prop_degree_centrality_of_ladder_graph(generator: LadderGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let degree_map = degree_centrality(&graph);

        if graph.vertex_count() == 2 {
            assert!(degree_map.iter().all(|(_, degree)| *degree == 1));
        } else {
            assert_eq!(
                degree_map
                    .iter()
                    .filter(|(_, degree)| **degree == 2)
                    .count(),
                4
            );
            assert_eq!(
                degree_map
                    .iter()
                    .filter(|(_, degree)| **degree == 3)
                    .count(),
                graph.vertex_count() - 4
            );
        }
    }

    #[quickcheck]
    fn prop_degree_centrality_of_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Directed> = generator.generate();

        let degree_map = degree_centrality(&graph);
        let in_degree_map = in_degree_centrality(&graph);
        let out_degree_map = out_degree_centrality(&graph);
    
        assert!(degree_map.iter().all(|(_, degree)| *degree == 2));
        assert!(in_degree_map.iter().all(|(_, degree)| *degree == 1));
        assert!(out_degree_map.iter().all(|(_, degree)| *degree == 1));
    }

}
