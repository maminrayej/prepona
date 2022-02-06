use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::common::DynIter;
use crate::provide::{Edges, InitializableStorage, MutEdges, MutVertices, Storage, Vertices};

use crate::gen::Generator;
use crate::storage::edge::Direction;

#[derive(Debug)]
pub struct CompleteGraphGenerator {
    vertex_count: usize,
}

impl CompleteGraphGenerator {
    pub fn init(vertex_count: usize) -> Self {
        CompleteGraphGenerator { vertex_count }
    }

    pub fn add_component_to<S>(storage: &mut S, vertex_count: usize) -> DynIter<'_, usize>
    where
        S: Storage + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
        Standard: Distribution<S::V>,
        Standard: Distribution<S::E>,
    {
        let mut rng = thread_rng();

        let vertex_tokens: Vec<usize> = (0..vertex_count)
            .into_iter()
            .map(|_| storage.add_vertex(rng.gen()))
            .collect();

        let vt_pairs = vertex_tokens
            .iter()
            .copied()
            .cartesian_product(vertex_tokens.iter().copied());

        for (src_vt, dst_vt) in vt_pairs {
            if (S::Dir::is_directed() && src_vt != dst_vt)
                || (S::Dir::is_undirected() && src_vt < dst_vt)
            {
                storage.add_edge(src_vt, dst_vt, rng.gen());
            }
        }

        DynIter::init(vertex_tokens.into_iter())
    }
}

impl<S, Dir> Generator<S, Dir> for CompleteGraphGenerator
where
    S: Storage<Dir = Dir> + InitializableStorage + Vertices + Edges + MutVertices + MutEdges,
    Dir: Direction,
    Standard: Distribution<S::V>,
    Standard: Distribution<S::E>,
{
    fn generate(&self) -> S {
        let mut storage = S::init();

        CompleteGraphGenerator::add_component_to(&mut storage, self.vertex_count);

        storage
    }
}

#[cfg(test)]
mod test {
    use super::CompleteGraphGenerator;
    use quickcheck::Arbitrary;

    impl Clone for CompleteGraphGenerator {
        fn clone(&self) -> Self {
            Self {
                vertex_count: self.vertex_count.clone(),
            }
        }
    }

    impl Arbitrary for CompleteGraphGenerator {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let vertex_count = usize::arbitrary(g) % 16 + 1;

            CompleteGraphGenerator::init(vertex_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use quickcheck_macros::quickcheck;

    use crate::{
        gen::Generator,
        provide::{Edges, Vertices},
        storage::{edge::Undirected, AdjMap},
    };

    use super::CompleteGraphGenerator;

    #[quickcheck]
    fn prop_gen_complete_graph(generator: CompleteGraphGenerator) {
        let graph: AdjMap<(), (), Undirected> = generator.generate();

        if graph.vertex_count() > 0 {
            assert_eq!(
                graph.edge_count(),
                graph.vertex_count() * (graph.vertex_count() - 1) / 2
            );
        }

        let vts: Vec<usize> = graph.vertex_tokens().collect();
        let vt_pairs = vts.iter().copied().cartesian_product(vts.iter().copied());

        for (src_vt, dst_vt) in vt_pairs {
            assert_eq!(
                graph
                    .outgoing_edges(src_vt)
                    .map(|et| graph.edge(et))
                    .filter(|(s_vt, d_vt, _)| (*s_vt == src_vt && *d_vt == dst_vt)
                        || (*d_vt == src_vt && *s_vt == dst_vt))
                    .count(),
                if src_vt != dst_vt { 1 } else { 0 }
            )
        }
    }
}
