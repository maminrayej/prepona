use magnitude::Magnitude;
pub trait Neighbors {
    fn neighbors(&self, v_index: usize) -> Vec<usize>;
}

pub trait Vertices {
    fn vertices(&self) -> Vec<usize>;

    fn vertex_count(&self) -> usize {
        self.vertices().len()
    }
}

pub trait Edges<W> {
    fn edges(&self) -> Vec<(usize, usize, Magnitude<W>)>;

    fn edges_from(&self, src_index: usize) -> Vec<(usize, Magnitude<W>)> {
        self.edges()
            .into_iter()
            .filter(|(v1, _, _)| *v1 == src_index)
            .map(|(_, v2, weight)| (v2, weight))
            .collect()
    }

    fn edges_count(&self) -> usize {
        self.edges().len()
    }
}

pub trait Graph<W> {
    fn add_vertex(&mut self) -> usize;

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize, edge_weight: Magnitude<W>);

    fn remove_edge(&mut self, src_vertex_id: usize, dst_vertex_id: usize) -> Magnitude<W>;
}
