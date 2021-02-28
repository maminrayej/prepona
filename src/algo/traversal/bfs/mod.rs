mod listener;

pub use listener::BfsListener;

use magnitude::Magnitude;
use std::{cell::RefCell, collections::VecDeque};

use super::Color;
use crate::provide::{self, IdMap};

/// Visits graph vertices in a breath-first manner.
pub struct Bfs<'a, L: BfsListener> {
    queue: VecDeque<usize>,
    colors: Vec<Color>,
    discovered: Vec<Magnitude<usize>>,
    finished: Vec<Magnitude<usize>>,
    time: usize,
    id_map: IdMap,
    start_ids: Vec<usize>,
    listener: RefCell<&'a mut L>,
}

impl<'a, L: BfsListener> Bfs<'a, L> {
    /// Initializes the structure.
    ///
    /// # Arguments
    /// * `graph`: Graph to perform the Bfs on.
    /// * `listener`: To listen to bfs events.
    pub fn init<G>(graph: &G, listener: &'a mut L) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        Bfs::init_with_starts(graph, listener, vec![])
    }

    /// Initializes the structure.
    ///
    /// # Arguments
    /// * `graph`: Graph to perform the BFS on.
    /// * `listener`: To listen to bfs events.
    /// * `start_ids`: List of ids to start the bfs from.
    pub fn init_with_starts<G>(graph: &G, listener: &'a mut L, mut start_ids: Vec<usize>) -> Self
    where
        G: provide::Vertices + provide::Neighbors,
    {
        let vertex_count = graph.vertex_count();

        let id_map = graph.continuos_id_map();

        start_ids = start_ids
            .into_iter()
            .map(|real_id| id_map.virt_id_of(real_id))
            .collect();

        Bfs {
            queue: VecDeque::new(),
            colors: vec![Color::White; vertex_count],
            discovered: vec![Magnitude::PosInfinite; vertex_count],
            finished: vec![Magnitude::PosInfinite; vertex_count],
            time: 0,
            id_map: graph.continuos_id_map(),
            listener: RefCell::new(listener),
            start_ids,
        }
    }

    fn next_start_id(&self) -> Option<usize> {
        if self.start_ids.is_empty() {
            self.colors.iter().position(|color| *color == Color::White)
        } else {
            self.start_ids
                .iter()
                .find(|virt_id| self.colors[**virt_id] == Color::White)
                .and_then(|virt_id| Some(*virt_id))
        }
    }

    /// Performs Bfs visit and calls the listener on every event.
    pub fn execute<G>(&mut self, graph: &G)
    where
        G: provide::Vertices + provide::Neighbors,
    {
        while let Some(start_id) = self.next_start_id() {
            self.time += 1;
            self.queue.push_back(start_id);
            self.listener.borrow_mut().on_start(self, start_id);

            while let Some(virt_id) = self.queue.pop_front() {
                let color = self.colors[virt_id];

                match color {
                    Color::White => {
                        self.time += 1;
                        self.discovered[virt_id] = self.time.into();
                        self.listener.borrow_mut().on_white(self, virt_id);

                        self.colors[virt_id] = Color::Gray;

                        let real_id = self.id_map.real_id_of(virt_id);

                        let mut neighbors = graph
                            .neighbors_unchecked(real_id)
                            .into_iter()
                            .map(|real_id| self.id_map.virt_id_of(real_id))
                            .filter(|virt_id| self.colors[*virt_id] == Color::White)
                            .collect();

                        self.queue.push_back(virt_id);
                        self.queue.append(&mut neighbors);
                    }
                    Color::Gray => {
                        self.listener.borrow_mut().on_gray(self, virt_id);

                        self.colors[virt_id] = Color::Black;
                        self.time += 1;
                        self.finished[virt_id] = self.time.into();
                        self.listener.borrow_mut().on_black(self, virt_id);
                    }
                    Color::Black => {}
                }
            }
            self.listener.borrow_mut().on_finish(self);
        }
    }

    /// # Returns
    /// Queue of the bfs structure.
    pub fn get_queue(&self) -> &VecDeque<usize> {
        &self.queue
    }

    /// # Returns
    /// Color of each vertex. Note that color of vertex with virtual id of `i` is in `get_colors()[i]`.
    pub fn get_colors(&self) -> &Vec<Color> {
        &self.colors
    }

    /// # Returns
    /// discovered time of each vertex. Note that discovered time of vertex with virtual id of `i` is in `get_colors()[i]`.
    pub fn get_discovered(&self) -> &Vec<Magnitude<usize>> {
        &self.discovered
    }

    /// # Returns
    /// finished time of each vertex. Note that finished time of vertex with virtual id of `i` is in `get_colors()[i]`.
    pub fn get_finished(&self) -> &Vec<Magnitude<usize>> {
        &self.finished
    }

    /// # Returns
    /// `IdMap` used by `Bfs` to map real ids to virtual ids(and vice versa).
    pub fn get_id_map(&self) -> &IdMap {
        &self.id_map
    }

    /// # Returns
    /// (Discovered time, Finished time, `IdMap`) 
    pub fn dissolve(self) -> (Vec<Magnitude<usize>>, Vec<Magnitude<usize>>, IdMap) {
        (self.discovered, self.finished, self.id_map)
    }
}