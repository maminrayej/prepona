mod empty_graph;
pub use empty_graph::EmptyGraph;

mod path_graph;
pub use path_graph::PathGraph;

mod cycle_graph;
pub use cycle_graph::CycleGraph;

mod complete_graph;
pub use complete_graph::CompleteGraph;

mod star_graph;
pub use star_graph::StarGraph;

mod wheel_graph;
pub use wheel_graph::WheelGraph;

use crate::give::*;

pub trait Make<S>
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn make(&self) -> S {
        let mut storage = S::init();

        self.append(&mut storage, NodeID(0));

        storage
    }

    fn append(&self, storage: &mut S, start: NodeID) -> NodeID;
}
