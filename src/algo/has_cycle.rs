// use crate::provide;
// use crate::traversal::Dfs;

// pub fn has_cycle<G, W>(graph: &G) -> bool
// where
//     G: provide::Graph<W> + provide::Neighbors + provide::Vertices,
// {
//     let mut vertices = graph.vertices();

//     let threshold = if graph.is_directed() { 0 } else { 1 };

//     while let Some(&src_index) = vertices.iter().take(1).next() {
//         let mut dfs = Dfs::init(src_index);

//         if let Some(current_index) = dfs.next(graph) {
//             let visited = dfs.get_visited();
//             let neighbors = graph.neighbors(current_index);
            
//             if neighbors.into_iter().filter(|v| visited.contains(v)).count() > threshold {
//                 return true;
//             }
//         }

//         vertices.retain(|v| !dfs.get_visited().contains(v));
//     }

//     false
// }
