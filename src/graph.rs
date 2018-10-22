extern crate pathfinding;
extern crate petgraph;

use self::pathfinding::prelude::{astar, topological_sort};
use self::petgraph::{graph::NodeIndex, Direction::Incoming, Direction::Outgoing, Graph};

use std::collections::HashMap;
use std::collections::LinkedList;

/// Returns list of neighbors of a node.
fn neighbors<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex)> {
    graph
        .neighbors_directed(n, Outgoing)
        .collect::<LinkedList<NodeIndex>>()
}
/// Returns list of neighbors of a node with the corresponding cost.
fn neighbors_cost<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex, u32)> {
    let mut list: LinkedList<(NodeIndex, u32)> = LinkedList::new();
    let mut neighbors = graph
        .neighbors_directed(n, Outgoing)
        .collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        list.push_back((*element, 1));
    }
    return list;
}

/// Puts the ancestors of `node` onto a HashMap
fn ancestors<N, E>(graph: &Graph<N, E>, node: NodeIndex) -> HashMap<NodeIndex, bool> {
    let ancestors = HashMap::<NodeIndex, bool>::new();
    return add_ancestors(graph, ancestors, node);
}

/// Puts the ancestors of `node` onto the HashMap `ancestors`
fn add_ancestors<N, E>(
    graph: &Graph<N, E>,
    mut ancestors: HashMap<NodeIndex, bool>,
    node: NodeIndex,
) -> HashMap<NodeIndex, bool> {
    ancestors.insert(node, true);
    let mut neighbors = graph
        .neighbors_directed(node, Incoming)
        .collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        ancestors = add_ancestors(graph, ancestors, *element);
    }
    return ancestors;
}

/// Compares ancestors of `node` with `ancestors` and returns the lowest common ancestor.
fn compare_ancestors<N, E>(
    graph: &Graph<N, E>,
    ancestors: &HashMap<NodeIndex, bool>,
    node: NodeIndex,
) -> Option<NodeIndex> {
    return compare_ancestors_node(graph, ancestors, node, 0).0;
}

/// Compares ancestors of `node` with `ancestors` and returns the lowest common ancestor along with the cost.
fn compare_ancestors_node<N, E>(
    graph: &Graph<N, E>,
    ancestors: &HashMap<NodeIndex, bool>,
    node: NodeIndex,
    cost: i32,
) -> (Option<NodeIndex>, i32) {
    if ancestors.contains_key(&node) {
        return (Some(node), cost);
    }
    let mut neighbors = graph
        .neighbors_directed(node, Incoming)
        .collect::<LinkedList<NodeIndex>>();
    let mut lca_cost = (None, <i32>::max_value());
    for element in neighbors.iter_mut() {
        let ancestor_cost = compare_ancestors_node(&graph, ancestors, *element, cost + 1);
        if ancestor_cost.0.is_some() && ancestor_cost.1 < lca_cost.1 {
            lca_cost = ancestor_cost;
        }
    }
    return lca_cost;
}

/// Check if there is a cycle in the graph.
fn check_cycle<N, E>(graph: &Graph<N, E>, node: NodeIndex) -> bool {
    let mut neighbors = graph
        .neighbors_directed(node, Incoming)
        .collect::<LinkedList<NodeIndex>>();
    let mut visited = HashMap::<NodeIndex, bool>::new();
    visited.insert(node, true);
    for element in neighbors.iter_mut() {
        if check_cycle_node(graph, visited.clone(), *element) {
            return true;
        }
    }
    return false;
}

/// Check if there is a cycle in the graph by checking if a node is visited multiple times.
fn check_cycle_node<N, E>(
    graph: &Graph<N, E>,
    mut visited: HashMap<NodeIndex, bool>,
    node: NodeIndex,
) -> bool {
    if visited.contains_key(&node) {
        return true;
    }
    visited.insert(node, true);
    let mut neighbors = graph
        .neighbors_directed(node, Incoming)
        .collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        if check_cycle_node(graph, visited.clone(), *element) {
            return true;
        }
    }
    return false;
}

