pub trait EdgeDirection {
    fn is_directed() -> bool;

    fn is_undirected() -> bool {
        !Self::is_directed()
    }
}

pub struct DirectedEdge;
impl EdgeDirection for DirectedEdge {
    fn is_directed() -> bool {
        true
    }
}

pub struct UndirectedEdge;
impl EdgeDirection for UndirectedEdge {
    fn is_directed() -> bool {
        false
    }
}
