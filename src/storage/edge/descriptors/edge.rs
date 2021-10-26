use super::{EdgeDescriptor, FixedSizeMutEdgeDescriptor};
use crate::storage::{edge::Direction, vertex::VertexToken};

/// A directed [`Edge`].
pub type DirectedEdge<VT> = Edge<VT, true>;

/// An undirected [`Edge`].
pub type UndirectedEdge<VT> = Edge<VT, false>;

/// Most common type of edge that connects two vertices.
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the source and destination of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
#[derive(PartialEq, Eq)]
pub struct Edge<VT: VertexToken, const DIR: bool> {
    src_vt: VT,
    dst_vt: VT,
}

impl<VT: VertexToken, const DIR: bool> Edge<VT, DIR> {
    /// # Arguments
    /// * `src_vt`: Token of the source vertex.
    /// * `dst_vt`: Token of the destination vertex.
    ///
    /// # Returns
    /// An `Edge` that holds the connection between `src_vt` and `dst_vt`.
    pub fn init(src_vt: VT, dst_vt: VT) -> Self {
        Edge { src_vt, dst_vt }
    }
}

impl<VT: VertexToken, const DIR: bool> Direction<DIR> for Edge<VT, DIR> {}

impl<VT: VertexToken, const DIR: bool> EdgeDescriptor<VT, DIR> for Edge<VT, DIR> {
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.src_vt))
        } else {
            Box::new(std::iter::once(&self.src_vt).chain(Some(&self.dst_vt)))
        }
    }

    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.dst_vt))
        } else {
            Box::new(std::iter::once(&self.dst_vt).chain(Some(&self.src_vt)))
        }
    }

    fn is_source(&self, vt: &VT) -> bool {
        &self.src_vt == vt || (Self::is_undirected() && &self.dst_vt == vt)
    }

    fn is_destination(&self, vt: &VT) -> bool {
        &self.dst_vt == vt || (Self::is_undirected() && &self.src_vt == vt)
    }

    fn contains(&self, vt: &VT) -> bool {
        &self.src_vt == vt || &self.dst_vt == vt
    }

    fn sources_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }

    fn destinations_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }
}

impl<VT: VertexToken, const DIR: bool> FixedSizeMutEdgeDescriptor<VT, DIR> for Edge<VT, DIR> {
    fn replace_src(&mut self, _: &VT, vt: VT) {
        self.src_vt = vt
    }

    fn replace_dst(&mut self, _: &VT, vt: VT) {
        self.dst_vt = vt
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn storage_descriptor_edge_undirected() {
        // Given an undirected edge.
        let edge = UndirectedEdge::init(0, 1);

        // It must return both 0 and 1 as sources.
        assert_eq!(
            HashSet::<_>::from_iter([0, 1].iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return both 0 and 1 as destinations.
        assert_eq!(
            HashSet::<_>::from_iter([0, 1].iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is both a source and a destination.
        assert!(edge.is_source(&0));
        assert!(edge.is_destination(&0));

        // 1 is both a source and destination.
        assert!(edge.is_source(&1));
        assert!(edge.is_destination(&1));

        // It must contain both 0 and 1
        assert!(edge.contains(&0));
        assert!(edge.contains(&1));

        // It must contain 2 sources and 2 destinations.
        assert_eq!(edge.sources_count(), 2);
        assert_eq!(edge.destinations_count(), 2);
    }

    #[test]
    fn storage_descriptor_edge_directed() {
        // Given a directed edge.
        let edge = DirectedEdge::init(0, 1);

        // It must return only 0 as its source.
        assert_eq!(
            HashSet::<_>::from_iter([0].iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return only 1 as its destination.
        assert_eq!(
            HashSet::<_>::from_iter([1].iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is only a source.
        assert!(edge.is_source(&0));
        assert!(!edge.is_destination(&0));

        // 1 is only a destination.
        assert!(!edge.is_source(&1));
        assert!(edge.is_destination(&1));

        // It must contain both 0 and 1.
        assert!(edge.contains(&0));
        assert!(edge.contains(&1));

        // It contain only one source and one destination.
        assert_eq!(edge.sources_count(), 1);
        assert_eq!(edge.destinations_count(), 1);
    }
}
