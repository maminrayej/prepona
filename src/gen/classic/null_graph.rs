use rand::{distributions::Standard, prelude::Distribution};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

#[derive(Debug)]
pub struct NullGraphGenerator;

impl<S> Generator<S> for NullGraphGenerator
where
    S: InitializableStorage + MutStorage,
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
        storage::AdjMap,
    };

    use super::NullGraphGenerator;

    #[quickcheck]
    fn prop_gen_null_graph(generator: NullGraphGenerator) {
        let graph: AdjMap<(), (), false> = generator.generate();

        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }
}
