use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::{
    provide::{Edges, InitializableStorage, MutStorage, Vertices},
    storage::edge::Undirected,
};

use super::{CompleteGraphGenerator, PathGraphGenerator};
use crate::gen::Generator;

#[derive(Debug)]
pub struct LollipopGraphGenerator {
    complete_graph_size: usize,
    bridge_size: usize,
}

impl LollipopGraphGenerator {
    pub fn init(complete_graph_size: usize, bridge_size: usize) -> Self {
        if complete_graph_size < 3 {
            panic!("Complete graph size must be greater than 2 to generate a lollipop graph")
        }
        if bridge_size < 1 {
            panic!("Bridge size must be greater than 2 to generate a lollipop graph")
        }

        LollipopGraphGenerator {
            complete_graph_size,
            bridge_size,
        }
    }
}

impl<S> Generator<S, Undirected> for LollipopGraphGenerator
where
    S: Edges<Dir = Undirected>,
    S: Vertices<Dir = Undirected>,
    S: MutStorage,
    S: InitializableStorage<Dir = Undirected>,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        // Generate the compelete graph
        let mut storage: S = CompleteGraphGenerator::init(self.complete_graph_size).generate();
        let first_graph_tokens: Vec<usize> = storage.vertex_tokens().collect();

        let mut rng = thread_rng();

        // Create a bridge
        let bridge_vertex_tokens: Vec<usize> =
            PathGraphGenerator::add_component_to(&mut storage, self.bridge_size).collect_vec();

        // Connect complete graph to bridge
        storage.add_edge(first_graph_tokens[0], bridge_vertex_tokens[0], rng.gen());

        storage
    }
}

#[cfg(test)]
mod test {
    use super::LollipopGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for LollipopGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                complete_graph_size: self.complete_graph_size.clone(),
                bridge_size: self.bridge_size.clone(),
            }
        }
    }

    impl Arbitrary for LollipopGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let complete_graph_size = usize::arbitrary(g) % 8 + 3;
            let bridge_size = usize::arbitrary(g) % 8 + 1;

            LollipopGraphGenerator::init(complete_graph_size, bridge_size)
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

    use super::LollipopGraphGenerator;

    #[quickcheck]
    fn prop_gen_lollipop_graph(generator: LollipopGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == generator.complete_graph_size - 1)
                .count(),
            generator.complete_graph_size - 1
                + if generator.complete_graph_size - 1 == 2 {
                    generator.bridge_size - 1
                } else {
                    0
                }
        );
        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == generator.complete_graph_size)
                .count(),
            1
        );

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 2)
                .count(),
            generator.bridge_size - 1
                + if generator.complete_graph_size - 1 == 2 {
                    generator.complete_graph_size - 1
                } else {
                    0
                }
        );

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 1)
                .count(),
            1
        );
    }
}
