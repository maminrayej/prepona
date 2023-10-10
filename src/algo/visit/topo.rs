use std::ops::ControlFlow;

use crate::error::{Error, Result};
use crate::provide::EdgeRef;

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
        F: FnMut(&'a S::Node) -> VisitFlow,
    {
        let mut result = Ok(());

        let dfs = Dfs::new(self.storage, None.into_iter());

        #[rustfmt::skip]
        dfs.exec(|e| {
            let flow = if let DfsEvent::Discover(node) = e {
                f(node)
            } else if let DfsEvent::Edge { ety: EdgeType::Back, .. } = e {
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
