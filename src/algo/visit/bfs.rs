use std::collections::VecDeque;

use crate::algo::visit::ControlFlow;
use crate::provide::*;

use super::macros::on_event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BfsEvent {
    Begin(NodeID),
    Discover(NodeID),
    Finish(NodeID),
    End(NodeID),
}

#[derive(Debug)]
pub struct Bfs<'a, S: Storage> {
    storage: &'a S,
    idmap: S::Map,
    starters: Vec<NodeID>,

    color: Vec<Color>,
}

impl<'a, S> Bfs<'a, S>
where
    S: Node + Edge,
{
    pub fn init(storage: &'a S) -> Self {
        Self::with_starters(storage, vec![])
    }

    pub fn with_starters(storage: &'a S, starters: Vec<NodeID>) -> Self {
        let node_count = storage.node_count();

        Self {
            storage,
            idmap: storage.idmap(),
            starters,
            color: vec![Color::White; node_count],
        }
    }

    #[rustfmt::skip]
    fn next_start(&mut self) -> Option<NodeID> {
        if !self.starters.is_empty() {
            Some(self.starters.swap_remove(0))
        } else {
            self.color.iter().position(|c| *c == Color::White).map(|i| self.idmap[i])
        }
    }

    pub fn execute(&mut self, callback: impl FnMut(BfsEvent) -> ControlFlow) {
        self._execute(callback);
    }

    fn _execute(&mut self, mut callback: impl FnMut(BfsEvent) -> ControlFlow) -> ControlFlow {
        let mut queue = VecDeque::new();

        while let Some(start) = self.next_start() {
            on_event!(callback(BfsEvent::Begin(start)));

            queue.push_back(start);

            while let Some(next) = queue.pop_front() {
                let next_vid = self.idmap[next];
                let color = self.color[next_vid];

                match color {
                    Color::White => {
                        on_event!(callback(BfsEvent::Discover(next)));

                        self.color[next_vid] = Color::Gray;

                        queue.extend(self.storage.succs(next));
                        queue.push_back(next);
                    }
                    Color::Gray => {
                        on_event!(callback(BfsEvent::Finish(next)));

                        self.color[next_vid] = Color::Black;
                    }
                    Color::Black => {}
                }
            }
            on_event!(callback(BfsEvent::End(start)));
        }

        ControlFlow::Break(())
    }
}
