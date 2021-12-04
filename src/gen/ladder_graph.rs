use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use super::Generator;

pub struct LadderGraphGenerator {
    vertex_count: usize,
}

impl LadderGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count % 2 != 0 {
            panic!("Number of vertices must be an even number to generate a ladder graph")
        }

        LadderGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for LadderGraphGenerator
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

        let mut index = 0;
        let (odd, even): (Vec<usize>, Vec<usize>) = vertex_tokens.into_iter().partition(|_| {
            index += 1;
            index % 2 == 0
        });

        for vts in odd.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        for vts in even.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        for (src_vt, dst_vt) in odd.iter().zip(even.iter()) {
            storage.add_edge(*src_vt, *dst_vt, rng.gen());
        }

        storage
    }
}
