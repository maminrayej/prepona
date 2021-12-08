use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{Edges, InitializableStorage, MutStorage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct CompleteMultiPartiteGraphGenerator {
    set_sizes: Vec<usize>,
}

impl CompleteMultiPartiteGraphGenerator {
    pub fn init(set_sizes: impl Iterator<Item = usize>) -> Self {
        let set_sizes = set_sizes.collect_vec();

        if set_sizes.iter().any(|size| *size == 0) {
            panic!("Each set must contain at least one element to create a complete multipartite graph")
        }

        CompleteMultiPartiteGraphGenerator { set_sizes }
    }
}

impl<S> Generator<S, Undirected> for CompleteMultiPartiteGraphGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutStorage,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let partitions = (0..self.set_sizes.len())
            .map(|set_index| {
                (0..self.set_sizes[set_index])
                    .map(|_| storage.add_vertex(rng.gen()))
                    .collect_vec()
            })
            .collect_vec();

        // Connect each partition to all other partitions
        for i in 0..partitions.len() {
            for j in (i + 1)..partitions.len() {
                let vt_pairs = partitions[i]
                    .iter()
                    .copied()
                    .cartesian_product(partitions[j].iter().copied());

                for (src_vt, dst_vt) in vt_pairs {
                    storage.add_edge(src_vt, dst_vt, rng.gen());
                }
            }
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::CompleteMultiPartiteGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for CompleteMultiPartiteGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                set_sizes: self.set_sizes.clone(),
            }
        }
    }

    impl Arbitrary for CompleteMultiPartiteGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let set_count = usize::arbitrary(g) % 3 + 1;

            CompleteMultiPartiteGraphGenerator::init(
                (0..set_count).map(|_| usize::arbitrary(g) % 5 + 1),
            )
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

    use super::CompleteMultiPartiteGraphGenerator;

    #[quickcheck]
    fn prop_gen_complete_multi_partite_graph(generator: CompleteMultiPartiteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        let set_sizes = generator.set_sizes;

        let vertex_token_count = graph.vertex_tokens().count();

        for set_size in &set_sizes {
            assert_eq!(
                graph
                    .vertex_tokens()
                    .map(|vt| graph.outgoing_edges(vt).count())
                    .filter(|out_degree| *out_degree == vertex_token_count - *set_size)
                    .count(),
                set_sizes.iter().filter(|ss| *ss == set_size).sum::<usize>()
            );
        }
    }
}
