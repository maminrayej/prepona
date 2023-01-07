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

use crate::provide::*;

pub trait Generate<S>
where
    S: EmptyStorage + AddNode + AddEdge,
{
    fn generate(&self) -> S {
        let mut storage = S::init();

        self.generate_into(&mut storage, NodeID(0));

        storage
    }

    fn generate_into(&self, storage: &mut S, start: NodeID) -> NodeID;
}
