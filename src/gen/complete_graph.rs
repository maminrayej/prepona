use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::Generator;

pub struct CompleteGraphGenerator {
    vertex_count: usize,
}

impl CompleteGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        CompleteGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for CompleteGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..self.vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let vt_pairs = vertex_tokens
            .iter()
            .copied()
            .cartesian_product(vertex_tokens.iter().copied());

        for (src_vt, dst_vt) in vt_pairs {
            storage.add_edge(src_vt, dst_vt, rng.gen());
        }

        storage
    }
}
