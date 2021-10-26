/// Describes wether an entity is directed or not.
///
/// The entity can be an edge, a graph or any other struct that can act as an entity with direction.
pub trait Direction<const DIR: bool> {
    /// # Returns
    /// * `true`: If the entity has direction.
    /// * `false`: Otherwise.
    fn is_directed() -> bool {
        DIR
    }

    /// # Returns
    /// * `true`: If the entity has no direction.
    /// * `false`: Otherwise.
    fn is_undirected() -> bool {
        !DIR
    }
}

/// Default implementation for a directed entity.
pub struct Directed;
impl Direction<true> for Directed {}

/// Default implementation for an undirected entity.
pub struct Undirected;
impl Direction<false> for Undirected {}
