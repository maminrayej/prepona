macro_rules! impl_arbitrary {
    ($provider: tt) => {
        #[cfg(test)]
        mod impl_arbitrary {
            use itertools::Itertools;
            use quickcheck::Arbitrary;
            use rand::{thread_rng, Rng};

            use $crate::provide::{
                AddEdgeProvider, AddNodeProvider, Direction, EdgeProvider, EmptyStorage,
                NodeProvider,
            };

            impl<Dir: Direction + Arbitrary> Arbitrary for super::$provider<Dir> {
                fn arbitrary(g: &mut quickcheck::Gen) -> Self {
                    let node_count = usize::arbitrary(g) % 20;

                    let mut rng = thread_rng();
                    let edge_probability = rng.gen::<f64>() * rng.gen::<f64>();

                    let mut provider = Self::init();

                    let nodes = (0..node_count)
                        .map(|node_id| {
                            provider.add_node(node_id.into());
                            node_id.into()
                        })
                        .collect_vec();

                    nodes.iter().cartesian_product(nodes.iter()).for_each(
                        |(src_node, dst_node)| {
                            let p = rng.gen::<f64>();

                            if p <= edge_probability
                                && !provider.contains_edge(*src_node, *dst_node)
                            {
                                provider.add_edge(*src_node, *dst_node);
                            }
                        },
                    );

                    provider
                }

                fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
                    let mut even_nodes = Self::init();
                    let mut odd_nodes = Self::init();

                    for (index, node) in self.nodes().enumerate() {
                        if index % 2 == 0 {
                            even_nodes.add_node(node)
                        } else {
                            odd_nodes.add_node(node)
                        };
                    }

                    for (src_node, dst_node) in self.edges() {
                        if even_nodes.contains_node(src_node) && even_nodes.contains_node(dst_node)
                        {
                            even_nodes.add_edge(src_node, dst_node);
                        } else if odd_nodes.contains_node(src_node)
                            && odd_nodes.contains_node(dst_node)
                        {
                            odd_nodes.add_edge(src_node, dst_node);
                        }
                    }

                    let before_count = self.node_count();

                    Box::new(
                        [even_nodes, odd_nodes]
                            .into_iter()
                            .filter(move |provider| provider.node_count() < before_count),
                    )
                }
            }
        }
    };
}

