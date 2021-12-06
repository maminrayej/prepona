use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::provide::{InitializableStorage, MutStorage};

use crate::gen::Generator;

#[derive(Debug)]
pub struct LadderGraphGenerator {
    vertex_count: usize,
}

impl LadderGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        if vertex_count % 2 != 0 && vertex_count >= 2 {
            panic!("Number of vertices must be at least 2 and an even number to generate a ladder graph")
        }

        LadderGraphGenerator { vertex_count }
    }
}

impl<S> Generator<S> for LadderGraphGenerator
where
    S: InitializableStorage + MutStorage,
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

        let mut index = 0;
        let (odd, even): (Vec<usize>, Vec<usize>) = vertex_tokens.into_iter().partition(|_| {
            index += 1;
            index % 2 == 0
        });

        for vts in odd.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        for vts in even.windows(2) {
            storage.add_edge(vts[0], vts[1], rng.gen());
        }

        for (src_vt, dst_vt) in odd.iter().zip(even.iter()) {
            storage.add_edge(*src_vt, *dst_vt, rng.gen());
        }

        storage
    }
}

#[cfg(test)]
mod test {
    use super::LadderGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for LadderGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for LadderGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = (usize::arbitrary(g) % 16 + 1) * 2;

            LadderGraphGenerator::init(vertex_count)
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

    use super::LadderGraphGenerator;

    #[quickcheck]
    fn prop_gen_ladder_graph(generator: LadderGraphGenerator) {
        let graph: AdjMap<(), (), false> = generator.generate();

        if graph.vertex_count() == 2 {
            assert_eq!(graph.edge_count(), 1);
            assert_eq!(
                graph
                    .edges()
                    .filter(|(src_vt, dst_vt, _)| src_vt == dst_vt)
                    .count(),
                0
            )
        } else {
            assert_eq!(
                graph
                    .vertex_tokens()
                    .map(|vt| graph.outgoing_edges(vt).count())
                    .filter(|out_degree| *out_degree == 2)
                    .count(),
                4
            );

            assert_eq!(
                graph
                    .vertex_tokens()
                    .map(|vt| graph.outgoing_edges(vt).count())
                    .filter(|out_degree| *out_degree == 3)
                    .count(),
                graph.vertex_count() - 4
            );
        }
    }
}
