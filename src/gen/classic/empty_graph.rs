use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct EmptyGraphGenerator {
    vertex_count: usize,
}

impl EmptyGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        EmptyGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S, Undirected> for EmptyGraphGenerator
where
    S: Storage<Dir = Undirected> + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        for _ in 0..self.vertex_count {
            storage.add_vertex(rng.gen());
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::EmptyGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for EmptyGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for EmptyGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 1;

            EmptyGraphGenerator::init(vertex_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        gen::Generator,
        provide::Edges,
        storage::{edge::Undirected, AdjMap},
    };

    use super::EmptyGraphGenerator;

    #[quickcheck]
    fn prop_gen_empty_graph(generator: EmptyGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert_eq!(graph.edge_count(), 0);
    }
}
