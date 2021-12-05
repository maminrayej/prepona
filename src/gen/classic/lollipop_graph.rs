use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::CompleteGraphGenerator;
use crate::gen::Generator;

pub struct LollipopGraphGenerator {
    compelete_graph_size: usize,
    bridge_size: usize,
}

impl LollipopGraphGenerator {
    pub fn init(first_complete_graph_size: usize, bridge_size: usize) -> Self {
        // TODO: Validate sizes
        LollipopGraphGenerator {
            compelete_graph_size: first_complete_graph_size,
            bridge_size,
        }
    }
}

impl<S> Generator<S> for LollipopGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        // Generate the compelete graph
        let mut storage: S = CompleteGraphGenerator::init(self.compelete_graph_size).generate();
        let first_graph_tokens: Vec<usize> = storage.vertex_tokens().collect();

        let mut rng = thread_rng();

        // Create a bridge
        let bridge_vertex_tokens: Vec<usize> = (0..self.bridge_size)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for vts in bridge_vertex_tokens.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        // Connect complete graph to bridge
        storage.add_edge(first_graph_tokens[0], bridge_vertex_tokens[0], rng.gen());

        storage
    }
}
