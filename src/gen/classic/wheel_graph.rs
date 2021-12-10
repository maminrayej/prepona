use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::{
    provide::{Edges, InitializableStorage, MutEdges, MutVertices, Vertices},
    storage::edge::Undirected,
};

use super::CycleGraphGenerator;
use crate::gen::Generator;

#[derive(Debug)]
pub struct WheelGraphGenerator {
    vertex_count: usize,
}

impl WheelGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 4 {
            panic!("Vertex count must be atleast 4 to form a wheel graph")
        }

        WheelGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S, Undirected> for WheelGraphGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutVertices + MutEdges,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage: S = CycleGraphGenerator::init(self.vertex_count - 1).generate();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = storage.vertex_tokens().collect();
        let universal_vt = storage.add_vertex(rng.gen());

        for other_vt in vertex_tokens {
            storage.add_edge(universal_vt, other_vt, rng.gen());
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::WheelGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for WheelGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for WheelGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 4;

            WheelGraphGenerator::init(vertex_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::{
        gen::Generator,
        provide::{Edges, MutVertices, Vertices},
        storage::{edge::Undirected, AdjMap},
    };

    use super::WheelGraphGenerator;

    #[quickcheck]
    fn prop_gen_wheel_graph(generator: WheelGraphGenerator) {
        let mut graph: AdjMap<(), (), Undirected> = generator.generate();

        let vt_out_degree: Vec<(usize, usize)> = graph
            .vertex_tokens()
            .map(|vt| (vt, graph.outgoing_edges(vt).count()))
            .collect();

        let universal_vt_index = vt_out_degree
            .iter()
            .position_max_by(|(_, deg1), (_, deg2)| deg1.cmp(deg2))
            .unwrap();

        let (universal_vt, universal_vt_deg) = vt_out_degree[universal_vt_index];

        assert_eq!(universal_vt_deg, graph.vertex_count() - 1);

        assert!(graph.has_vt(universal_vt));

        // If we remove the universal vertex, we have a cycle
        graph.remove_vertex(universal_vt);

        for vt in graph.vertex_tokens() {
            assert_eq!(graph.neighbors(vt).filter(|n_vt| *n_vt != vt).count(), 2);
        }

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 2)
                .count(),
            graph.vertex_count()
        );
    }
}
