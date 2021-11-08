mod edge;
mod hyperedges;

use super::Direction;
use crate::storage::vertex::VertexToken;
use crate::storage::StorageError;
use anyhow::Result;

pub use edge::*;
pub use hyperedges::*;

/// Describes what is expected from an edge.
///
/// Each struct that wants to be integrated into the library as an edge, must at least implement this trait.
///
/// `EdgeDescriptor` must be generic and flexible enough so that any kind of edge can be modeled with it.
/// Therefore, in the most general sense it is considered that each edge can connect multiple sources to multiple destinations at once[^note].
/// It is a given that connecting one source to one destination is included in this definition.
///
/// Note that `EdgeDescriptor` provides default implementation for some of the methods. Time complexity of these methods are specified.
/// Implementors of this trait must provide info about complexity if they're not using the default implementation.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * `PartialEq`, `Eq`: Each edge must be comparable with another edge that is of the same type.
/// * [`Direction`]: Edge must be either directed or undirected.
///
/// [^note]: In even more [general] sense an edge can connect to other edges. But, they're not accounted for in our definitions.
///
/// [general]: https://en.wikipedia.org/wiki/Hypergraph#Further_generalizations
pub trait EdgeDescriptor<VT: VertexToken, const DIR: bool>:
    PartialEq + Eq + Direction<DIR>
{
    // TODO: Add post condition that tokens can't repeat themselves.

    /// # Returns
    /// If edge is:
    /// * Directed: An iterator over tokens of source vertices.
    /// * Undirected: An iterator over tokens of all vertices.
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    /// # Returns
    /// If edge is:
    /// * Directed: An iterator over tokens of destination vertices.
    /// * Undirected: An iterator over tokens of all vertices.
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a source or not.
    ///
    /// # Returns
    /// If edge is:
    /// * Directed:
    ///     * `true`: If `vt` is the token of one of the source vertices.
    ///     * `false`: Otherwise.
    /// * Undirected:
    ///     * `true`: If `vt` is the token of a source or destination vertex.
    ///     * `false`: Otherwise.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::get_sources`] + |V<sub>src</sub>|)
    fn is_source(&self, vt: &VT) -> bool {
        self.get_sources().any(|src_vt| src_vt == vt)
    }

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a destination or not.
    ///
    /// # Returns
    /// If edge is:
    /// * Directed:
    ///     * `true`: If `vt` is the token of one of the destination vertices.
    ///     * `false`: Otherwise.
    /// * Undirected:
    ///     * `true`: If `vt` is the token of a source or destination vertex.
    ///     * `false`: Otherwise.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::get_destinations`] + |V<sub>dst</sub>|)
    fn is_destination(&self, vt: &VT) -> bool {
        self.get_destinations().any(|dst_vt| dst_vt == vt)
    }

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a source or a destination.
    ///
    /// # Returns
    /// * `true`: If one of the sources or destinations has a token equal to `vt`.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`EdgeDescriptor::is_destination`])
    fn contains(&self, vt: &VT) -> bool {
        self.is_source(vt) || self.is_destination(vt)
    }

    /// # Returns
    /// If edge is:
    /// * Directed: Number of source vertices.
    /// * Undirected: Number of total vertices.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::get_sources`] + |V<sub>src</sub>|)
    fn sources_count(&self) -> usize {
        self.get_sources().count()
    }

    /// # Returns
    /// If edge is:
    /// * Directed: Number of destination vertices.
    /// * Undirected: Number of total vertices.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::get_destinations`] + |V<sub>dst</sub>|)
    fn destinations_count(&self) -> usize {
        self.get_destinations().count()
    }
}

/// This trait builds upon [`EdgeDescriptor`] and adds limited mutability to the edge.
///
/// This trait allows mutability as long as the number of sources and destinations of the edge remain the same.
/// Therefore it only allows replacing vertex tokens.
/// If you want adding and removal of vertex tokens, checkout [`MutEdgeDescriptor`].
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// [`EdgeDescriptor`]: Every edge must implement this trait first.
pub trait FixedSizeMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    EdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `target_vt`: Token of the target vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `target_vt`.
    ///
    /// # Preconditions
    /// If edge is:
    /// * Directed: `target_vt` must be the token of a source vertex. `vt` must not exist as a source.
    /// * Undirected: `target_vt` can be the token of a source or a destination vertex. `vt` must not exist as a source.
    ///
    /// # Postconditions
    /// `target_vt` is replaced with `vt`.
    fn replace_src(&mut self, target_vt: &VT, vt: VT);

    /// # Arguments
    /// * `target_vt`: Token of the target vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replaced `target_vt`.
    ///
    /// # Preconditions
    /// If edge is:
    /// * Directed: `target_vt` must be the token of a destination vertex. `vt` must not exist as destination.
    /// * Undirected: `target_vt` can be the token of a source or a destination vertex. `vt` must not exist as destination.
    ///
    /// # Postconditions
    /// `target_vt` is replaced with `vt`.
    fn replace_dst(&mut self, target_vt: &VT, vt: VT);
}