/// A lowest common ancestor function for binary trees.
///
/// This function calculates the lowest common ancestor of two nodes in a graph that is structured as a binary tree.
///
/// * `graph` - Graph that the lowest common ancestor is applied on.
/// * `root`  - The root node of the binary tree.
/// * `node1` - The first node to calculate lca.
/// * `node2` - The second node to calculate lca.
pub fn lca<N, E>(graph: &Graph<N, E>, node1: NodeIndex, node2: NodeIndex) -> Option<NodeIndex> {
    if check_cycle(graph, node1) || check_cycle(graph, node2) {
        return None;
    }
    let ancestors = ancestors(graph, node1);
    return compare_ancestors(graph, &ancestors, node2);
}

#[cfg(test)]
mod tests {
    use super::lca;
    use super::Graph;

    /// Tests normal operations of lca on a connected graph structured as an directed acyclic graph.
    #[test]
    fn testlca_normal() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");
        let n8 = map.add_node("8");

        map.extend_with_edges(&[
            (n1, n2),
            (n2, n3),
            (n2, n4),
            (n3, n5),
            (n4, n6),
            (n5, n7),
            (n6, n7),
            (n7, n8),
        ]);
        assert_eq!(true, lca(&map, n8, n4).is_some());
        assert_eq!(n4, lca(&map, n8, n4).unwrap());

        assert_eq!(true, lca(&map, n8, n2).is_some());
        assert_eq!(n2, lca(&map, n8, n2).unwrap());

        assert_eq!(true, lca(&map, n8, n5).is_some());
        assert_eq!(n5, lca(&map, n8, n5).unwrap());
    }

    /// Tests normal operations of lca on a connected graph structured as an directed acyclic graph.
    #[test]
    fn testlca_merge() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");
        let n8 = map.add_node("8");

        map.extend_with_edges(&[
            (n1, n2),
            (n2, n3),
            (n2, n4),
            (n3, n5),
            (n4, n6),
            (n5, n7),
            (n6, n7),
            (n7, n8),
        ]);

        assert_eq!(true, lca(&map, n8, n4).is_some());
        assert_eq!(n4, lca(&map, n8, n4).unwrap());

        assert_eq!(true, lca(&map, n8, n2).is_some());
        assert_eq!(n2, lca(&map, n8, n2).unwrap());

        assert_eq!(true, lca(&map, n8, n5).is_some());
        assert_eq!(n5, lca(&map, n8, n5).unwrap());

        assert_eq!(true, lca(&map, n3, n7).is_some());
        assert_eq!(n3, lca(&map, n3, n7).unwrap());
    }

    /// Tests that `None` is returned when nodes are not connected.
    #[test]
    fn testlca_notconn() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");

        assert_eq!(false, lca(&map, n1, n2).is_some());

        assert_eq!(false, lca(&map, n3, n4).is_some());

        assert_eq!(false, lca(&map, n5, n6).is_some());
    }

    /// Tests that the correct node is returned when there are separate connected graphs, also tests if `None` is returned when Nodes are not connected.
    #[test]
    fn testlca_separate() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");
        let n8 = map.add_node("8");

        map.extend_with_edges(&[(n1, n2), (n2, n3), (n2, n4), (n5, n6), (n5, n7), (n6, n8)]);

        assert_eq!(true, lca(&map, n2, n4).is_some());
        assert_eq!(n2, lca(&map, n2, n4).unwrap());

        assert_eq!(true, lca(&map, n7, n8).is_some());
        assert_eq!(n5, lca(&map, n7, n8).unwrap());

        assert_eq!(false, lca(&map, n4, n6).is_some());
    }

    /// Tests that the same node is returned for when `node1` and `node2` are the same.
    #[test]
    fn testlca_samenode() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("root");

        assert_eq!(true, lca(&map, n1, n1).is_some());
        assert_eq!(n1, lca(&map, n1, n1).unwrap());
    }

    /// Tests that the function returns `None` when there is a cycle in a path.
    #[test]
    fn testlca_structure() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");
        map.extend_with_edges(&[
            (n1, n2),
            (n1, n3),
            (n5, n1),
            (n2, n4),
            (n2, n5),
            (n3, n6),
            (n3, n7),
        ]);
        assert_eq!(false, lca(&map, n2, n6).is_some());
        assert_eq!(false, lca(&map, n6, n7).is_some());
    }
}
