use std::collections::HashSet;
use std::marker::PhantomData;

use crate::graph::{DefaultEdge, DirectedEdge, Edge, EdgeType, FlowEdge, UndirectedEdge};
use crate::storage::GraphStorage;

/// An `AdjList` that stores edges of type `DefaultEdge`.
pub type List<W> = AdjList<W, DefaultEdge<W>, UndirectedEdge>;
pub type DiList<W> = AdjList<W, DefaultEdge<W>, DirectedEdge>;

/// An `AdjList` that stores edges of type `FlowEdge`.
pub type FlowList<W> = AdjList<W, FlowEdge<W>, UndirectedEdge>;
pub type DiFlowList<W> = AdjList<W, FlowEdge<W>, DirectedEdge>;

pub struct AdjList<W, E: Edge<W>, Ty: EdgeType> {
    edges_of: Vec<Vec<E>>,
    reusable_ids: HashSet<usize>,

    vertex_count: usize,
    is_directed: bool,

    phantom_w: PhantomData<W>,
    phantom_ty: PhantomData<Ty>,
}

impl<W, E: Edge<W>, Ty: EdgeType> AdjList<W, E, Ty> {
    pub fn init() -> Self {
        AdjList {
            edges_of: vec![],
            reusable_ids: HashSet::new(),

            vertex_count: 0,
            is_directed: Ty::is_directed(),

            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }

    fn next_reusable_id(&mut self) -> Option<usize> {
        if let Some(id) = self.reusable_ids.iter().take(1).next().copied() {
            self.reusable_ids.remove(&id);

            Some(id)
        } else {
            None
        }
    }

    fn validate_id(&self, vertex_id: usize) {
        if self.reusable_ids.contains(&vertex_id) || vertex_id >= self.edges_of.len() {
            panic!(format!(
                "Vertex with id: {} is not present in the graph",
                vertex_id
            ))
        }
    }
}

impl<W: Copy, E: Edge<W>, Ty: EdgeType> GraphStorage<W, E, Ty> for AdjList<W, E, Ty> {
    fn add_vertex(&mut self) -> usize {
        self.vertex_count += 1;

        if let Some(reusable_id) = self.next_reusable_id() {
            reusable_id
        } else {
            self.edges_of.push(vec![]);

            self.edges_of.len() - 1
        }
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.validate_id(vertex_id);

        self.edges_of[vertex_id].clear();

        for src_id in 0..self.vertex_count() {
            self.edges_of[src_id].retain(|edge| edge.get_dst_id() != vertex_id)
        }

        self.vertex_count -= 1;

        self.reusable_ids.insert(vertex_id);
    }

    fn add_edge(&mut self, edge: E) {
        let (src_id, dst_id) = (edge.get_src_id(), edge.get_dst_id());

        self.validate_id(src_id);
        self.validate_id(dst_id);

        let weight = *edge.get_weight();

        self.edges_of[src_id].push(edge);

        if self.is_undirected() {
            self.edges_of[dst_id].push(E::init(dst_id, src_id, weight));
        }
    }

    fn update_edge(&mut self, edge: E) {
        let (src_id, dst_id) = (edge.get_src_id(), edge.get_dst_id());

        self.remove_edge(src_id, dst_id);

        self.add_edge(edge);
    }

    fn remove_edge(&mut self, src_id: usize, dst_id: usize) -> E {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        if let Some((index, _)) = self.edges_of[src_id]
            .iter()
            .enumerate()
            .find(|(_, edge)| edge.get_dst_id() == dst_id)
        {
            if self.is_undirected() {
                self.edges_of[dst_id].retain(|edge| edge.get_dst_id() != src_id);
            }

            self.edges_of[src_id].remove(index)
        } else {
            panic!(
                "There is no edge from vertex: {} to vertex: {}",
                src_id, dst_id
            )
        }
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.edges_of.len())
            .filter(|vertex_id| !self.reusable_ids.contains(vertex_id))
            .collect()
    }

    fn edge(&self, src_id: usize, dst_id: usize) -> Option<&E> {
        self.validate_id(src_id);
        self.validate_id(dst_id);

        self.edges_of[src_id]
            .iter()
            .find(|edge| edge.get_dst_id() == dst_id)
    }

    fn edges(&self) -> Vec<&E> {
        self.vertices()
            .into_iter()
            .flat_map(|src_id| self.edges_from(src_id).into_iter())
            .collect()
    }

