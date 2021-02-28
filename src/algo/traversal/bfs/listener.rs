use super::Bfs;

#[allow(unused_variables)]
pub trait BfsListener<L: BfsListener = Self> {
    fn on_start(&mut self, bfs: &Bfs<L>, virt_id: usize) {}
    fn on_white(&mut self, bfs: &Bfs<L>, virt_id: usize) {}
    fn on_gray(&mut self, bfs: &Bfs<L>, virt_id: usize) {}
    fn on_black(&mut self, bfs: &Bfs<L>, virt_id: usize) {}
    fn on_finish(&mut self, bfs: &Bfs<L>) {}
}
