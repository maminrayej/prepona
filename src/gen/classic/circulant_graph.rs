use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

pub struct CirculantGraphGenerator {
    offsets: Vec<usize>,
    vertex_count: usize,
}

impl CirculantGraphGenerator {
    pub fn init(offsets: impl Iterator<Item = usize>, vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a circulant graph")
        }

        CirculantGraphGenerator {
            offsets: offsets.collect(),
            vertex_count,
        }
    }
}

impl<S> Generator<S> for CirculantGraphGenerator
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

        for (index, src_vt) in vertex_tokens.iter().copied().enumerate() {
            for offset in self.offsets.iter().copied() {
                storage.add_edge(
                    src_vt,
                    vertex_tokens[(index + offset) % self.vertex_count],
                    rng.gen(),
                );
                storage.add_edge(
                    src_vt,
                    vertex_tokens[(index - offset) % self.vertex_count],
                    rng.gen(),
                );
            }
        }

        storage
    }
}