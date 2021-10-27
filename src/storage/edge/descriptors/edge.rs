use super::{CheckedFixedSizeMutEdgeDescriptor, EdgeDescriptor, FixedSizeMutEdgeDescriptor};
use crate::storage::{edge::Direction, vertex::VertexToken};

/// A directed [`Edge`].
pub type DirectedEdge<VT> = Edge<VT, true>;

/// An undirected [`Edge`].
pub type UndirectedEdge<VT> = Edge<VT, false>;

/// Most common type of edge that connects two vertices.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
#[derive(PartialEq, Eq)]
pub struct Edge<VT, const DIR: bool>
where
    VT: VertexToken,
{
    src_vt: VT,
    dst_vt: VT,
}

impl<VT, const DIR: bool> Edge<VT, DIR>
where
    VT: VertexToken,
{
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

impl<VT, const DIR: bool> Direction<DIR> for Edge<VT, DIR> where VT: VertexToken {}

impl<VT, const DIR: bool> EdgeDescriptor<VT, DIR> for Edge<VT, DIR>
where
    VT: VertexToken,
{
    /// # Complexity
    /// O(1)
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.src_vt))
        } else {
            Box::new(std::iter::once(&self.src_vt).chain(Some(&self.dst_vt)))
        }
    }

    /// # Complexity
    /// O(1)
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_directed() {
            Box::new(std::iter::once(&self.dst_vt))
        } else {
            Box::new(std::iter::once(&self.dst_vt).chain(Some(&self.src_vt)))
        }
    }

    /// # Complexity
    /// O(1)
    fn is_source(&self, vt: &VT) -> bool {
        &self.src_vt == vt || (Self::is_undirected() && &self.dst_vt == vt)
    }

    /// # Complexity
    /// O(1)
    fn is_destination(&self, vt: &VT) -> bool {
        &self.dst_vt == vt || (Self::is_undirected() && &self.src_vt == vt)
    }

    /// # Complexity
    /// O(1)
    fn contains(&self, vt: &VT) -> bool {
        &self.src_vt == vt || &self.dst_vt == vt
    }

    /// # Complexity
    /// O(1)
    fn sources_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }

    /// # Complexity
    /// O(1)
    fn destinations_count(&self) -> usize {
        if Self::is_directed() {
            1
        } else {
            2
        }
    }
}

impl<VT, const DIR: bool> FixedSizeMutEdgeDescriptor<VT, DIR> for Edge<VT, DIR>
where
    VT: VertexToken,
{
    /// # Complexity
    /// O(1)
    fn replace_src(&mut self, target_vt: &VT, vt: VT) {
        if &self.src_vt == target_vt {
            self.src_vt = vt;
        } else if Self::is_undirected() {
            self.dst_vt = vt;
        }
    }

    /// # Complexity
    /// O(1)
    fn replace_dst(&mut self, target_vt: &VT, vt: VT) {
        if &self.dst_vt == target_vt {
            self.dst_vt = vt;
        } else if Self::is_undirected() {
            self.src_vt = vt;
        }
    }
}

impl<VT, const DIR: bool> CheckedFixedSizeMutEdgeDescriptor<VT, DIR> for Edge<VT, DIR> where
    VT: VertexToken
{
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn assert_undirected_edge_description<VT: VertexToken>(
        edge: &UndirectedEdge<VT>,
        src_vt: &VT,
        dst_vt: &VT,
    ) {
        // It must return both 0 and 1 as sources.
        assert_eq!(
            HashSet::<_>::from_iter([src_vt, dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return both 0 and 1 as destinations.
        assert_eq!(
            HashSet::<_>::from_iter([src_vt, dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is both a source and a destination.
        assert!(edge.is_source(src_vt));
        assert!(edge.is_destination(src_vt));

        // 1 is both a source and destination.
        assert!(edge.is_source(dst_vt));
        assert!(edge.is_destination(dst_vt));

        // It must contain both 0 and 1
        assert!(edge.contains(src_vt));
        assert!(edge.contains(dst_vt));

        // It must contain 2 sources and 2 destinations.
        assert_eq!(edge.sources_count(), 2);
        assert_eq!(edge.destinations_count(), 2);
    }

    fn assert_directed_edge_description<VT: VertexToken>(
        edge: &DirectedEdge<VT>,
        src_vt: &VT,
        dst_vt: &VT,
    ) {
        // It must return only 0 as its source.
        assert_eq!(
            HashSet::<_>::from_iter([src_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        // It must return only 1 as its destination.
        assert_eq!(
            HashSet::<_>::from_iter([dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        // 0 is only a source.
        assert!(edge.is_source(src_vt));
        assert!(!edge.is_destination(src_vt));

        // 1 is only a destination.
        assert!(!edge.is_source(dst_vt));
        assert!(edge.is_destination(dst_vt));

        // It must contain both 0 and 1.
        assert!(edge.contains(src_vt));
        assert!(edge.contains(dst_vt));

        // It contain only one source and one destination.
        assert_eq!(edge.sources_count(), 1);
        assert_eq!(edge.destinations_count(), 1);
    }

    #[test]
    fn undirected_edge_descriptor() {
        assert_undirected_edge_description(&UndirectedEdge::init(0, 1), &0, &1);
    }

    #[test]
    fn directed_edge_descriptor() {
        assert_directed_edge_description(&DirectedEdge::init(0, 1), &0, &1);
    }

    #[test]
    fn undirected_edge_fixed_size_descriptor_replace_src() {
        let mut edge = UndirectedEdge::init(0, 1);

        edge.replace_src(&0, 2);
        edge.replace_src(&1, 3);

        assert_undirected_edge_description(&edge, &2, &3);
    }

    #[test]
    fn undirected_edge_fixed_size_descriptor_replace_dst() {
        let mut edge = UndirectedEdge::init(0, 1);

        edge.replace_dst(&0, 2);
        edge.replace_dst(&1, 3);

        assert_undirected_edge_description(&edge, &2, &3);
    }

    #[test]
    fn directed_edge_fixed_size_descriptor_replace_src_dst() {
        let mut edge = DirectedEdge::init(0, 1);

        edge.replace_src(&0, 2);
        edge.replace_dst(&1, 3);

        assert_directed_edge_description(&edge, &2, &3);
    }

    // TODO: Implement tests to check `Edge` actually conforms to the definitions and default implementation of CheckedFixedSizeMutEdgeDescriptor behave correctly for it.
}
