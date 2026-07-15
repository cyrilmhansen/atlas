use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit::{Bfs, Dfs};

fn reachable_node_indices(graph: &UnGraph<(), ()>, source: NodeIndex) -> Vec<usize> {
    let mut traversal = Bfs::new(graph, source);
    let mut reached = Vec::new();
    while let Some(node) = traversal.next(graph) {
        reached.push(node.index());
    }
    reached.sort_unstable();
    reached
}

#[test]
fn petgraph_bfs_visits_exactly_the_reachable_component() {
    let mut graph = UnGraph::<(), ()>::new_undirected();
    let nodes: Vec<_> = (0..6).map(|_| graph.add_node(())).collect();
    graph.extend_with_edges([
        (nodes[0], nodes[1]),
        (nodes[1], nodes[2]),
        (nodes[2], nodes[0]),
        (nodes[3], nodes[4]),
    ]);

    assert_eq!(reachable_node_indices(&graph, nodes[0]), vec![0, 1, 2]);
    assert_eq!(reachable_node_indices(&graph, nodes[5]), vec![5]);
}

fn depth_first_node_indices(graph: &UnGraph<(), ()>, source: NodeIndex) -> Vec<usize> {
    let mut traversal = Dfs::new(graph, source);
    let mut reached = Vec::new();
    while let Some(node) = traversal.next(graph) {
        reached.push(node.index());
    }
    reached
}

#[test]
fn petgraph_dfs_visits_exactly_the_reachable_component_in_repeatable_preorder() {
    let mut graph = UnGraph::<(), ()>::new_undirected();
    let nodes: Vec<_> = (0..7).map(|_| graph.add_node(())).collect();
    graph.extend_with_edges([
        (nodes[0], nodes[1]),
        (nodes[1], nodes[2]),
        (nodes[2], nodes[0]),
        (nodes[2], nodes[2]),
        (nodes[0], nodes[3]),
        (nodes[4], nodes[5]),
    ]);

    let first = depth_first_node_indices(&graph, nodes[0]);
    let second = depth_first_node_indices(&graph, nodes[0]);
    assert_eq!(first, second);

    let mut reached = first;
    reached.sort_unstable();
    assert_eq!(reached, vec![0, 1, 2, 3]);
    assert_eq!(depth_first_node_indices(&graph, nodes[6]), vec![6]);
}

#[test]
fn petgraph_dfs_preorder_can_visit_a_deeper_node_before_a_shallower_node() {
    let mut graph = DiGraph::<(), ()>::new();
    let source = graph.add_node(());
    let deep_parent = graph.add_node(());
    let shallow = graph.add_node(());
    let deep_child = graph.add_node(());
    graph.add_edge(source, deep_parent, ());
    graph.add_edge(source, shallow, ());
    graph.add_edge(deep_parent, deep_child, ());

    let mut traversal = Dfs::new(&graph, source);
    let mut reached = Vec::new();
    while let Some(node) = traversal.next(&graph) {
        reached.push(node);
    }

    let deep_position = reached.iter().position(|node| *node == deep_child).unwrap();
    let shallow_position = reached.iter().position(|node| *node == shallow).unwrap();
    assert!(
        deep_position < shallow_position,
        "DFS preorder: {reached:?}"
    );
}

#[test]
fn petgraph_dijkstra_goal_none_returns_all_reachable_distances() {
    let mut graph = DiGraph::<(), u32>::new();
    let source = graph.add_node(());
    let via = graph.add_node(());
    let target = graph.add_node(());
    let end = graph.add_node(());
    let unreachable = graph.add_node(());

    graph.add_edge(source, target, 4);
    graph.add_edge(source, via, 1);
    graph.add_edge(via, target, 2);
    graph.add_edge(via, end, 5);
    graph.add_edge(target, end, 1);

    let distances = dijkstra(&graph, source, None, |edge| *edge.weight());

    assert_eq!(distances.get(&source), Some(&0));
    assert_eq!(distances.get(&via), Some(&1));
    assert_eq!(distances.get(&target), Some(&3));
    assert_eq!(distances.get(&end), Some(&4));
    assert!(!distances.contains_key(&unreachable));
}
