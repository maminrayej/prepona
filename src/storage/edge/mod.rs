//! Contains structures and traits necessary to describe edges and anything related to them.

mod descriptor;
mod direction;

pub use descriptor::*;
pub use direction::*;

#[derive(Debug)]
pub(crate) struct Edge<E: EdgeDescriptor> {
    inner: E,
    src_vt: usize,
    dst_vt: usize,
}

impl<E: EdgeDescriptor> Edge<E> {
    pub fn init(src_vt: usize, dst_vt: usize, inner: E) -> Self {
        Edge {
            src_vt,
            dst_vt,
            inner,
        }
    }

    pub fn view(&self) -> (usize, usize, &E) {
        (self.src_vt, self.dst_vt, &self.inner)
    }

    pub fn view_mut(&mut self) -> (usize, usize, &mut E) {
        (self.src_vt, self.dst_vt, &mut self.inner)
    }

    pub fn into_inner(self) -> E {
        self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::storage::edge::EdgeDescriptor;

    use super::Edge;

    impl<E: EdgeDescriptor + Clone> Clone for Edge<E> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
                src_vt: self.src_vt.clone(),
                dst_vt: self.dst_vt.clone(),
            }
        }
    }
}
