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
/// `EdgeDescriptor` must be generic and flexible enough so that any kind of edge can be modeled through it.
/// Therefore, in the most general sense it is considered that each edge can connect multiple sources to multiple destinations at once[^note].
/// It is a given that connecting one source to one destination is included in this definition.
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * `PartialEq`, `Eq`: Each edge must be comparable with another edge that is of the same type.
/// * [`Direction`]: Each must be either directed or undirected.
///
/// [^note]: In even more [`general`] sense an edge can connect to other edges. But, they're not accounted for in our definitions.
///
/// [`general`]: https://en.wikipedia.org/wiki/Hypergraph#Further_generalizations
pub trait EdgeDescriptor<VT: VertexToken, const DIR: bool>:
    PartialEq + Eq + Direction<DIR>
{
    /// # Returns
    /// An iterator over tokens of source vertices.
    fn get_sources(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    /// # Returns
    /// An iterator over tokens of destination vertices.
    fn get_destinations(&self) -> Box<dyn Iterator<Item = &VT> + '_>;

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a source or not.
    ///
    /// # Returns
    /// * `true`: If one of the sources of this edge has token: `vt`.
    /// * `false`: Otherwise.
    fn is_source(&self, vt: &VT) -> bool {
        self.get_sources().any(|src_vt| src_vt == vt)
    }

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a destination or not.
    ///
    /// # Returns
    /// * `true`: If one of the destinations of this edge has token: `vt`.
    /// * `false`: Otherwise.
    fn is_destination(&self, vt: &VT) -> bool {
        self.get_destinations().any(|dst_vt| dst_vt == vt)
    }

    /// # Arguments
    /// * `vt`: Token of the vertex to be checked if it's a source or a destination.
    ///
    /// # Returns
    /// * `true`: If one of the sources or destinations of this edge has token: `vt`.
    /// * `false`: Otherwise.
    fn contains(&self, vt: &VT) -> bool {
        self.is_source(vt) || self.is_destination(vt)
    }

    /// # Returns
    /// Number of sources of this edge.
    fn sources_count(&self) -> usize {
        self.get_sources().count()
    }

    /// # Returns
    /// Number of destinations of this edge.
    fn destinations_count(&self) -> usize {
        self.get_destinations().count()
    }
}

/// This trait builds upon [`EdgeDescriptor`] and adds limited mutability to the edge.
///
/// This trait allows mutability as long as the number of sources and destinations of the edge remain the same.
/// Therefore it only allows replacing tokens.
/// If you want adding and removal of tokens, checkout [`MutEdgeDescriptor`].
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
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
    /// `src_vt` must be the token of a source vertex in this edge.
    ///
    /// # Postconditions
    /// `src_vt` will be replaced by `vt`.
    fn replace_src(&mut self, src_vt: &VT, vt: VT);

    /// # Arguments
    /// * `dst_vt`: Token of the destination vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replaced `dst_vt`.
    ///
    /// # Preconditions
    /// `dst_vt` must be the token of a destination vertex in this edge.
    ///
    /// # Postconditions
    /// `dst_vt` is replaced with `vt`.
    fn replace_dst(&mut self, dst_vt: &VT, vt: VT);
}

/// Checked version of [`FixedSizeMutEdgeDescriptor`] trait.
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * [`FixedSizeMutEdgeDescriptor`]: Checked version of each method internally calls the unchecked version if the preconditions are met.
pub trait CheckedFixedSizeMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    FixedSizeMutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `src_vt`: Token of the source vertex that is going to be replaced.
    /// * `vt`: Token of the vertex to replace `src_vt`.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Specifically a [`StorageError::NotSource`] error if `src_vt` is not the token of a source vertex in this edge.
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
    /// * `Err`: Specifically a [`StorageError::NotDestination`] error if `dst_vt` is not the token of a destination vertex in this edge.
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
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
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
    /// * `src_vt`: Token of the vertex to be added as source in this edge.
    /// * `dst_vt`: Token of the vertex to be added as destination in this edge.
    ///
    /// # Preconditions
    /// The tuple (`src_vt`, `dst_vt`) must not already exist in the edge.
    /// In other words, a source vertex with token: `src_vt` must not be already connected to a destination vertex with token: `dst_vt`.
    ///
    /// # Postconditions
    /// Edge will contain connection between `src_vt` and `dst_vt`.
    fn add(&mut self, src_vt: VT, dst_vt: VT);

    /// # Arguments
    /// `src_vt`: Token of the vertex to be added as source in this edge.
    ///
    /// # Preconditions
    /// `src_vt` must not already exist as a source of this edge.
    ///
    /// # Postconditions
    /// Edge will contain `src_vt` as one of its sources.
    fn add_src(&mut self, src_vt: VT);

    /// # Arguments
    /// `dst_vt`: Token of the vertex to be added as destination in this edge.
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
    /// `vt` must be either the token of a source or destination vertex.
    ///
    /// # Postconditions
    /// `vt` along with all its related data is removed from edge.
    fn remove(&mut self, vt: VT);
}

/// Checked version of [`MutEdgeDescriptor`] trait.
///
/// # Generic parameters
/// * `VT`: The kind of token that represents the sources and destinations of the edge.
/// * `DIR`: Specifies wether the edge is directed or not.
///
/// # Required traits
/// * [`MutEdgeDescriptor`]: Checked version of each method internally calls the unchecked version if the preconditions are met.
pub trait CheckedMutEdgeDescriptor<VT: VertexToken, const DIR: bool>:
    MutEdgeDescriptor<VT, DIR>
{
    /// # Arguments
    /// * `src_vt`: Token of the vertex to be added as source in this edge.
    /// * `dst_vt`: Token of the vertex to be added as destination in this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Especially a [`StorageError::ConnectionAlreadyExists`] error if the connection between `src_vt` and `dst_vt` already exists.
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
    /// `src_vt`: Token of the vertex to be added as a source in this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Especially [`StorageError::VertexAlreadyExists`] error if `src_vt` already exists as the token of a source vertex.
    fn add_src_checked(&mut self, src_vt: VT) -> Result<()> {
        if self.is_source(&src_vt) {
            Err(StorageError::VertexAlreadyExists(src_vt.to_string()).into())
        } else {
            self.add_src(src_vt);

            Ok(())
        }
    }

    /// # Arguments
    /// `dst_vt`: Token of the vertex to be added as a source in this edge.
    ///
    /// # Returns
    /// * `Ok`: If preconditions are met.
    /// * `Err`: Especially [`StorageError::VertexAlreadyExists`] error if `dst_vt` already exists as the token of a destination vertex.
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
    /// * `Err`: Especially [`StorageError::VertexNotFound`] if edge does not contain `vt` as token (either a source or a destination).
    fn remove_checked(&mut self, vt: VT) -> Result<()> {
        if !self.contains(&vt) {
            return Err(StorageError::VertexNotFound(vt.to_string()).into());
        }

        self.remove(vt);

        Ok(())
    }
}
