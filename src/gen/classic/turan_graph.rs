use itertools::repeat_n;
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    provide::{Edges, InitializableStorage, MutEdges, MutVertices, Vertices},
    storage::edge::Undirected,
};

use super::CompleteMultiPartiteGraphGenerator;
use crate::gen::Generator;

#[derive(Debug)]
pub struct TuranGraphGenerator {
    vertex_count: usize,
    partition_count: usize,
}

impl TuranGraphGenerator {
    pub fn init(vertex_count: usize, partition_count: usize) -> Self {
        TuranGraphGenerator {
            vertex_count,
            partition_count,
        }
    }

    pub fn set_sizes_of(vertex_count: usize, partition_count: usize) -> Vec<usize> {
        if partition_count < 1 || partition_count > vertex_count {
            panic!("Partition size must be between 1 and vertex count: 1 <= partition size <= vertex count")
        }

        repeat_n(
            vertex_count / partition_count,
            partition_count - (vertex_count % partition_count),
        )
        .chain(repeat_n(
            vertex_count / partition_count + 1,
            vertex_count % partition_count,
        ))
        .collect()
    }
}

impl<S> Generator<S, Undirected> for TuranGraphGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutVertices + MutEdges,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let set_sizes =
            TuranGraphGenerator::set_sizes_of(self.vertex_count, self.partition_count).into_iter();

        CompleteMultiPartiteGraphGenerator::init(set_sizes).generate()
    }
}

#[cfg(test)]
mod test {
    use super::TuranGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for TuranGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
                partition_count: self.partition_count.clone(),
            }
        }
    }

    impl Arbitrary for TuranGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 1;
            let partition_count = usize::arbitrary(g) % vertex_count + 1;

            TuranGraphGenerator::init(vertex_count, partition_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        gen::Generator,
        provide::{Edges, Vertices},
        storage::{edge::Undirected, AdjMap},
    };

    use super::TuranGraphGenerator;

    #[quickcheck]
    fn prop_gen_turan_graph(generator: TuranGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let vertex_count = generator.vertex_count;
        let partition_count = generator.partition_count;

        let set_sizes = TuranGraphGenerator::set_sizes_of(vertex_count, partition_count);

        assert_eq!(set_sizes.iter().copied().sum::<usize>(), vertex_count);

        for set_size in &set_sizes {
            assert_eq!(
                graph
                    .vertex_tokens()
                    .map(|vt| graph.outgoing_edges(vt).count())
                    .filter(|out_degree| *out_degree == vertex_count - *set_size)
                    .count(),
                set_sizes.iter().filter(|ss| *ss == set_size).sum::<usize>()
            );
        }
    }
}
