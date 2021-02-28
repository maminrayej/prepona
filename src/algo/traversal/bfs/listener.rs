use super::Bfs;

#[allow(unused_variables)]
/// Trait for structures that want to listen to `Bfs` events.
pub trait BfsListener<L: BfsListener = Self> {
    /// Gets called by `Bfs` when starting the algorithm.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `bfs`: `Bfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Bfs` provides.
    fn on_start(&mut self, bfs: &Bfs<L>, virt_id: usize) {}

    /// Gets called by `Bfs` when visiting a vertex for the first time.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `bfs`: `Bfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Bfs` provides.
    fn on_white(&mut self, bfs: &Bfs<L>, virt_id: usize) {}

    /// Gets called by `Bfs` when visiting a vertex that has been on the stack(it's now on top of the stack).
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `bfs`: `Bfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Bfs` provides.
    fn on_gray(&mut self, bfs: &Bfs<L>, virt_id: usize) {}

    /// Gets called by `Bfs` when a vertex is permanently visited(removed from the stack).
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `bfs`: `Bfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Bfs` provides.
    fn on_black(&mut self, bfs: &Bfs<L>, virt_id: usize) {}

    /// Gets called by `Bfs` when finishing the algorithm.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `bfs`: `Bfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Bfs` provides.
    fn on_finish(&mut self, bfs: &Bfs<L>) {}
}
