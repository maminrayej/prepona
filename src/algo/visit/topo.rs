use std::ops::ControlFlow;

use crate::error::{Error, Result};
use crate::provide::{EdgeRef, NodeId};

use super::{Continue, Dfs, DfsEvent, EdgeType, VisitFlow};

pub struct Topo<'a, S> {
    storage: &'a S,
}

impl<'a, S> Topo<'a, S>
where
    S: EdgeRef,
{
    pub fn new(storage: &'a S) -> Self {
        Self { storage }
    }

    pub fn exec<F>(&self, mut f: F) -> Result<()>
    where
        F: FnMut((NodeId, &'a S::Node)) -> VisitFlow,
    {
        let dfs = Dfs::new(self.storage, None.into_iter());

        let mut result = Ok(());
        dfs.exec(|e| {
            let flow = if let DfsEvent::Discover(nid, node) = e {
                f((nid, node))
            } else if let DfsEvent::Edge {
                ety: EdgeType::Back,
                ..
            } = e
            {
                result = Err(Error::NotDAG);

                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(Continue::Noop)
            };

            flow
        });

        result
    }
}
