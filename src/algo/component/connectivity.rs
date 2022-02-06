use anyhow::Result;

use crate::algo::errors::AlgoError;
use crate::algo::traversal::{ControlFlow, Event, DFS};
use crate::common::RealID;
use crate::provide::{Storage, Vertices};
use crate::storage::edge::Undirected;

pub fn connected_components<G>(graph: &G) -> Vec<Vec<RealID>>
where
    G: Storage<Dir = Undirected> + Vertices,
{
    let mut ccs = vec![];

    let mut current = vec![];

    DFS::init(graph).execute(|event| {
        match event {
            Event::Discover(_, vid) => current.push(vid),
            Event::End(_) => {
                ccs.push(current.clone());
                current.clear();
            }
            _ => {}
        }

        ControlFlow::Continue
    });

    ccs
}

pub fn number_connected_components<G>(graph: &G) -> usize
where
    G: Storage<Dir = Undirected> + Vertices,
{
    connected_components(graph).len()
}

pub fn is_connected<G>(graph: &G) -> Result<bool>
where
    G: Storage<Dir = Undirected> + Vertices,
{
    if graph.vertex_count() == 0 {
        Err(
            AlgoError::UndefinedConcept("Connectivity is undefined for null graph".to_string())
                .into(),
        )
    } else {
        Ok(number_connected_components(graph) == 1)
    }
}

#[cfg(test)]
mod tests {

    use quickcheck_macros::quickcheck;

    use crate::algo::component::{connected_components, is_connected, number_connected_components};
    use crate::gen::{CompleteGraphGenerator, Generator, NullGraphGenerator};
    use crate::storage::edge::Undirected;
    use crate::storage::AdjMap;

    #[quickcheck]
    fn prop_connected_components(generator: NullGraphGenerator) {
        let mut graph: AdjMap<(), (), Undirected> = generator.generate();

        CompleteGraphGenerator::add_component_to(&mut graph, 3);
        CompleteGraphGenerator::add_component_to(&mut graph, 4);
        CompleteGraphGenerator::add_component_to(&mut graph, 5);
        CompleteGraphGenerator::add_component_to(&mut graph, 6);

        let ccs = connected_components(&graph);
        let ccs_count = number_connected_components(&graph);
        let is_connected = is_connected(&graph).unwrap();

        assert_eq!(ccs.len(), 4);
        assert_eq!(ccs.into_iter().map(|cc| cc.len()).sum::<usize>(), 18);
        assert_eq!(ccs_count, 4);
        assert!(!is_connected);
    }
}
