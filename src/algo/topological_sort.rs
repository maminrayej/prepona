use crate::provide;
use crate::traversal::Dfs;

pub fn topological_sort<G>(graph: &G) -> Vec<usize>
where
    G: provide::Vertices + provide::Neighbors,
{
    let mut sorted_vertex_ids = vec![];

    let dfs = Dfs::init(graph);

    dfs.execute_with_black_callback(graph, |virt_id| sorted_vertex_ids.push(virt_id));
    
    sorted_vertex_ids.reverse();

    sorted_vertex_ids
        .into_iter()
        .map(|virt_id| dfs.get_id_map().get_virt_to_real(virt_id).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::MatGraph;
    use crate::provide::*;
    use crate::storage::Mat;

    #[test]
    fn topological_sort_test() {
        // a  -->  b.
        let mut graph = MatGraph::init(Mat::<usize>::init(true));
        let a = graph.add_vertex();
        let b = graph.add_vertex();
        let c = graph.add_vertex();
        graph.add_edge(a, b, 1.into());
        graph.add_edge(a, c, 1.into());

        let sorted_ids = topological_sort(&graph);

        println!("{:?}", sorted_ids);
    }
}
