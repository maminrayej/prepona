use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{Edges, InitializableStorage, MutStorage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct CycleGraphGenerator {
    vertex_count: usize,
}

impl CycleGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a cycle graph")
        }

        CycleGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S, Undirected> for CycleGraphGenerator
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

        let vertex_tokens: Vec<usize> = (0..self.vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for (src_vt, dst_vt) in vertex_tokens.into_iter().circular_tuple_windows() {
            storage.add_edge(src_vt, dst_vt, rng.gen());
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::CycleGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for CycleGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for CycleGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 3;

            CycleGraphGenerator::init(vertex_count)
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

    use super::CycleGraphGenerator;

    #[quickcheck]
    fn prop_gen_cycle_graph(generator: CycleGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        for vt in graph.vertex_tokens() {
            assert_eq!(graph.neighbors(vt).filter(|n_vt| *n_vt != vt).count(), 2);
        }

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 2)
                .count(),
            graph.vertex_count()
        );
    }
}
