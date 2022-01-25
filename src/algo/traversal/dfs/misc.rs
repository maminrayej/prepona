use std::collections::HashMap;

use magnitude::Magnitude;

use crate::common::{IdMap, RealID};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexColor {
    White,
    Gray,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    ForwardEdge(RealID, RealID),
    TreeEdge(RealID, RealID),
    BackEdge(RealID, RealID),
    CrossEdge(RealID, RealID),
}

pub struct State {
    pub id_map: IdMap,

    pub time: usize,
    pub discover: Vec<Magnitude<usize>>,
    pub finished: Vec<Magnitude<usize>>,
    pub parent: HashMap<usize, usize>,
}

pub enum Event<'a> {
    Begin(&'a State, RealID),
    Discover(&'a State, RealID),
    Finish(&'a State, RealID),
    End(&'a State),
    VisitEdge(&'a State, EdgeType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Continue,
    Prune,
    Return,
}
