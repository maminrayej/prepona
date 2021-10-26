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
    /// * `true`: If `vt` is the token of one of the sources.
    /// * `false`: Otherwise.
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
    /// * `true`: If `vt` is the token of one of the destinations.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O(|[`EdgeDescriptor::get_destinations`] + V<sub>dst</sub>|)
    fn is_destination(&self, vt: &VT) -> bool {
        self.get_destinations().any(|dst_vt| dst_vt == vt)
    }

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a source or a destination.
    ///
    /// # Returns
    /// * `true`: If one of the sources or destinations has token equal to `vt`.
    /// * `false`: Otherwise.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`EdgeDescriptor::is_destination`])
    fn contains(&self, vt: &VT) -> bool {
        self.is_source(vt) || self.is_destination(vt)
    }

    /// # Returns
    /// Number of sources.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::get_sources`] + |V<sub>src</sub>|)
    fn sources_count(&self) -> usize {
        self.get_sources().count()
    }

    /// # Returns
    /// Number of destinations.
    ///
    /// # Complexity
    /// O(|[`EdgeDescriptor::get_destinations`] + V<sub>dst</sub>|)
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
    /// * `src_vt`: Token of the source vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `src_vt`.
    ///
    /// # Preconditions
    /// `src_vt` must be the token of one of the source vertices in this edge.
    ///
    /// # Postconditions
    /// `src_vt` will be replaced by `vt`.
    fn replace_src(&mut self, src_vt: &VT, vt: VT);

    /// # Arguments
    /// * `dst_vt`: Token of the destination vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replaced `dst_vt`.
    ///
    /// # Preconditions
    /// `dst_vt` must be the token of one of the destination vertices in this edge.
    ///
    /// # Postconditions
    /// `dst_vt` is replaced with `vt`.
    fn replace_dst(&mut self, dst_vt: &VT, vt: VT);
}

/// Checked version of [`FixedSizeMutEdgeDescriptor`] trait.
///
/// Note that `CheckedFixedSizeMutEdgeDescriptor` provides default implementation and info about [complexity] for all of its methods.
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
///
/// [complexity]: https://en.wikipedia.org/wiki/Time_complexity
pub trait CheckedFixedSizeMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    FixedSizeMutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `src_vt`: Token of the source vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `src_vt`.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::NotSource`] error if `src_vt` is not the token of a source vertex in the edge.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`FixedSizeMutEdgeDescriptor::replace_src`]).
    fn replace_src_checked(&mut self, src_vt: &VT, vt: VT) -> Result<()> {
        if !self.is_source(src_vt) {
            Err(StorageError::NotSource(src_vt.to_string()).into())
        } else {
            self.replace_src(src_vt, vt);

            Ok(())
        }
    }

    /// # Arguments
    /// * `dst_vt`: Token of the destination vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `dst_vt`.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::NotDestination`] error if `dst_vt` is not the token of a destination vertex in the edge.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_destination`] + [`FixedSizeMutEdgeDescriptor::replace_dst`]).
    fn replace_dst_checked(&mut self, dst_vt: &VT, vt: VT) -> Result<()> {
        if !self.is_destination(dst_vt) {
            Err(StorageError::NotDestination(dst_vt.to_string()).into())
        } else {
            self.replace_dst(dst_vt, vt);

            Ok(())
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
    /// The pair (`src_vt`, `dst_vt`) must not already exist in the edge.
    /// In other words, a source vertex with token: `src_vt` must not already be connected to a destination vertex with token: `dst_vt` using this edge.
    ///
    /// # Postconditions
    /// Edge will contain the connection between `src_vt` and `dst_vt`.
    fn add(&mut self, src_vt: VT, dst_vt: VT);

    /// # Arguments
    /// `src_vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Preconditions
    /// `src_vt` must not already exist as a source of this edge.
    ///
    /// # Postconditions
    /// Edge will contain `src_vt` as one of its sources.
    fn add_src(&mut self, src_vt: VT);

    /// # Arguments
    /// `dst_vt`: Token of the vertex to be added as a destination to this edge.
    ///
    /// # Preconditions
    /// `dst_vt` must not already exist as a destination of this edge.
    ///
    /// # Postconditions
    /// Edge will contain `dst_vt` as one its destinations.
    fn add_dst(&mut self, dst_vt: VT);

    /// # Arguments
    /// `vt`: Token to be removed from this edge.
    ///
    /// # Preconditions
    /// `vt` must be either the token of a source or a destination vertex.
    ///
    /// # Postconditions
    /// `vt` along with all its related data is removed from edge.
    fn remove(&mut self, vt: VT);
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
    /// `src_vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically [`StorageError::VertexAlreadyExists`] error if `src_vt` already exists as the token of a source vertex.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_source`] + [`MutEdgeDescriptor::add_src`])
    fn add_src_checked(&mut self, src_vt: VT) -> Result<()> {
        if self.is_source(&src_vt) {
            Err(StorageError::VertexAlreadyExists(src_vt.to_string()).into())
        } else {
            self.add_src(src_vt);

            Ok(())
        }
    }

    /// # Arguments
    /// `dst_vt`: Token of the vertex to be added as a source to this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically [`StorageError::VertexAlreadyExists`] error if `dst_vt` already exists as the token of a destination vertex.
    ///
    /// # Complexity
    /// O([`EdgeDescriptor::is_destination`] + [`MutEdgeDescriptor::add_dst`])
    fn add_dst_checked(&mut self, dst_vt: VT) -> Result<()> {
        if self.is_destination(&dst_vt) {
            Err(StorageError::VertexAlreadyExists(dst_vt.to_string()).into())
        } else {
            self.add_dst(dst_vt);

            Ok(())
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
    fn remove_checked(&mut self, vt: VT) -> Result<()> {
        if !self.contains(&vt) {
            return Err(StorageError::VertexNotFound(vt.to_string()).into());
        }

        self.remove(vt);

        Ok(())
    }
}
