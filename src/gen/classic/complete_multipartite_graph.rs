use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

pub struct CompleteMultiPartiteGraphGenerator {
    set_sizes: Vec<usize>,
}

impl CompleteMultiPartiteGraphGenerator {
    pub fn init(set_sizes: impl Iterator<Item = usize>) -> Self {
        // TODO: Validate sizes
        CompleteMultiPartiteGraphGenerator {
            set_sizes: set_sizes.collect(),
        }
    }
}

impl<S> Generator<S> for CompleteMultiPartiteGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let mut first_set_tokens: Vec<usize> = (0..self.set_sizes[0])
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for set_size in self.set_sizes.iter().skip(1).copied() {
            let second_set_tokens: Vec<usize> = (0..set_size)
                .map(|_| storage.add_vertex(rng.gen()))
                .collect();

            let vt_pairs = first_set_tokens
                .iter()
                .copied()
                .cartesian_product(second_set_tokens.iter().copied());

            for (src_vt, dst_vt) in vt_pairs {
                storage.add_edge(src_vt, dst_vt, rng.gen());
            }

            first_set_tokens = second_set_tokens;
        }

        storage
    }
}
