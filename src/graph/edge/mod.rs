mod default_edge;
mod flow_edge;

use magnitude::Magnitude;

pub use default_edge::DefaultEdge;
pub use flow_edge::FlowEdge;

pub trait EdgeDir {
    fn is_directed() -> bool;
    fn is_undirected() -> bool;
}

pub struct DirectedEdge;
impl EdgeDir for DirectedEdge {
    fn is_directed() -> bool {
        true
    }

    fn is_undirected() -> bool {
        false
    }
}

pub struct UndirectedEdge;
impl EdgeDir for UndirectedEdge {
    fn is_directed() -> bool {
        false
    }

    fn is_undirected() -> bool {
        true
    }
}

pub trait Edge<W> {
    fn init(weight: Magnitude<W>) -> Self;

    fn get_weight(&self) -> &Magnitude<W>;

    fn set_weight(&mut self, weight: Magnitude<W>);

    fn set_id(&mut self, id: usize);

    fn get_id(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directed_edge() {
        assert!(DirectedEdge::is_directed());
        assert!(!DirectedEdge::is_undirected());
    }

    #[test]
    fn test_undirected_edge() {
        assert!(UndirectedEdge::is_undirected());
        assert!(!UndirectedEdge::is_directed());
    }
}
