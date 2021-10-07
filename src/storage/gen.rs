// Initilization or generation traits
pub trait Init<SA> {
    fn init() -> Self;

    fn init_with_capacity(vertex_count: usize, edge_count: usize);
}
