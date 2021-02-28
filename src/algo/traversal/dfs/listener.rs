use super::Dfs;

#[allow(unused_variables)]
/// Trait for structures that want to listen to `Dfs` events.
pub trait DfsListener<L: DfsListener = Self> {
    /// Gets called by `Dfs` when starting the algorithm.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `dfs`: `Dfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Dfs` provides.
    fn on_start(&mut self, dfs: &Dfs<L>, virt_id: usize) {}

    /// Gets called by `Dfs` when visiting a vertex for the first time.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `dfs`: `Dfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Dfs` provides.
    fn on_white(&mut self, dfs: &Dfs<L>, virt_id: usize) {}

    /// Gets called by `Dfs` when visiting a vertex that has been on the stack(it's now on top of the stack).
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `dfs`: `Dfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Dfs` provides.
    fn on_gray(&mut self, dfs: &Dfs<L>, virt_id: usize) {}

    /// Gets called by `Dfs` when a vertex is permanently visited(removed from the stack).
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `dfs`: `Dfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Dfs` provides.
    fn on_black(&mut self, dfs: &Dfs<L>, virt_id: usize) {}

    /// Gets called by `Dfs` when finishing the algorithm.
    /// This function may called multiple times because graph may not be connected.
    ///
    /// # Arguments
    /// * `dfs`: `Dfs` struct. You can query it for its stack, discovered vertices and etc.
    /// * `virt_id`: Virtual id of the vertex. You can access the real id of the vertex by using `IdMap` that `Dfs` provides.
    fn on_finish(&mut self, dfs: &Dfs<L>) {}
}
