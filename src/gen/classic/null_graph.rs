use rand::{distributions::Standard, prelude::Distribution};

use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct NullGraphGenerator;

impl<S> Generator<S, Undirected> for NullGraphGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutVertices + MutEdges,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        S::init()
    }
}

#[cfg(test)]
mod test {
    use super::NullGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for NullGraphGenerator {
        fn clone(&self) -> Self {
            Self {}
        }
    }

    impl Arbitrary for NullGraphGenerator {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            NullGraphGenerator
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

    use super::NullGraphGenerator;

    #[quickcheck]
    fn prop_gen_null_graph(generator: NullGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
}
