use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::CompleteGraphGenerator;
use crate::gen::Generator;

pub struct BarbellGraphGenerator {
    first_complete_graph_size: usize,
    second_complete_graph_size: usize,
    bridge_size: usize,
}

impl BarbellGraphGenerator {
    pub fn init(
        first_complete_graph_size: usize,
        second_complete_graph_size: usize,
        bridge_size: usize,
    ) -> Self {
        // TODO: Validate sizes
        BarbellGraphGenerator {
            first_complete_graph_size,
            second_complete_graph_size,
            bridge_size,
        }
    }
}

impl<S> Generator<S> for BarbellGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        // Generate first compelete graph
        let mut storage: S =
            CompleteGraphGenerator::init(self.first_complete_graph_size).generate();
        let first_graph_tokens: Vec<usize> = storage.vertex_tokens().collect();

        let mut rng = thread_rng();

        // Create second compelete graph
        let second_graph_tokens: Vec<usize> = (0..self.second_complete_graph_size)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let vt_pairs = second_graph_tokens
            .iter()
            .copied()
            .cartesian_product(second_graph_tokens.iter().copied());

        for (src_vt, dst_vt) in vt_pairs {
            storage.add_edge(src_vt, dst_vt, rng.gen());
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
