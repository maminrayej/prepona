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
#[derive(Debug, PartialEq, Eq)]
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
        if Self::is_undirected() && self.src_vt != self.dst_vt {
            Box::new(std::iter::once(&self.src_vt).chain(Some(&self.dst_vt)))
        } else {
            Box::new(std::iter::once(&self.src_vt))
        }
    }

    /// # Complexity
    /// O(1)
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_> {
        if Self::is_undirected() && self.src_vt != self.dst_vt {
            Box::new(std::iter::once(&self.dst_vt).chain(Some(&self.src_vt)))
        } else {
            Box::new(std::iter::once(&self.dst_vt))
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
        self.get_sources().count()
    }

    /// # Complexity
    /// O(1)
    fn destinations_count(&self) -> usize {
        self.get_destinations().count()
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
pub mod test {
    use super::*;
    use quickcheck::Arbitrary;

    impl<VT: VertexToken + Clone, const DIR: bool> Clone for Edge<VT, DIR> {
        fn clone(&self) -> Self {
            Self {
                src_vt: self.src_vt.clone(),
                dst_vt: self.dst_vt.clone(),
            }
        }
    }

    impl<VT: VertexToken + Arbitrary, const DIR: bool> Arbitrary for Edge<VT, DIR> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let src_vt = VT::arbitrary(g);
            let dst_vt = VT::arbitrary(g);

            Edge { src_vt, dst_vt }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::edge::test_utils;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn prop_edge_description(edge: UndirectedEdge<usize>, dir_edge: DirectedEdge<usize>) {
        test_utils::prop_edge_description(edge);
        test_utils::prop_edge_description(dir_edge);
    }

    #[quickcheck]
    fn prop_undirected_fixed_size_descriptor_replace_src(edge: UndirectedEdge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_undirected_fixed_size_descriptor_replace_dst(edge: UndirectedEdge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_undirected_checked_fixed_size_descriptor_replace_src(edge: UndirectedEdge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_undirected_checked_fixed_size_descriptor_replace_dst(edge: UndirectedEdge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_directed_fixed_size_descriptor_replace_src(edge: DirectedEdge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_directed_fixed_size_descriptor_replace_dst(edge: DirectedEdge<usize>) {
        test_utils::prop_fixed_size_descriptor_replace_dst(edge);
    }

    #[quickcheck]
    fn prop_directed_checked_fixed_size_descriptor_replace_src(edge: DirectedEdge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_src(edge);
    }

    #[quickcheck]
    fn prop_directed_checked_fixed_size_descriptor_replace_dst(edge: DirectedEdge<usize>) {
        test_utils::prop_checked_fixed_size_descriptor_replace_dst(edge);
    }
}