/// Checked version of [`FixedSizeMutEdgeDescriptor`] trait.
///
/// Note that `CheckedFixedSizeMutEdgeDescriptor` provides default implementation and info about complexity for all of its methods.
/// These default implementations take into account all preconditions listed for [`FixedSizeMutEdgeDescriptor`] methods.
/// But if you want to override these basic implementations, make sure to:
/// 1. Take into account all the preconditions of the methods in [`FixedSizeMutEdgeDescriptor`] before calling them.
/// 2. Provide complexity info for your implementations.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * [`FixedSizeMutEdgeDescriptor`]: Checked version of each method internally calls the unchecked version if the preconditions are met.
pub trait CheckedFixedSizeMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    FixedSizeMutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `target_vt`: Token of the target vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `target_vt`.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::InvalidVertexToken`] error if `target_vt` does not satisfy the specified precondition.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`EdgeDescriptor::is_destination`] + [`FixedSizeMutEdgeDescriptor::replace_src`]).
    fn replace_src_checked(&mut self, target_vt: &VT, vt: VT) -> Result<()> {
        if !self.is_source(target_vt) {
            Err(StorageError::InvalidVertexToken(target_vt.to_string()).into())
        } else if self.is_source(&vt) {
            Err(StorageError::InvalidVertexToken(vt.to_string()).into())
        } else {
            self.replace_src(target_vt, vt);

            Ok(())
        }
    }

    /// # Arguments
    /// * `dst_vt`: Token of the destination vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `dst_vt`.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::InvalidVertexToken`] error if `target_vt` does not satisfy the specified precondition.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_destination`] + [`EdgeDescriptor::is_source`] + [`FixedSizeMutEdgeDescriptor::replace_dst`]).
    fn replace_dst_checked(&mut self, target_vt: &VT, vt: VT) -> Result<()> {
        if self.is_destination(target_vt) || (Self::is_undirected() && self.is_source(target_vt)) {
            self.replace_dst(target_vt, vt);

            Ok(())
        } else {
            Err(StorageError::InvalidVertexToken(target_vt.to_string()).into())
        }
    }
}

/// This trait builds upon [`FixedSizeMutEdgeDescriptor`] and adds more mutability to the edge.
///
/// Methods of this trait allow to change the number of sources and destinations of the edge.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// [`FixedSizeMutEdgeDescriptor`]: Technically operations described in `FixedSizeMutEdgeDescriptor` can be emulated using operations described in `MutEdgeDescriptor`.
/// Therefore, any edge that wants to implement `MutEdgeDescriptor` can and should also implement `FixedSizeMutEdgeDescriptor`.
/// This helps to describe a meaningful dependency among different edge descriptors.
pub trait MutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    FixedSizeMutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `src_vt`: Token of the vertex to be added as a source to this edge.
    /// * `dst_vt`: Token of the vertex to be added as a destination to this edge.
    ///
    /// # Preconditions
    /// The pair (`src_vt`, `dst_vt`) must not already be connected through this edge.
    ///
    /// # Postconditions
    /// Edge will contain the connection between `src_vt` and `dst_vt`.
    fn add(&mut self, src_vt: VT, dst_vt: VT);

    /// # Arguments
    /// `vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Preconditions
    /// If edge is:
    /// * Directed: `vt` must not already exist as a source of this edge.
    /// * Undirected: Edge must not contain `vt`.
    ///
    /// # Postconditions
    /// Edge will contain `vt` as one of its sources.
    fn add_src(&mut self, vt: VT);

    /// # Arguments
    /// `vt`: Token of the vertex to be added as a destination to this edge.
    ///
    /// # Preconditions
    /// If edge is:
    /// * Directed: `vt` must not already exist as a destination of this edge.
    /// * Undirected: Edge must not contain `vt`.
    ///
    /// # Postconditions
    /// Edge will contain `vt` as one its destinations.
    fn add_dst(&mut self, vt: VT);

    /// # Arguments
    /// `vt`: Token to be removed from this edge.
    ///
    /// # Preconditions
    /// `vt` must be either the token of a source or a destination vertex.
    ///
    /// # Postconditions
    /// `vt` along with all its related data is removed from edge.
    fn remove(&mut self, vt: &VT);
}

