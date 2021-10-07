use super::common::Token;
use super::vertex::CheckedVertexReport;

// Edge related traits
pub trait EdgeManipulation<VT: Token, E, EA, ET: Token> {
    fn add_edge(&mut self, source_vertex_token: &VT, destination_vertex_token: &VT, edge: E) -> ET;

    fn remove_edge(
        &mut self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    );

    fn clear_edges(&mut self);
}

pub trait CheckedEdgeManipulation<'a, VT: Token, E, EA: 'a, ET: Token>:
    EdgeManipulation<VT, E, EA, ET> + EdgeReport<'a, VT, EA, ET>
{
    fn add_edge_checked(
        &mut self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge: E,
    ) -> ET;

    fn remove_edge_checked(
        &mut self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    );

    fn clear_edges_checked(&mut self);
}

pub trait EdgeReport<'a, VT: Token, EA: 'a, ET: Token> {
    fn edges_iter<Iter: Iterator<Item = &'a EA>>(&self) -> Iter;

    fn has_edge(
        &self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    ) -> bool;

    fn edge_attribute_of(
        &self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    ) -> &EA;

    fn edge_count(&self) -> usize;
}

pub trait CheckedEdgeReport<'a, V, VA: 'a, VT: Token, EA: 'a, ET: Token>:
    EdgeReport<'a, VT, EA, ET> + CheckedVertexReport<'a, V, VA, VT>
{
    fn edges_iter_checked<Iter: Iterator<Item = &'a EA>>(&self) -> Iter;

    fn has_edge_checked(
        &self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    ) -> bool {
        if !self.has_vertex_checked(source_vertex_token) {
            panic!("Source does not exist")
        } else if !self.has_vertex_checked(destination_vertex_token) {
            panic!("Destination does not exist")
        }

        self.has_edge(source_vertex_token, destination_vertex_token, edge_token)
    }

    fn edge_attribute_of_checked(
        &self,
        source_vertex_token: &VT,
        destination_vertex_token: &VT,
        edge_token: &ET,
    ) -> &EA {
        if !self.has_edge_checked(source_vertex_token, destination_vertex_token, edge_token) {
            panic!("Edge does not exist")
        }

        self.edge_attribute_of(source_vertex_token, destination_vertex_token, edge_token)
    }

    fn edge_count_checked(&self) -> usize {
        self.edge_count()
    }
}
