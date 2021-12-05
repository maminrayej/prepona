use itertools::repeat_n;
use rand::{distributions::Standard, prelude::Distribution};

use crate::provide::{InitializableStorage, MutStorage};

use super::CompleteMultiPartiteGraphGenerator;
use crate::gen::Generator;

pub struct TuranGraphGenerator {
    vertex_count: usize,
    partition_count: usize,
}

impl TuranGraphGenerator {
    pub fn init(vertex_count: usize, partition_count: usize) -> Self {
        if partition_count < 1 || partition_count > vertex_count {
            panic!("Partition count must be between 1 and vertex count: 1 <= partition count <= vertex count")
        }

        TuranGraphGenerator {
            vertex_count,
            partition_count,
        }
    }
}

impl<S> Generator<S> for TuranGraphGenerator
where
    S: InitializableStorage + MutStorage,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let set_iter = repeat_n(
            self.vertex_count / self.partition_count,
            self.partition_count - (self.vertex_count % self.partition_count),
        )
        .chain(repeat_n(
            self.vertex_count / self.partition_count + 1,
            self.vertex_count % self.partition_count,
        ));

        CompleteMultiPartiteGraphGenerator::init(set_iter).generate()
    }
}
