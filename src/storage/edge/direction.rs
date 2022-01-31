/// Describes wether an entity is directed or not.
///
/// The entity can be an edge, a graph or any other struct that can act as an entity with direction.
pub trait Direction {
    /// # Returns
    /// * `true`: If the entity has direction.
    /// * `false`: Otherwise.
    fn is_directed() -> bool;

    /// # Returns
    /// * `true`: If the entity has no direction.
    /// * `false`: Otherwise.
    fn is_undirected() -> bool;
}

/// Default implementation for a directed entity.
#[derive(Debug, Clone, Copy)]
pub struct Directed;
impl Direction for Directed {
    fn is_directed() -> bool {
        true
    }

    fn is_undirected() -> bool {
        false
    }
}

/// Default implementation for an undirected entity.
#[derive(Debug, Clone, Copy)]
pub struct Undirected;
impl Direction for Undirected {
    fn is_directed() -> bool {
        false
    }

    fn is_undirected() -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use quickcheck::Arbitrary;

    use super::{Directed, Undirected};

    impl Arbitrary for Directed {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Directed {}
        }
    }

    impl Arbitrary for Undirected {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Undirected {}
        }
    }
}
