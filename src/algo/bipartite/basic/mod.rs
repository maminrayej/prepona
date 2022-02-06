use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use itertools::{Either, Itertools};

use crate::algo::component::connected_components;
use crate::algo::errors::AlgoError;
use crate::common::{RealID, VirtID};
use crate::provide::{Edges, Storage, Vertices};
use crate::storage::edge::{Direction, Undirected};
use crate::view::GenericView;

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

pub fn color<G>(graph: &G) -> Result<(HashSet<RealID>, HashSet<RealID>)>
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
                        return Err(
                            AlgoError::NotBipartite("Graph is not bipartite".to_string()).into(),
                        );
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
        .partition_map(|(index, color)| {
            if color == Color::Blue {
                Either::Left(id_map[VirtID::from(index)])
            } else {
                Either::Right(id_map[VirtID::from(index)])
            }
        }))
}

pub fn is_bipartite<G>(graph: &G) -> bool
where
    G: Storage + Vertices,
{
    color(graph).is_ok()
}

pub fn is_bipartite_node_set<G>(
    graph: &G,
    top_vertex_ids: impl Iterator<Item = usize>,
) -> Result<bool>
where
    G: Storage<Dir = Undirected> + Vertices + Edges,
{
    let top_vertex_set: HashSet<RealID> = top_vertex_ids.map(|vid| RealID::from(vid)).collect();

    for cc in connected_components(graph) {
        let view = GenericView::init(graph, |vid| cc.contains(&RealID::from(vid)), |_| true);

        if let Ok((x, y)) = color(&view) {
            if !((x.is_subset(&top_vertex_set) && y.is_disjoint(&top_vertex_set))
                || (y.is_subset(&top_vertex_set) && x.is_disjoint(&top_vertex_set)))
            {
                return Ok(false);
            }
        } else {
            return Err(AlgoError::NotBipartite(
                "Graph contains a connected component that is not bipartite".to_string(),
            )
            .into());
        }
    }

    Ok(true)
}

pub fn density<G>(graph: &G, top_vertex_ids: impl Iterator<Item = usize>) -> f64
where
    G: Storage + Vertices + Edges,
{
    let n = graph.vertex_count() as f64;
    let m = graph.edge_count() as f64;
    let nt = top_vertex_ids.count() as f64;
    let nb = n - nt;

    return if m == 0.0 {
        return 0.0;
    } else {
        if G::Dir::is_directed() {
            m / (2.0 * (nt * nb))
        } else {
            m / (nt * nb)
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use quickcheck_macros::quickcheck;

    use crate::algo::bipartite;
    use crate::common::RealID;
    use crate::gen::{
        BarbellGraphGenerator, CompleteGraphGenerator, CycleGraphGenerator, EmptyGraphGenerator,
        Generator, LadderGraphGenerator, LollipopGraphGenerator, NullGraphGenerator,
        PathGraphGenerator, StarGraphGenerator, WheelGraphGenerator,
    };
    use crate::provide::{Edges, MutEdges, MutVertices, Vertices};
    use crate::storage::edge::Undirected;
    use crate::storage::AdjMap;

    fn assert_coloring<G>(
        graph: &G,
        (top_vertices, bottom_vertices): (HashSet<RealID>, HashSet<RealID>),
    ) where
        G: Vertices,
    {
        for vid in graph.vertex_tokens() {
            let v_rid = RealID::from(vid);

            for nid in graph.neighbors(vid) {
                let n_rid = RealID::from(nid);

                assert!(
                    top_vertices.contains(&v_rid) && bottom_vertices.contains(&n_rid)
                        || bottom_vertices.contains(&v_rid) && top_vertices.contains(&n_rid)
                )
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

    #[quickcheck]
    fn prop_two_coloring_node_set(generator: PathGraphGenerator) {
        let mut path1: AdjMap<(), (), Undirected> = generator.generate();
        let path2: AdjMap<(), (), Undirected> = generator.generate();

        let (path1_top_vertices, _) = bipartite::color(&path1).unwrap();
        let (path2_top_vertices, _) = bipartite::color(&path2).unwrap();

        let vid_map: HashMap<usize, usize> = path2
            .vertex_tokens()
            .map(|vid| (vid, path1.add_vertex(())))
            .collect();

        path2.edge_tokens().for_each(|eid| {
            let (sid, did, _) = path2.edge(eid);

            let new_sid = vid_map[&sid];
            let new_did = vid_map[&did];

            path1.add_edge(new_sid, new_did, ());
        });

        let top_vertices = vid_map
            .into_iter()
            .filter(|(vid, _)| path2_top_vertices.contains(&RealID::from(*vid)))
            .map(|(_, new_vid)| new_vid)
            .chain(path1_top_vertices.into_iter().map(|v_rid| v_rid.inner()));

        assert!(bipartite::is_bipartite_node_set(&path1, top_vertices).unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_node_set_wrong_top_vertices(generator: PathGraphGenerator) {
        let mut path1: AdjMap<(), (), Undirected> = generator.generate();
        let path2: AdjMap<(), (), Undirected> = generator.generate();

        let (path1_top_vertices, path2_bottom_vertices) = bipartite::color(&path1).unwrap();
        let (path2_top_vertices, _) = bipartite::color(&path2).unwrap();

        let vid_map: HashMap<usize, usize> = path2
            .vertex_tokens()
            .map(|vid| (vid, path1.add_vertex(())))
            .collect();

        path2.edge_tokens().for_each(|eid| {
            let (sid, did, _) = path2.edge(eid);

            let new_sid = vid_map[&sid];
            let new_did = vid_map[&did];

            path1.add_edge(new_sid, new_did, ());
        });

        let top_vertices = vid_map
            .into_iter()
            .filter(|(vid, _)| path2_top_vertices.contains(&RealID::from(*vid)))
            .map(|(_, new_vid)| new_vid)
            .chain(path1_top_vertices.into_iter().map(|v_rid| v_rid.inner()))
            .chain(path2_bottom_vertices.into_iter().map(|v_rid| v_rid.inner()));

        assert!(!bipartite::is_bipartite_node_set(&path1, top_vertices).unwrap());
    }

    #[quickcheck]
    fn prop_two_coloring_node_set_fail(generator: PathGraphGenerator) {
        let mut graph1: AdjMap<(), (), Undirected> = generator.generate();
        let (top_vertices, _) = bipartite::color(&graph1).unwrap();

        CompleteGraphGenerator::add_component_to(&mut graph1, 4);

        assert!(bipartite::is_bipartite_node_set(
            &graph1,
            top_vertices.into_iter().map(|v_rid| v_rid.inner())
        )
        .is_err())
    }
}
