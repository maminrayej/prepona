use std::collections::{HashMap, VecDeque};

use anyhow::Result;

use crate::algo::errors::AlgoError;
use crate::common::{RealID, VirtID};
use crate::provide::{Storage, Vertices};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Unknown,
}

impl Color {
    pub fn opposite_of(color: Color) -> Color {
        match color {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
            Color::Unknown => panic!("BUG: Unknown does not have an opposite color"),
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, &Self::Unknown)
    }

    pub fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}

pub fn color<G>(graph: &G) -> Result<HashMap<RealID, Color>>
where
    G: Storage + Vertices,
{
    use super::Color::*;

    let id_map = graph.id_map();

    let mut color_of = vec![Unknown; graph.vertex_count()];

    for s_rid in graph.vertex_tokens().map(|vid| RealID::from(vid)) {
        let s_vid = id_map[s_rid];

        if color_of[s_vid.inner()].is_known() {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(s_vid);
        color_of[s_vid.inner()] = Red;

        while !queue.is_empty() {
            let v_vid = queue.pop_front().unwrap();
            let v_rid = id_map[v_vid];

            let color = Color::opposite_of(color_of[v_vid.inner()]);
            for n_rid in graph.neighbors(v_rid.inner()).map(|nid| RealID::from(nid)) {
                let n_vid = id_map[n_rid];

                if color_of[n_vid.inner()].is_known() {
                    if color_of[n_vid.inner()] == color_of[v_vid.inner()] {
                        return Err(AlgoError::GraphIsNotBipartite.into());
                    }
                } else {
                    color_of[n_vid.inner()] = color;
                    queue.push_back(n_vid);
                }
            }
        }
    }

    Ok(color_of
        .into_iter()
        .enumerate()
        .map(|(index, color)| (id_map[VirtID::from(index)], color))
        .collect())
}

pub fn is_bipartite<G>(graph: &G) -> bool
where
    G: Storage + Vertices,
{
    color(graph).is_ok()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck_macros::quickcheck;

    use crate::algo::bipartite;
    use crate::common::RealID;
    use crate::gen::{
        BarbellGraphGenerator, CompleteGraphGenerator, CycleGraphGenerator, EmptyGraphGenerator,
        Generator, LadderGraphGenerator, LollipopGraphGenerator, NullGraphGenerator,
        PathGraphGenerator, StarGraphGenerator, WheelGraphGenerator,
    };
    use crate::provide::Vertices;
    use crate::storage::edge::Undirected;
    use crate::storage::AdjMap;

    use super::Color;

    fn assert_coloring<G>(graph: &G, colors: HashMap<RealID, Color>)
    where
        G: Vertices,
    {
        for vid in graph.vertex_tokens() {
            let v_color = colors[&RealID::from(vid)];

            for nid in graph.neighbors(vid) {
                let n_color = colors[&RealID::from(nid)];

                assert_ne!(v_color, n_color);
            }
        }
    }

    #[quickcheck]
    fn prop_two_coloring_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        if graph.vertex_count() > 2 {
            assert!(bipartite::color(&graph).is_err());
            assert!(!bipartite::is_bipartite(&graph));
        } else {
            let colors = bipartite::color(&graph);
            assert!(colors.is_ok());
            assert!(bipartite::is_bipartite(&graph));
            assert_coloring(&graph, colors.unwrap());
        }
    }

    #[quickcheck]
    fn prop_two_coloring_empty_graph(generator: EmptyGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let colors = bipartite::color(&graph);
        assert!(colors.is_ok());
        assert!(bipartite::is_bipartite(&graph));
        assert_coloring(&graph, colors.unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_null_graph(generator: NullGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let colors = bipartite::color(&graph);
        assert!(colors.is_ok());
        assert!(bipartite::is_bipartite(&graph));
        assert_coloring(&graph, colors.unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        if graph.vertex_count() % 2 == 0 {
            let colors = bipartite::color(&graph);
            assert!(colors.is_ok());
            assert!(bipartite::is_bipartite(&graph));
            assert_coloring(&graph, colors.unwrap());
        } else {
            assert!(bipartite::color(&graph).is_err());
            assert!(!bipartite::is_bipartite(&graph));
        }
    }

    #[quickcheck]
    fn prop_two_coloring_ladder_graph(generator: LadderGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let colors = bipartite::color(&graph);
        assert!(colors.is_ok());
        assert!(bipartite::is_bipartite(&graph));
        assert_coloring(&graph, colors.unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let colors = bipartite::color(&graph);
        assert!(colors.is_ok());
        assert!(bipartite::is_bipartite(&graph));
        assert_coloring(&graph, colors.unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_lollipop_graph(generator: LollipopGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert!(bipartite::color(&graph).is_err());
        assert!(!bipartite::is_bipartite(&graph));
    }

    #[quickcheck]
    fn prop_two_coloring_barbell_graph(generator: BarbellGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert!(bipartite::color(&graph).is_err());
        assert!(!bipartite::is_bipartite(&graph));
    }

    #[quickcheck]
    fn prop_two_coloring_star_graph(generator: StarGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let colors = bipartite::color(&graph);
        assert!(colors.is_ok());
        assert!(bipartite::is_bipartite(&graph));
        assert_coloring(&graph, colors.unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_wheel_graph(generator: WheelGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert!(bipartite::color(&graph).is_err());
        assert!(!bipartite::is_bipartite(&graph));
    }
}
