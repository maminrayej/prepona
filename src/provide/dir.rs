pub trait Direction {
    fn is_directed() -> bool;

    fn is_undirected() -> bool {
        !Self::is_directed()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Directed;
impl Direction for Directed {
    fn is_directed() -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Undirected;
impl Direction for Undirected {
    fn is_directed() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Incoming,
    Outgoing,
}

#[cfg(test)]
mod arbitrary {
    use quickcheck::Arbitrary;

    use super::{Directed, Undirected};

    impl Arbitrary for Directed {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Directed
        }
    }

    impl Arbitrary for Undirected {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Undirected
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_type() {
        assert!(Directed::is_directed());
        assert!(!Directed::is_undirected());
    }

    #[test]
    fn undirected_type() {
        assert!(!Undirected::is_directed());
        assert!(Undirected::is_undirected());
    }
}