/// Checked version of [`MutEdgeDescriptor`] trait.
///
/// Note that `CheckedMutEdgeDescriptor` provides default implementation and info about time complexity for all of its methods.
/// These default implementations take into account all preconditions listed for [`MutEdgeDescriptor`] methods.
/// But if you want to override these basic implementations, make sure to:
/// 1. Take into account all the preconditions of the methods in [`MutEdgeDescriptor`] before calling them.
/// 2. Provide info about time complexity of your implementation.
///
/// # Generic parameters
/// * `VT`: The type of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * [`MutEdgeDescriptor`]: Checked version of each method internally calls the unchecked version if the preconditions are met.
pub trait CheckedMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    MutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `src_vt`: Token of the vertex to be added as a source to this edge.
    /// * `dst_vt`: Token of the vertex to be added as a destination to this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::ConnectionAlreadyExists`] error if the connection between `src_vt` and `dst_vt` already exists.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`EdgeDescriptor::is_destination`] + [`MutEdgeDescriptor::add`])
    fn add_checked(&mut self, src_vt: VT, dst_vt: VT) -> Result<()> {
        if self.is_source(&src_vt) && self.is_destination(&dst_vt) {
            Err(
                StorageError::ConnectionAlreadyExists(src_vt.to_string(), dst_vt.to_string())
                    .into(),
            )
        } else {
            self.add(src_vt, dst_vt);

            Ok(())
        }
    }

    /// # Arguments
    /// `vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically [`StorageError::VertexAlreadyExists`] error if `vt` already exists.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`MutEdgeDescriptor::add_src`])
    fn add_src_checked(&mut self, vt: VT) -> Result<()> {
        if !self.is_source(&vt) {
            self.add_src(vt);

            Ok(())
        } else {
            Err(StorageError::VertexAlreadyExists(vt.to_string()).into())
        }
    }

    /// # Arguments
    /// `vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically [`StorageError::VertexAlreadyExists`] error if `vt` already exists as the token of a destination vertex.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_destination`] + [`MutEdgeDescriptor::add_dst`])
    fn add_dst_checked(&mut self, vt: VT) -> Result<()> {
        if !self.is_destination(&vt) {
            self.add_dst(vt);

            Ok(())
        } else {
            Err(StorageError::VertexAlreadyExists(vt.to_string()).into())
        }
    }

    /// # Arguments
    /// `vt`: Token of the vertex to be removed along with all its related data.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically [`StorageError::VertexNotFound`] if edge does not contain `vt` as token (either as a source or a destination).
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::contains`] + [`MutEdgeDescriptor::remove`])
    fn remove_checked(&mut self, vt: &VT) -> Result<()> {
        if !self.contains(vt) {
            return Err(StorageError::VertexNotFound(vt.to_string()).into());
        }

        self.remove(vt);

        Ok(())
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::test_utils::get_non_duplicate;
    use rand::distributions::Standard;
    use rand::prelude::{Distribution, IteratorRandom};
    use std::collections::HashSet;

    pub fn assert_edge_description<VT, ED, const DIR: bool>(
        edge: &ED,
        src_vts_iter: impl IntoIterator<Item = VT>,
        dst_vts_iter: impl IntoIterator<Item = VT>,
    ) where
        VT: VertexToken,
        ED: EdgeDescriptor<VT, DIR>,
    {
        let src_vts: Vec<VT> = src_vts_iter.into_iter().collect();
        let dst_vts: Vec<VT> = dst_vts_iter.into_iter().collect();

        let src_vts_set = if DIR {
            HashSet::<_>::from_iter(src_vts.iter())
        } else {
            HashSet::<_>::from_iter(src_vts.iter().chain(dst_vts.iter()))
        };

        let dst_vts_set = if DIR {
            HashSet::<_>::from_iter(dst_vts.iter())
        } else {
            HashSet::<_>::from_iter(dst_vts.iter().chain(src_vts.iter()))
        };

        assert_eq!(src_vts_set, HashSet::<_>::from_iter(edge.get_sources()));
        assert_eq!(
            dst_vts_set,
            HashSet::<_>::from_iter(edge.get_destinations()),
        );

        for src_vt in src_vts.iter() {
            assert!(edge.is_source(src_vt));
            if !DIR {
                assert!(edge.is_destination(src_vt));
            }
            assert!(edge.contains(src_vt));
        }

        for dst_vt in dst_vts.iter() {
            assert!(edge.is_destination(dst_vt));
            if !DIR {
                assert!(edge.is_source(dst_vt));
            }
            assert!(edge.contains(dst_vt));
        }

        assert_eq!(edge.sources_count(), src_vts_set.len());
        assert_eq!(edge.destinations_count(), dst_vts_set.len());
    }

    pub fn prop_edge_description<VT, ED, const DIR: bool>(edge: ED)
    where
        VT: VertexToken + Clone,
        ED: EdgeDescriptor<VT, DIR>,
    {
        assert_edge_description(
            &edge,
            edge.get_sources().cloned(),
            edge.get_destinations().cloned(),
        );
    }

    pub fn prop_fixed_size_descriptor_replace_src<VT, FED, const DIR: bool>(mut edge: FED)
    where
        VT: VertexToken + Clone,
        FED: FixedSizeMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_src_vt = get_non_duplicate(src_vts.iter().cloned(), 1)[0].clone();

            edge.replace_src(&src_vt, new_src_vt.clone());

            let new_src_vts = src_vts
                .iter()
                .cloned()
                .filter(|vt| *vt != src_vt)
                .chain(std::iter::once(new_src_vt));

            test_utils::assert_edge_description(
                &edge,
                new_src_vts,
                edge.get_destinations().cloned(),
            );
        }
    }

    pub fn prop_fixed_size_descriptor_replace_dst<VT, FED, const DIR: bool>(mut edge: FED)
    where
        VT: VertexToken + Clone,
        FED: FixedSizeMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        if !dst_vts.is_empty() {
            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_dst_vt = get_non_duplicate(dst_vts.iter().cloned(), 1)[0].clone();

            edge.replace_dst(&dst_vt, new_dst_vt.clone());

            let new_dst_vts = dst_vts
                .iter()
                .cloned()
                .filter(|vt| *vt != dst_vt)
                .chain(std::iter::once(new_dst_vt));

            test_utils::assert_edge_description(&edge, edge.get_sources().cloned(), new_dst_vts);
        }
    }

    pub fn prop_checked_fixed_size_descriptor_replace_src<VT, FED, const DIR: bool>(mut edge: FED)
    where
        VT: VertexToken + Clone,
        FED: CheckedFixedSizeMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_vts = get_non_duplicate(src_vts.iter().cloned(), 2);
            let invalid_vt = new_vts[0].clone();
            let new_src_vt = new_vts[1].clone();

            assert!(edge
                .replace_src_checked(&invalid_vt, new_src_vt.clone())
                .is_err());

            assert!(edge
                .replace_src_checked(&src_vt, new_src_vt.clone())
                .is_ok());

            let new_src_vts = src_vts
                .iter()
                .cloned()
                .filter(|vt| *vt != src_vt)
                .chain(std::iter::once(new_src_vt));

            test_utils::assert_edge_description(
                &edge,
                new_src_vts,
                edge.get_destinations().cloned(),
            );
        }
    }

    pub fn prop_checked_fixed_size_descriptor_replace_dst<VT, FED, const DIR: bool>(mut edge: FED)
    where
        VT: VertexToken + Clone,
        FED: CheckedFixedSizeMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        if !dst_vts.is_empty() {
            // Choose one destination vertex randomly.
            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_vts = get_non_duplicate(dst_vts.iter().cloned(), 2);
            let invalid_vt = new_vts[0].clone();
            let new_dst_vt = new_vts[1].clone();

            assert!(edge
                .replace_dst_checked(&invalid_vt, new_dst_vt.clone())
                .is_err());

            assert!(edge
                .replace_dst_checked(&dst_vt, new_dst_vt.clone())
                .is_ok());

            let new_dst_vts = dst_vts
                .iter()
                .cloned()
                .filter(|vt| *vt != dst_vt)
                .chain(std::iter::once(new_dst_vt));

            test_utils::assert_edge_description(&edge, edge.get_sources().cloned(), new_dst_vts);
        }
    }

    pub fn prop_mut_descriptor_add_src<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: MutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        let new_src_vt = get_non_duplicate(src_vts.iter().cloned(), 1)[0].clone();

        edge.add_src(new_src_vt.clone());

        let new_src_vts = src_vts.iter().cloned().chain(std::iter::once(new_src_vt));

        test_utils::assert_edge_description(&edge, new_src_vts, edge.get_destinations().cloned());
    }

    pub fn prop_mut_descriptor_add_dst<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: MutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        let new_dst_vt = get_non_duplicate(dst_vts.iter().cloned(), 1)[0].clone();

        edge.add_dst(new_dst_vt.clone());

        let new_dst_vts = dst_vts.iter().cloned().chain(std::iter::once(new_dst_vt));

        test_utils::assert_edge_description(&edge, edge.get_sources().cloned(), new_dst_vts);
    }

    pub fn prop_mut_descriptor_add<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: MutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        let new_vts = get_non_duplicate(src_vts.iter().cloned(), 2);
        let new_src_vt = new_vts[0].clone();
        let new_dst_vt = new_vts[1].clone();

        edge.add(new_src_vt.clone(), new_dst_vt.clone());

        let new_src_vts = src_vts.iter().cloned().chain([new_src_vt]);
        let new_dst_vts = dst_vts.iter().cloned().chain([new_dst_vt]);

        test_utils::assert_edge_description(&edge, new_src_vts, new_dst_vts);
    }

    pub fn prop_mut_descriptor_remove<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: MutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.iter().choose(&mut rand::thread_rng()).unwrap();

            edge.remove(src_vt);

            let new_src_vts = src_vts.iter().cloned().filter(|vt| vt != src_vt);

            test_utils::assert_edge_description(
                &edge,
                new_src_vts,
                edge.get_destinations().cloned(),
            );
        }
    }

    pub fn prop_checked_mut_descriptor_add_src<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: CheckedMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_src_vt = get_non_duplicate(src_vts.iter().cloned(), 1)[0].clone();

            assert!(edge.add_src_checked(src_vt).is_err());
            assert!(edge.add_src_checked(new_src_vt.clone()).is_ok());

            let new_src_vts = src_vts.iter().cloned().chain(std::iter::once(new_src_vt));

            test_utils::assert_edge_description(
                &edge,
                new_src_vts,
                edge.get_destinations().cloned(),
            );
        }
    }

    pub fn prop_checked_mut_descriptor_add_dst<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: CheckedMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        if !dst_vts.is_empty() {
            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_dst_vt = get_non_duplicate(dst_vts.iter().cloned(), 1)[0].clone();

            assert!(edge.add_dst_checked(dst_vt).is_err());
            assert!(edge.add_dst_checked(new_dst_vt.clone()).is_ok());

            let new_dst_vts = dst_vts.iter().cloned().chain(std::iter::once(new_dst_vt));

            test_utils::assert_edge_description(&edge, edge.get_sources().cloned(), new_dst_vts);
        }
    }

    pub fn prop_checked_mut_descriptor_add<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: CheckedMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();
        let dst_vts: Vec<VT> = edge.get_destinations().cloned().collect();

        if !src_vts.is_empty() && !dst_vts.is_empty() {
            let src_vt = src_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let dst_vt = dst_vts
                .iter()
                .choose(&mut rand::thread_rng())
                .cloned()
                .unwrap();

            let new_vts = get_non_duplicate(src_vts.iter().cloned(), 2);
            let new_src_vt = new_vts[0].clone();
            let new_dst_vt = new_vts[1].clone();

            assert!(edge.add_checked(src_vt, dst_vt).is_err());
            assert!(edge
                .add_checked(new_src_vt.clone(), new_dst_vt.clone())
                .is_ok());

            let new_src_vts = src_vts.iter().cloned().chain([new_src_vt]);
            let new_dst_vts = dst_vts.iter().cloned().chain([new_dst_vt]);

            test_utils::assert_edge_description(&edge, new_src_vts, new_dst_vts);
        }
    }

    pub fn prop_checked_mut_descriptor_remove<VT, MED, const DIR: bool>(mut edge: MED)
    where
        VT: VertexToken + Clone,
        MED: CheckedMutEdgeDescriptor<VT, DIR>,
        Standard: Distribution<VT>,
    {
        let src_vts: Vec<VT> = edge.get_sources().cloned().collect();

        if !src_vts.is_empty() {
            let src_vt = src_vts.iter().choose(&mut rand::thread_rng()).unwrap();

            let invalid_vt = get_non_duplicate(src_vts.iter().cloned(), 1)[0].clone();

            assert!(edge.remove_checked(&invalid_vt).is_err());
            assert!(edge.remove_checked(src_vt).is_ok());

            let new_src_vts = src_vts.iter().cloned().filter(|vt| vt != src_vt);

            test_utils::assert_edge_description(
                &edge,
                new_src_vts,
                edge.get_destinations().cloned(),
            );
        }
    }
}
