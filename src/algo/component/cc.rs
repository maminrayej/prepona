use crate::algo::visit::{Continue, ControlFlow, Dfs, DfsEvent};
use crate::provide::*;

pub struct ConnectedComponents<'a, S> {
    storage: &'a S,
}

impl<'a, S> ConnectedComponents<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        ConnectedComponents { storage }
    }

    pub fn exec(&self) {
        let mut ccs = vec![];

        let mut current = vec![];

        Dfs::init(self.storage).execute(|event| {
            match event {
                DfsEvent::Discover(node) => current.push(node),
                DfsEvent::End(_) => {
                    ccs.push(current.clone());
                    current.clear();
                }
                _ => {}
            }

            ControlFlow::Continue(Continue::Noop)
        });

        // TODO: Return the view of each connecte component
    }
}
