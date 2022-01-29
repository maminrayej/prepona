use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{Edges, MutEdges, MutVertices, Storage, Vertices, InitializableStorage};

use crate::gen::Generator;
use crate::storage::edge::Undirected;

#[derive(Debug)]
pub struct StarGraphGenerator {
    vertex_count: usize,
}

impl StarGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count < 3 {
            panic!("Vertex count must be atleast 3 to form a star graph")
        }

        StarGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S, Undirected> for StarGraphGenerator
where
    S: Storage<Dir = Undirected> + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..self.vertex_count - 1)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let center_vt = storage.add_vertex(rng.gen());

        for vt in vertex_tokens {
            storage.add_edge(center_vt, vt, rng.gen());
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::StarGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for StarGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for StarGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 3;

            StarGraphGenerator::init(vertex_count)
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

    use super::StarGraphGenerator;

    #[quickcheck]
    fn prop_gen_star_graph(generator: StarGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == 1)
                .count(),
            graph.vertex_count() - 1
        );

        assert_eq!(
            graph
                .vertex_tokens()
                .map(|vt| graph.outgoing_edges(vt).count())
                .filter(|out_degree| *out_degree == graph.vertex_count() - 1)
                .count(),
            1
        );
    }
}