    fn edges_from(&self, src_id: usize) -> Vec<&E> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .filter(|edge| edge.get_weight().is_finite())
            .collect()
    }

    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.validate_id(src_id);

        self.edges_of[src_id]
            .iter()
            .map(|edge| edge.get_dst_id())
            .collect()
    }

    fn is_directed(&self) -> bool {
        self.is_directed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directed_empty_list() {
        // Given: An empty directed list.
        let list = DiList::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 0);
        assert_eq!(list.edges_of.len(), 0);
        assert_eq!(list.reusable_ids.len(), 0);
        assert_eq!(list.is_directed(), true);
    }

    #[test]
    fn undirected_empty_list() {
        // Given: An empty undirected list.
        let list = List::<usize>::init();

        // When: Doing nothing.

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 0);
        assert_eq!(list.edges_of.len(), 0);
        assert_eq!(list.reusable_ids.len(), 0);
        assert_eq!(list.is_directed(), false);
    }

    #[test]
    fn directed_add_vertex() {
        // Given: An empty directed list.
        let mut list = DiList::<usize>::init();

        // When: Adding 3 vertices.
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.is_empty()));
        assert_eq!(list.reusable_ids.len(), 0);

        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| list.edge(src_id, dst_id).is_none()));
    }

    #[test]
    fn undirected_add_vertex() {
        // Given: An empty undirected list.
        let mut list = List::<usize>::init();

        // When: Adding 3 vertices.
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert_eq!(list.reusable_ids.len(), 0);

        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| list.edge(src_id, dst_id).is_none()));
    }

    #[test]
    fn directed_delete_vertex() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b.
        list.remove_vertex(a);
        list.remove_vertex(b);

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 1);
        assert_eq!(list.edges_of.len(), 3);

        // Vertices a and b must be reusable.
        assert_eq!(list.reusable_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.reusable_ids.contains(vertex_id)));

        // list must only contain c.
        assert_eq!(list.vertices().len(), 1);
        assert!(list.vertices().contains(&c));

        assert!(list.edge(c, c).is_none());
    }

    #[test]
    fn undirected_delete_vertex() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b.
        list.remove_vertex(a);
        list.remove_vertex(b);

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 1);
        assert_eq!(list.edges_of.len(), 3);

        // Vertices a and b must be reusable.
        assert_eq!(list.reusable_ids.len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.reusable_ids.contains(vertex_id)));

        // list must only contain c.
        assert_eq!(list.vertices().len(), 1);
        assert!(list.vertices().contains(&c));

        assert!(list.edge(c, c).is_none());
    }

    #[test]
    fn directed_add_vertex_after_vertex_deletion() {
        // Given: Directed graph
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        list.remove_vertex(a);
        list.remove_vertex(b);
        let _ = list.add_vertex();
        let _ = list.add_vertex();

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);

        // There must be no reusable id.
        assert_eq!(list.reusable_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| list.edge(src_id, dst_id).is_none()));
    }

    #[test]
    fn undirected_add_vertex_after_vertex_deletion() {
        // Given: Undirected graph
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Removing vertices a and b and afterwards adding two new vertices.
        list.remove_vertex(a);
        list.remove_vertex(b);
        let _ = list.add_vertex();
        let _ = list.add_vertex();

        // Then:
        assert_eq!(list.edges().len(), 0);
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);

        // There must be no reusable id.
        assert_eq!(list.reusable_ids.len(), 0);

        // Vertex ids a and b must be reused.
        assert_eq!(list.vertices().len(), 3);
        assert!(vec![a, b, c]
            .iter()
            .all(|vertex_id| list.vertices().contains(vertex_id)));

        // Edge weight between any two vertices must be positive infinity.
        // Also edge src and dst must be set correctly.
        assert!(vec![a, b, c]
            .into_iter()
            .flat_map(|vertex_id| vec![(vertex_id, a), (vertex_id, b), (vertex_id, c)])
            .all(|(src_id, dst_id)| list.edge(src_id, dst_id).is_none()));
    }

    #[test]
    fn directed_add_edge() {
        // Given: Directed list
        //
        //      a   b   c
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Adding edges
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 1));

        assert_eq!(list.edges().len(), 3);
        for edge in list.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
                (0, 1) => assert_eq!(edge.get_weight().unwrap(), 1),
                (1, 2) => assert_eq!(edge.get_weight().unwrap(), 2),
                (2, 0) => assert_eq!(edge.get_weight().unwrap(), 3),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn undirected_add_edge() {
        // Given: Undirected list
        //
        //      a   b   c
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();

        // When: Adding edges
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 2));

        assert_eq!(list.edges().len(), 6);
        for edge in list.edges() {
            match (edge.get_src_id(), edge.get_dst_id()) {
                (0, 1) | (1, 0) => assert_eq!(edge.get_weight().unwrap(), 1),
                (1, 2) | (2, 1) => assert_eq!(edge.get_weight().unwrap(), 2),
                (2, 0) | (0, 2) => assert_eq!(edge.get_weight().unwrap(), 3),
                _ => panic!("Unknown vertex id"),
            }
        }
    }

    #[test]
    fn directed_remove_edge() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // When: Removing edges a --> b and b --> c
        //
        //      a   b   c
        //      ^       |
        //      '--------
        //
        list.remove_edge(a, b);
        list.remove_edge(b, c);

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of[a].is_empty());
        assert!(list.edges_of[b].is_empty());
        assert_eq!(list.edges_of[c].len(), 1);

        assert_eq!(list.edges().len(), 1);
        assert_eq!(list.edge(c, a).unwrap().get_weight().unwrap(), 3);
    }

    #[test]
    fn undirected_remove_edge() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // When: Removing edges a --- b and b --- c
        //
        //      a   b   c
        //      |       |
        //      '--------
        //
        list.remove_edge(a, b);
        list.remove_edge(b, c);

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of[a].len(), 1);
        assert!(list.edges_of[b].is_empty());
        assert_eq!(list.edges_of[c].len(), 1);

        assert_eq!(list.edges().len(), 2);
        assert_eq!(list.edge(a, c).unwrap().get_weight().unwrap(), 3);
    }

    #[test]
    fn directed_neighbors() {
        // Given: Directed list
        //
        //      a  -->  b  -->  c
        //      ^               |
        //      '----------------
        //
        let mut list = DiList::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 1));

        assert_eq!(list.neighbors(a).len(), 1);
        assert_eq!(*list.neighbors(a).get(0).unwrap(), b);

        assert_eq!(list.neighbors(b).len(), 1);
        assert_eq!(*list.neighbors(b).get(0).unwrap(), c);

        assert_eq!(list.neighbors(c).len(), 1);
        assert_eq!(*list.neighbors(c).get(0).unwrap(), a);
    }

    #[test]
    fn undirected_neighbors() {
        // Given: Undirected list
        //
        //      a  ---  b  ---  c
        //      |               |
        //      '----------------
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();
        let c = list.add_vertex();
        list.add_edge((a, b, 1).into());
        list.add_edge((b, c, 2).into());
        list.add_edge((c, a, 3).into());

        // When: Doing nothing.

        // Then:
        assert_eq!(list.vertex_count(), 3);
        assert_eq!(list.edges_of.len(), 3);
        assert!(list.edges_of.iter().all(|edges| edges.len() == 2));

        assert_eq!(list.neighbors(a).len(), 2);
        assert!(vec![b, c]
            .iter()
            .all(|vertex_id| list.neighbors(a).contains(vertex_id)));

        assert_eq!(list.neighbors(b).len(), 2);
        assert!(vec![a, c]
            .iter()
            .all(|vertex_id| list.neighbors(b).contains(vertex_id)));

        assert_eq!(list.neighbors(c).len(), 2);
        assert!(vec![a, b]
            .iter()
            .all(|vertex_id| list.neighbors(c).contains(vertex_id)));
    }

    #[test]
    #[should_panic(expected = "Vertex with id: 0 is not present in the graph")]
    fn first_vertex_not_present() {
        // Given: list
        //
        //      a
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();

        // When: Removing vertex a and try to pass it as valid vertex id.
        list.remove_vertex(a);
        list.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Vertex with id: 1 is not present in the graph")]
    fn second_vertex_not_present() {
        // Given: list
        //
        //      a
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();

        // When: Removing vertex b and try to pass it as valid vertex id.
        list.remove_vertex(b);
        list.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "Vertex with id: 3 is not present in the graph")]
    fn non_existent_vertex() {
        // Given: list
        //
        //      a
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = 3;

        // When: Trying to access b which is never add to graph.
        list.edge(a, b);

        // Then: Code should panic.
    }

    #[test]
    #[should_panic(expected = "There is no edge from vertex: 0 to vertex: 1")]
    fn non_existent_edge() {
        // Given: List
        //
        //      a   b
        //
        let mut list = List::<usize>::init();
        let a = list.add_vertex();
        let b = list.add_vertex();

        // When: Trying to remove non existent edge between a and b.
        list.remove_edge(a, b);

        // Then: Code should panic.
    }
}
