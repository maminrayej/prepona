use itertools::Itertools;

use crate::{
    common::{DynIter, IdMap},
    provide::{Edges, Vertices},
    storage::edge::Directed,
};

pub struct DfsBuilder<'a, G, Handler>
where
    Handler: Fn(DfsEvent) -> bool,
    G: Vertices<Dir = Directed> + Edges<Dir = Directed>,
{
    graph: &'a G,
    handler: Option<Handler>,
    vertices: DynIter<'a, usize>,
    depth_limit: usize,
}

impl<'a, G, Handler> DfsBuilder<'a, G, Handler>
where
    Handler: Fn(DfsEvent) -> bool,
    G: Vertices<Dir = Directed> + Edges<Dir = Directed>,
{
    pub fn init(graph: &'a G) -> Self {
        DfsBuilder {
            graph,
            handler: None,
            vertices: DynIter::init(None.into_iter()),
            depth_limit: usize::MAX,
        }
    }

    pub fn vertices(mut self, vertices: impl Iterator<Item = usize> + 'a) -> Self {
        self.vertices = DynIter::init(vertices);

        self
    }

    pub fn depth_limit(mut self, depth_limit: usize) -> Self {
        self.depth_limit = depth_limit;

        self
    }

    pub fn handler(mut self, handler: Handler) -> Self {
        self.handler = Some(handler);

        self
    }

    pub fn build(self) -> Dfs<'a, G, Handler> {
        Dfs::init(
            self.graph,
            self.vertices.collect_vec(),
            self.depth_limit,
            self.handler,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DfsEvent {
    Discover(usize, usize),
    TreeEdge(usize, usize),
    BackEdge(usize, usize),
    CrossEdge(usize, usize),
    ForwardEdge(usize, usize),
    Finish(usize, usize),
}

pub struct Dfs<'a, G, Handler>
where
    Handler: Fn(DfsEvent) -> bool,
    G: Vertices<Dir = Directed> + Edges<Dir = Directed>,
{
    graph: &'a G,
    vertices: Vec<usize>,
    id_map: IdMap,

    handler: Option<Handler>,

    depth_limit: usize,

    visited: Vec<bool>,

    time: usize,
    start_time: Vec<usize>,
    end_time: Vec<usize>,
}

impl<'a, G, Handler> Dfs<'a, G, Handler>
where
    Handler: Fn(DfsEvent) -> bool,
    G: Vertices<Dir = Directed> + Edges<Dir = Directed>,
{
    fn init(
        graph: &'a G,
        vertices: Vec<usize>,
        depth_limit: usize,
        handler: Option<Handler>,
    ) -> Self {
        let id_map = graph.id_map();
        let vertex_count = graph.vertex_count();

        Dfs {
            graph,
            vertices,
            depth_limit,
            handler,

            id_map,

            visited: vec![false; vertex_count],

            time: 0,
            start_time: vec![0; vertex_count],
            end_time: vec![usize::MAX; vertex_count],
        }
    }

    pub fn execute(mut self) {
        let vertices = self.vertices.clone();

        for s_rid in vertices {
            let s_vid = self.id_map.virt_of(s_rid);

            if !self.visited[s_vid] {
                if !self.traverse(s_rid, 0) {
                    return;
                }
            }
        }
    }

    fn traverse(&mut self, u_rid: usize, depth: usize) -> bool {
        if depth > self.depth_limit {
            return true;
        }

        let u_vid = self.id_map.virt_of(u_rid);

        self.visited[u_vid] = true;
        self.start_time[u_vid] = self.time;

        // Dicover
        if !self
            .handler
            .as_ref()
            .map(|h| h(DfsEvent::Discover(u_rid, self.time)))
            .unwrap_or(true)
        {
            return false;
        }

        self.time += 1;

        for v_rid in self.graph.neighbors(u_rid) {
            let v_vid = self.id_map.virt_of(v_rid);

            if !self.visited[v_vid] {
                // Tree edge
                if !self
                    .handler
                    .as_ref()
                    .map(|h| h(DfsEvent::TreeEdge(u_rid, v_rid)))
                    .unwrap_or(true)
                {
                    return false;
                }

                self.traverse(v_rid, depth + 1);
            } else {
                if self.start_time[u_vid] >= self.start_time[v_vid]
                    && self.end_time[u_vid] <= self.end_time[v_vid]
                {
                    // Back edge
                    if !self
                        .handler
                        .as_ref()
                        .map(|h| h(DfsEvent::BackEdge(u_rid, v_rid)))
                        .unwrap_or(true)
                    {
                        return false;
                    }
                } else if self.start_time[u_vid] < self.start_time[v_vid]
                    && self.end_time[u_vid] > self.end_time[v_vid]
                {
                    // Forward edge
                    if !self
                        .handler
                        .as_ref()
                        .map(|h| h(DfsEvent::ForwardEdge(u_rid, v_rid)))
                        .unwrap_or(true)
                    {
                        return false;
                    }
                } else if self.start_time[u_vid] > self.start_time[v_vid]
                    && self.end_time[u_vid] > self.end_time[v_vid]
                {
                    // Cross edge
                    if !self
                        .handler
                        .as_ref()
                        .map(|h| h(DfsEvent::CrossEdge(u_rid, v_rid)))
                        .unwrap_or(true)
                    {
                        return false;
                    }
                }
            }
        }

        self.end_time[u_vid] = self.time;

        // Finish
        if !self
            .handler
            .as_ref()
            .map(|h| h(DfsEvent::Finish(u_rid, self.time)))
            .unwrap_or(true)
        {
            return false;
        }

        self.time += 1;

        return true;
    }
}
