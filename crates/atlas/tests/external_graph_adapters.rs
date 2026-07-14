use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit::Bfs;

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