macro_rules! impl_test_suite {
    ($provider: tt) => {
        #[cfg(test)]
        mod impl_test_suite {
            use itertools::Itertools;
            use rand::seq::IteratorRandom;

            use $crate::provide::NodeId;
            use $crate::provide::{
                AddEdgeProvider, AddNodeProvider, DelEdgeProvider, DelNodeProvider, Directed,
                Direction, EdgeProvider, NodeProvider, ProviderError, Undirected,
            };

            fn new_node_id<P>(provider: &P) -> NodeId
            where
                P: NodeProvider,
            {
                provider.nodes().max().map_or(0.into(), |node| node + 1)
            }

            macro_rules! assert_err {
                ($result: expr, $err_type:tt::$err_kind:tt($expected_node: expr)) => {
                    if let $err_type::$err_kind(node) = $result.err().unwrap() {
                        if node != $expected_node {
                            panic!("Function returned an incorrect node as part of its error")
                        }
                    } else {
                        panic!("Function returned an incorrect error kind");
                    }
                };
                ($result: expr, $err_type:tt::$err_kind:tt($expected_node1: expr, $expected_node2: expr)) => {
                    if let $err_type::$err_kind(node1, node2) = $result.err().unwrap() {
                        if node1 != $expected_node1 || node2 != $expected_node2 {
                            panic!("Function returned an incorrect edge as part of its error")
                        }
                    } else {
                        panic!("Function returned an incorrect error kind");
                    }
                };
            }

            #[test]
            fn provider_add_node() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + EdgeProvider,
                {
                    let node_count_before = provider.node_count();
                    let edge_count_before = provider.edge_count();

                    let new_node = new_node_id(&provider);
                    provider.add_node(new_node);

                    assert_eq!(provider.node_count(), node_count_before + 1);
                    assert_eq!(provider.edge_count(), edge_count_before);
                    assert!(provider.contains_node(new_node));
                    assert!(provider.nodes().contains(&new_node));
                    assert!(provider
                        .edges()
                        .all(|(src_node, dst_node)| src_node != new_node && dst_node != new_node));
                    assert_eq!(provider.successors(new_node).count(), 0);
                    assert_eq!(provider.predecessors(new_node).count(), 0);
                    assert_eq!(provider.incoming_edges(new_node).count(), 0);
                    assert_eq!(provider.outgoing_edges(new_node).count(), 0);
                    assert_eq!(provider.in_degree(new_node), 0);
                    assert_eq!(provider.out_degree(new_node), 0);

                    for other_node in provider.nodes() {
                        assert!(!provider.is_successor(new_node, other_node));
                        assert!(!provider.is_predecessor(new_node, other_node));
                        assert!(!provider.successors(other_node).contains(&new_node));
                        assert!(!provider.predecessors(other_node).contains(&new_node));
                        assert!(!provider.outgoing_edges(other_node).contains(&new_node));
                        assert!(!provider.incoming_edges(other_node).contains(&new_node));
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_del_node_without_edge() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + DelNodeProvider + AddEdgeProvider,
                {
                    let node_count_before = provider.node_count();
                    let edge_count_before = provider.edge_count();

                    let new_node = new_node_id(&provider);
                    provider.add_node(new_node);
                    provider.del_node(new_node);

                    assert_eq!(provider.node_count(), node_count_before);
                    assert_eq!(provider.edge_count(), edge_count_before);
                    assert!(!provider.contains_node(new_node));
                    assert!(!provider.nodes().contains(&new_node));
                    assert!(provider
                        .edges()
                        .all(|(src_node, dst_node)| src_node != new_node && dst_node != new_node));

                    for other_node in provider.nodes() {
                        assert!(!provider.successors(other_node).contains(&new_node));
                        assert!(!provider.predecessors(other_node).contains(&new_node));
                        assert!(!provider.outgoing_edges(other_node).contains(&new_node));
                        assert!(!provider.incoming_edges(other_node).contains(&new_node));
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_del_node_with_edges() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + DelNodeProvider + AddEdgeProvider,
                {
                    let node_count_before = provider.node_count();
                    let edge_count_before = provider.edge_count();

                    let new_node = new_node_id(&provider);
                    provider.add_node(new_node);

                    let nodes = provider.nodes().collect_vec();
                    for (index, neighbor) in nodes.into_iter().enumerate() {
                        if index % 2 == 0 {
                            provider.add_edge(new_node, neighbor);
                        } else {
                            provider.add_edge(neighbor, new_node);
                        }
                    }

                    provider.del_node(new_node);

                    assert_eq!(provider.node_count(), node_count_before);
                    assert_eq!(provider.edge_count(), edge_count_before);
                    assert!(!provider.contains_node(new_node));
                    assert!(!provider.nodes().contains(&new_node));
                    assert!(provider
                        .edges()
                        .all(|(src_node, dst_node)| src_node != new_node && dst_node != new_node));

                    for other_node in provider.nodes() {
                        assert!(!provider.successors(other_node).contains(&new_node));
                        assert!(!provider.predecessors(other_node).contains(&new_node));
                        assert!(!provider.outgoing_edges(other_node).contains(&new_node));
                        assert!(!provider.incoming_edges(other_node).contains(&new_node));
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_add_edge() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + AddEdgeProvider,
                {
                    let node_count_before = provider.node_count();
                    let edge_count_before = provider.edge_count();

                    let src_node = new_node_id(&provider);
                    let dst_node = new_node_id(&provider) + 1;
                    provider.add_node(src_node);
                    provider.add_node(dst_node);
                    provider.add_edge(src_node, dst_node);

                    assert_eq!(provider.node_count(), node_count_before + 2);
                    assert_eq!(provider.edge_count(), edge_count_before + 1);
                    assert!(provider.contains_node(src_node));
                    assert!(provider.contains_node(dst_node));
                    assert!(provider.nodes().contains(&src_node));
                    assert!(provider.nodes().contains(&dst_node));
                    assert!(provider.is_successor(src_node, dst_node));
                    assert!(provider.is_predecessor(dst_node, src_node));
                    assert!(provider.successors(src_node).contains(&dst_node));
                    assert!(provider.predecessors(dst_node).contains(&src_node));
                    assert!(provider.contains_edge(src_node, dst_node));
                    assert!(provider.outgoing_edges(src_node).contains(&dst_node));
                    assert_eq!(provider.outgoing_edges(src_node).count(), 1);
                    assert_eq!(provider.out_degree(src_node), 1);
                    assert!(provider.incoming_edges(dst_node).contains(&src_node));
                    assert_eq!(provider.incoming_edges(dst_node).count(), 1);
                    assert_eq!(provider.in_degree(dst_node), 1);

                    if P::Dir::is_undirected() {
                        assert!(provider.is_successor(dst_node, src_node));
                        assert!(provider.is_predecessor(src_node, dst_node));
                        assert!(provider.successors(dst_node).contains(&src_node));
                        assert!(provider.predecessors(src_node).contains(&dst_node));
                        assert!(provider.outgoing_edges(dst_node).contains(&src_node));
                        assert_eq!(provider.outgoing_edges(dst_node).count(), 1);
                        assert_eq!(provider.out_degree(dst_node), 1);
                        assert!(provider.incoming_edges(src_node).contains(&dst_node));
                        assert_eq!(provider.incoming_edges(src_node).count(), 1);
                        assert_eq!(provider.in_degree(src_node), 1);
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_del_edge() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + AddEdgeProvider + DelEdgeProvider,
                {
                    let node_count_before = provider.node_count();
                    let edge_count_before = provider.edge_count();

                    let src_node = new_node_id(&provider);
                    let dst_node = new_node_id(&provider) + 1;
                    provider.add_node(src_node);
                    provider.add_node(dst_node);
                    provider.add_edge(src_node, dst_node);
                    provider.del_edge(src_node, dst_node);

                    assert_eq!(provider.node_count(), node_count_before + 2);
                    assert_eq!(provider.edge_count(), edge_count_before);
                    assert!(provider.contains_node(src_node));
                    assert!(provider.contains_node(dst_node));
                    assert!(provider.nodes().contains(&src_node));
                    assert!(provider.nodes().contains(&dst_node));
                    assert!(!provider.is_successor(src_node, dst_node));
                    assert!(!provider.is_predecessor(dst_node, src_node));
                    assert!(!provider.successors(src_node).contains(&dst_node));
                    assert!(!provider.predecessors(dst_node).contains(&src_node));
                    assert!(!provider.contains_edge(src_node, dst_node));
                    assert!(!provider.outgoing_edges(src_node).contains(&dst_node));
                    assert_eq!(provider.outgoing_edges(src_node).count(), 0);
                    assert_eq!(provider.out_degree(src_node), 0);
                    assert!(!provider.incoming_edges(dst_node).contains(&src_node));
                    assert_eq!(provider.incoming_edges(dst_node).count(), 0);
                    assert_eq!(provider.in_degree(dst_node), 0);

                    if P::Dir::is_undirected() {
                        assert!(!provider.is_successor(dst_node, src_node));
                        assert!(!provider.is_predecessor(src_node, dst_node));
                        assert!(!provider.successors(dst_node).contains(&src_node));
                        assert!(!provider.predecessors(src_node).contains(&dst_node));
                        assert!(!provider.outgoing_edges(dst_node).contains(&src_node));
                        assert_eq!(provider.outgoing_edges(dst_node).count(), 0);
                        assert_eq!(provider.out_degree(dst_node), 0);
                        assert!(!provider.incoming_edges(src_node).contains(&dst_node));
                        assert_eq!(provider.incoming_edges(src_node).count(), 0);
                        assert_eq!(provider.in_degree(src_node), 0);
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_successors_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: NodeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.successors_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_predecessors_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: NodeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.predecessors_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_is_successor_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: NodeProvider,
                {
                    if provider.node_count() != 0 {
                        let invalid_node = new_node_id(&provider);

                        let valid_node = provider.nodes().choose(&mut rand::thread_rng()).unwrap();

                        assert_err!(
                            provider.is_successor_checked(invalid_node, valid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );

                        assert_err!(
                            provider.is_successor_checked(valid_node, invalid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_is_predecessor_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: NodeProvider,
                {
                    if provider.node_count() != 0 {
                        let invalid_node = new_node_id(&provider);

                        let valid_node = provider.nodes().choose(&mut rand::thread_rng()).unwrap();

                        assert_err!(
                            provider.is_predecessor_checked(invalid_node, valid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );

                        assert_err!(
                            provider.is_predecessor_checked(valid_node, invalid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_add_node_checked() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider,
                {
                    if provider.node_count() != 0 {
                        let existing_node =
                            provider.nodes().choose(&mut rand::thread_rng()).unwrap();

                        assert_err!(
                            provider.add_node_checked(existing_node),
                            ProviderError::DuplicatedNode(existing_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_del_node_checked() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: DelNodeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.del_node_checked(invalid_node),
                        ProviderError::NodeDoesNotExist(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_contains_edge_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: EdgeProvider,
                {
                    if provider.node_count() != 0 {
                        let invalid_node = new_node_id(&provider);

                        let valid_node = provider.nodes().choose(&mut rand::thread_rng()).unwrap();

                        assert_err!(
                            provider.contains_edge_checked(valid_node, invalid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );

                        assert_err!(
                            provider.contains_edge_checked(invalid_node, valid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_incoming_edges_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: EdgeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.incoming_edges_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_outgoing_edges_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: EdgeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.outgoing_edges_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_in_degree_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: EdgeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.in_degree_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_out_degree_checked() {
                fn test<P>(provider: P) -> bool
                where
                    P: EdgeProvider,
                {
                    let invalid_node = new_node_id(&provider);

                    assert_err!(
                        provider.out_degree_checked(invalid_node),
                        ProviderError::InvalidNode(invalid_node)
                    );

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_add_edge_checked() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddEdgeProvider,
                {
                    if provider.edge_count() != 0 {
                        let (src_node, dst_node) = provider.edges().choose(&mut rand::thread_rng()).unwrap();

                        assert_err!(
                            provider.add_edge_checked(src_node, dst_node),
                            ProviderError::MultiEdge(src_node, dst_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

            #[test]
            fn provider_del_edge_checked() {
                fn test<P>(mut provider: P) -> bool
                where
                    P: AddNodeProvider + DelEdgeProvider,
                {
                    if provider.node_count() != 0 {
                        let new_node = new_node_id(&provider);
                        let invalid_node = new_node + 1;
                        let valid_node = provider.nodes().choose(&mut rand::thread_rng()).unwrap();

                        provider.add_node(new_node);

                        assert_err!(
                            provider.del_edge_checked(invalid_node, valid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );
                        assert_err!(
                            provider.del_edge_checked(valid_node, invalid_node),
                            ProviderError::InvalidNode(invalid_node)
                        );
                        assert_err!(
                            provider.del_edge_checked(new_node, valid_node),
                            ProviderError::EdgeDoesNotExist(new_node, valid_node)
                        );
                    }

                    true
                }

                quickcheck::quickcheck(test as fn(super::$provider<Directed>) -> bool);
                quickcheck::quickcheck(test as fn(super::$provider<Undirected>) -> bool);
            }

        }
    };
}

pub(crate) use impl_arbitrary;
pub(crate) use impl_test_suite;
