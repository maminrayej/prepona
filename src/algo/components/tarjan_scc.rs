use crate::algo::traversal::dfs::{ControlFlow, EdgeType, Event, DFS};
use crate::provide::{Directed, EdgeProvider, NodeId, NodeIdMapProvider};

pub struct TarjanScc<'a, G> {
    graph: &'a G,

    low_id: Vec<usize>,
    id_of: Vec<usize>,
    stack: Vec<NodeId>,
}

impl<'a, G> TarjanScc<'a, G>
where
    G: EdgeProvider<Dir = Directed> + NodeIdMapProvider,
{
    pub fn new(graph: &'a G) -> Self {
        TarjanScc {
            graph,
            low_id: vec![0; graph.node_count()],
            id_of: vec![0; graph.node_count()],
            stack: vec![],
        }
    }

    pub fn execute(&mut self) -> Vec<Vec<NodeId>> {
        let mut id = 1;
        let mut components = vec![];

        DFS::init(self.graph).execute(|event| {
            match event {
                Event::Discover(state, v_rid) => {
                    let v_vid = state.id_map[v_rid];

                    self.id_of[v_vid] = id;
                    self.low_id[v_vid] = id;

                    id += 1;
                    self.stack.push(v_rid);
                }
                Event::Finish(state, v_rid) => {
                    let v_vid = state.id_map[v_rid];
                    let v_low_id = self.low_id[v_vid];
                    let v_id = self.id_of[v_vid];

                    if v_id == v_low_id {
                        let mut cc = vec![];
                        while let Some(rid) = self.stack.pop() {
                            cc.push(rid);

                            if rid == v_rid {
                                break;
                            }
                        }

                        components.push(cc);
                    } else if let Some(&p_vid) = state.parent.get(v_vid) {
                        let p_low_id = self.low_id[p_vid];

                        self.low_id[p_vid] = std::cmp::min(v_low_id, p_low_id);
                    }
                }
                Event::VisitEdge(state, EdgeType::BackEdge, src_rid, dst_rid) => {
                    let s_vid = state.id_map[src_rid];
                    let d_vid = state.id_map[dst_rid];

                    let s_low_id = self.low_id[s_vid];
                    let d_low_id = self.low_id[d_vid];

                    self.low_id[s_vid] = std::cmp::min(s_low_id, d_low_id);
                }
                _ => {}
            }

            ControlFlow::Continue
        });

        components
    }
}
