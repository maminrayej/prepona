use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::common::DynIter;
use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct PathGraphGenerator {
    vertex_count: usize,
}

impl PathGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 2 {
            panic!("Vertex count must at least be 2 to generate a path graph")
        }
        PathGraphGenerator { vertex_count }
    }

    pub fn add_component_to<S>(storage: &mut S, vertex_count: usize) -> DynIter<'_, usize>
    where
        S: Storage<Dir = Undirected>
            + InitializableStorage
            + Vertices
            + Edges
            + MutVertices
            + MutEdges,
        Standard: Distribution<S::V>,
        Standard: Distribution<S::E>,
    {
        // TODO: what will be the minimum vertex count
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        for vts in vertex_tokens.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        DynIter::init(vertex_tokens.into_iter())
    }
}

impl<S> Generator<S, Undirected> for PathGraphGenerator
where
    S: Storage<Dir = Undirected> + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();

        PathGraphGenerator::add_component_to(&mut storage, self.vertex_count);

        storage
    }
}

#[cfg(test)]
mod test {
    use super::PathGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for PathGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for PathGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 2;

            PathGraphGenerator::init(vertex_count)
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

    use super::PathGraphGenerator;

    #[quickcheck]
    fn prop_gen_path_graph(generator: PathGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 1)
                .count(),
            2
        );

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 2)
                .count(),
            graph.vertex_count() - 2
        );
    }
}
