// Vertex related traits
use super::common::Token;

pub trait VertexManipulation<V, VA, VT: Token> {
    fn add_vertex(&mut self, vertex: V) -> VT;

    fn remove_vertex(&mut self, vertex_token: &VT);

    fn clear_vertices(&mut self);
}

pub trait CheckedVertexManipulation<'a, V, VA: 'a, VT: Token>:
    VertexManipulation<V, VA, VT> + VertexReport<'a, V, VA, VT>
{
    fn add_vertex_checked(&mut self, vertex: V) -> VT;

    fn remove_vertex_checked(&mut self, vertex_token: &VT) {
        if !self.has_vertex(vertex_token) {
            panic!("Vertex does not exist")
        }

        self.remove_vertex(vertex_token);
    }

    fn clear_verties_checked(&mut self) {
        self.clear_vertices();
    }
}

pub trait VertexReport<'a, V, VA: 'a, VT: Token> {
    fn vertices_iter<Iter: Iterator<Item = &'a VA>>(&self) -> Iter;

    fn has_vertex(&self, vertex_token: &VT) -> bool;

    fn vertex_attribute_of(&self, vertex_token: &VT) -> &VA;

    fn neighbors_of(&self, vertex_token: &VT);

    fn vertex_count(&self) -> usize;
}

pub trait CheckedVertexReport<'a, V, VA: 'a, VT: Token>: VertexReport<'a, V, VA, VT> {
    fn vertices_iter_checked<Iter: Iterator<Item = &'a VA>>(&self) -> Iter {
        self.vertices_iter()
    }

    fn has_vertex_checked(&self, vertex_token: &VT) -> bool {
        self.has_vertex(vertex_token)
    }

    fn vertex_attribute_of_checked(&self, vertex_token: &VT) -> &VA {
        if !self.has_vertex(vertex_token) {
            panic!("Vertex does not exist")
        }

        self.vertex_attribute_of(vertex_token)
    }

    fn neighbors_of_checked(&self, vertex_token: &VT) {
        if !self.has_vertex(vertex_token) {
            panic!("Vertex does not exist")
        }

        self.neighbors_of(vertex_token)
    }

    fn vertex_count_checked(&self) -> usize {
        self.vertex_count()
    }
}
