use super::Dfs;

#[allow(unused_variables)]
pub trait DfsListener<L: DfsListener = Self> {
    fn on_start(&mut self, dfs: &Dfs<L>, virt_id: usize) {}
    fn on_white(&mut self, dfs: &Dfs<L>, virt_id: usize) {}
    fn on_gray(&mut self, dfs: &Dfs<L>, virt_id: usize) {}
    fn on_black(&mut self, dfs: &Dfs<L>, virt_id: usize) {}
    fn on_finish(&mut self, dfs: &Dfs<L>) {}
}
