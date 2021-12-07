use std::fmt::Debug;

use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::CompleteGraphGenerator;
use crate::gen::Generator;

#[derive(Debug)]
pub struct BarbellGraphGenerator {
    complete_graph_size: usize,
    bridge_size: usize,
}

impl BarbellGraphGenerator {
    pub fn init(complete_graph_size: usize, bridge_size: usize) -> Self {
        if complete_graph_size < 3 {
            panic!("Complete graph size must be greater than 2 to generate a barbell graph")
        }

        BarbellGraphGenerator {
            complete_graph_size,
            bridge_size,
        }
    }
}

impl<S> Generator<S> for BarbellGraphGenerator
where
    S: InitializableStorage + MutStorage + Debug,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        // Generate first compelete graph
        let mut storage: S = CompleteGraphGenerator::init(self.complete_graph_size).generate();

        let first_graph_tokens: Vec<usize> = storage.vertex_tokens().collect();

        let mut rng = thread_rng();

        // Create second compelete graph
        let second_graph_tokens: Vec<usize> = (0..self.complete_graph_size)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let vt_pairs = second_graph_tokens
            .iter()
            .copied()
            .cartesian_product(second_graph_tokens.iter().copied());

        for (src_vt, dst_vt) in vt_pairs {
            if src_vt < dst_vt {
                storage.add_edge(src_vt, dst_vt, rng.gen());
            }
        }

        if self.bridge_size == 0 {
            // Connect two graphs directly
            storage.add_edge(first_graph_tokens[0], second_graph_tokens[0], rng.gen());
        } else {
            // Create a bridge
            let bridge_vertex_tokens: Vec<usize> = (0..self.bridge_size)
                .into_iter()
                .map(|_| storage.add_vertex(rng.gen()))
                .collect();

            for vts in bridge_vertex_tokens.windows(2) {
                storage.add_edge(vts[0], vts[1], rng.gen());
            }

            // Connect first graph to bridge
            storage.add_edge(first_graph_tokens[0], bridge_vertex_tokens[0], rng.gen());

            // Connect bridge to second graph
            storage.add_edge(
                bridge_vertex_tokens[self.bridge_size - 1],
                second_graph_tokens[0],
                rng.gen(),
            );
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::BarbellGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for BarbellGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                complete_graph_size: self.complete_graph_size.clone(),
                bridge_size: self.bridge_size.clone(),
            }
        }
    }

    impl Arbitrary for BarbellGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let complete_graph_size = usize::arbitrary(g) % 8 + 3;
            let bridge_size = usize::arbitrary(g) % 8;

            BarbellGraphGenerator::init(complete_graph_size, bridge_size)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        gen::Generator,
        provide::{Edges, Vertices},
        storage::AdjMap,
    };

    use super::BarbellGraphGenerator;

    #[quickcheck]
    fn prop_gen_barbell_graph(generator: BarbellGraphGenerator) {
        let graph: AdjMap<(), (), false> = generator.generate();

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == generator.complete_graph_size - 1)
                .count(),
            generator.complete_graph_size * 2 - 2
                + if generator.complete_graph_size - 1 == 2 {
                    generator.bridge_size
                } else {
                    0
                }
        );
        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == generator.complete_graph_size)
                .count(),
            2
        );

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 2)
                .count(),
            generator.bridge_size
                + if generator.complete_graph_size - 1 == 2 {
                    generator.complete_graph_size * 2 - 2
                } else {
                    0
                }
        );
    }
}
