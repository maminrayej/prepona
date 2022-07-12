use crate::algo::traversal::dfs::{ControlFlow, Event, DFS};
use crate::provide::{NodeId, NodeIdMapProvider, NodeProvider, Undirected};

pub struct ConnectedComponents<'a, G> {
    graph: &'a G,
}

impl<'a, G> ConnectedComponents<'a, G>
where
    G: NodeProvider<Dir = Undirected> + NodeIdMapProvider,
{
    pub fn new(graph: &'a G) -> Self {
        Self { graph }
    }

    pub fn connected_components(&self) -> Vec<Vec<NodeId>> {
        let mut ccs = vec![];

        let mut current = vec![];

        DFS::init(self.graph).execute(|event| {
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

    pub fn number_connected_components(&self) -> usize {
        self.connected_components().len()
    }

    pub fn is_connected(&self) -> bool {
        if self.graph.node_count() == 0 {
            true
        } else {
            self.number_connected_components() == 1
        }
    }
}
