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
mod test {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    use rand::Rng;
    use std::collections::HashSet;

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

    fn assert_undirected_edge_description<VT: VertexToken>(
        edge: &UndirectedEdge<VT>,
        src_vt: &VT,
        dst_vt: &VT,
    ) {
        assert_eq!(
            HashSet::<_>::from_iter([src_vt, dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        assert_eq!(
            HashSet::<_>::from_iter([src_vt, dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        assert!(edge.is_source(src_vt));
        assert!(edge.is_destination(src_vt));

        assert!(edge.is_source(dst_vt));
        assert!(edge.is_destination(dst_vt));

        assert!(edge.contains(src_vt));
        assert!(edge.contains(dst_vt));

        assert_eq!(edge.sources_count(), 2);
        assert_eq!(edge.destinations_count(), 2);
    }

    fn assert_directed_edge_description<VT: VertexToken>(
        edge: &DirectedEdge<VT>,
        src_vt: &VT,
        dst_vt: &VT,
    ) {
        assert_eq!(
            HashSet::<_>::from_iter([src_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_sources())
        );

        assert_eq!(
            HashSet::<_>::from_iter([dst_vt].into_iter()),
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        assert!(edge.is_source(src_vt));

        assert!(edge.is_destination(dst_vt));

        assert!(edge.contains(src_vt));
        assert!(edge.contains(dst_vt));

        assert_eq!(edge.sources_count(), 1);
        assert_eq!(edge.destinations_count(), 1);
    }

    #[quickcheck]
    fn prop_undirected_edge_descriptor(edge: UndirectedEdge<usize>) {
        assert_undirected_edge_description(&edge, &edge.src_vt, &edge.dst_vt);
    }

    #[quickcheck]
    fn prop_directed_edge_descriptor(edge: DirectedEdge<usize>) {
        assert_directed_edge_description(&edge, &edge.src_vt, &edge.dst_vt)
    }

    #[quickcheck]
    fn prop_undirected_fixed_size_descriptor_replace_src(mut edge: UndirectedEdge<usize>) {
        let vts: Vec<usize> = edge.get_sources().copied().collect();
        let src_vt = vts[0];
        let dst_vt = vts[1];

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        edge.replace_src(&src_vt, new_src_vt);
        edge.replace_src(&dst_vt, new_dst_vt);

        assert_undirected_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    #[quickcheck]
    fn prop_undirected_fixed_size_descriptor_replace_dst(mut edge: UndirectedEdge<usize>) {
        let vts: Vec<usize> = edge.get_sources().copied().collect();
        let src_vt = vts[0];
        let dst_vt = vts[1];

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        edge.replace_dst(&src_vt, new_src_vt);
        edge.replace_dst(&dst_vt, new_dst_vt);

        assert_undirected_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    #[quickcheck]
    fn prop_directed_fixed_size_descriptor_replace_src_dst(mut edge: DirectedEdge<usize>) {
        let src_vt = edge.get_sources().next().copied().unwrap();
        let dst_vt = edge.get_destinations().next().copied().unwrap();

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        edge.replace_src(&src_vt, new_src_vt);
        edge.replace_dst(&dst_vt, new_dst_vt);

        assert_directed_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    #[quickcheck]
    fn prop_undirected_checked_fixed_size_descriptor_replace_src(mut edge: UndirectedEdge<usize>) {
        let vts: Vec<usize> = edge.get_sources().copied().collect();
        let src_vt = vts[0];
        let dst_vt = vts[1];

        let invalid_vt = get_non_duplicate([src_vt, dst_vt], 1)[0];

        assert!(edge.replace_src_checked(&invalid_vt, invalid_vt).is_err());

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        assert!(edge.replace_src_checked(&src_vt, new_src_vt).is_ok());
        assert!(edge.replace_src_checked(&dst_vt, new_dst_vt).is_ok());

        assert_undirected_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    #[quickcheck]
    fn prop_undirected_checked_fixed_size_descriptor_replace_dst(mut edge: UndirectedEdge<usize>) {
        let vts: Vec<usize> = edge.get_sources().copied().collect();
        let src_vt = vts[0];
        let dst_vt = vts[1];

        let invalid_vt = get_non_duplicate([src_vt, dst_vt], 1)[0];

        assert!(edge.replace_dst_checked(&invalid_vt, invalid_vt).is_err());

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        assert!(edge.replace_dst_checked(&src_vt, new_src_vt).is_ok());
        assert!(edge.replace_dst_checked(&dst_vt, new_dst_vt).is_ok());

        assert_undirected_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    #[quickcheck]
    fn prop_directed_checked_fixed_size_descriptor_replace_src_dst(mut edge: DirectedEdge<usize>) {
        let src_vt = edge.get_sources().next().copied().unwrap();
        let dst_vt = edge.get_destinations().next().copied().unwrap();

        let new_vts = get_non_duplicate([src_vt, dst_vt], 2);
        let new_src_vt = new_vts[0];
        let new_dst_vt = new_vts[1];

        if !edge.is_source(&dst_vt) {
            assert!(edge.replace_src_checked(&dst_vt, new_src_vt).is_err());
        }

        if !edge.is_destination(&src_vt) {
            assert!(edge.replace_dst_checked(&src_vt, new_dst_vt).is_err());
        }

        assert!(edge.replace_src_checked(&src_vt, new_src_vt).is_ok());
        assert!(edge.replace_dst_checked(&dst_vt, new_dst_vt).is_ok());

        assert_directed_edge_description(&edge, &new_src_vt, &new_dst_vt);
    }

    fn get_non_duplicate(set_iter: impl IntoIterator<Item = usize>, count: usize) -> Vec<usize> {
        let mut set = HashSet::<_>::from_iter(set_iter);

        let mut rng = rand::thread_rng();

        let mut values = vec![0; count];

        for index in 0..count {
            let mut value: usize = rng.gen();
            while set.contains(&value) {
                value = rng.gen();
            }
            values[index] = value;
            set.insert(value);
        }

        values
    }
}
