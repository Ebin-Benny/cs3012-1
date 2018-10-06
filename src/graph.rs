extern crate pathfinding;
extern crate petgraph;

use self::pathfinding::prelude::astar;
use self::petgraph::graph::NodeIndex;
use self::petgraph::Graph;
use std::collections::LinkedList;

/// Returns list of neighbors of a node with the corresponding cost.
fn neighbors<N, E>(graph: &Graph<N, E>, n: NodeIndex) -> LinkedList<(NodeIndex, u32)> {
    let mut list: LinkedList<(NodeIndex, u32)> = LinkedList::new();
    let mut neighbors = graph.neighbors(n).collect::<LinkedList<NodeIndex>>();
    for element in neighbors.iter_mut() {
        list.push_back((*element, 1));
    }
    return list;
}

/// A lowest common ancestor function for binary trees.
///
/// This function calculates the lowest common ancestor of two nodes in a graph that is structured as a binary tree.
///
/// * `graph` - Graph that the lowest common ancestor is applied on.
/// * `root`  - The root node of the binary tree.
/// * `node1` - The first node to calculate lca.
/// * `node2` - The second node to calculate lca.
pub fn lca<N, E>(
    graph: &Graph<N, E>,
    node1: NodeIndex,
    node2: NodeIndex,
) -> Option<NodeIndex> {
    let path1 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node1);
    let path2 = astar(&root, |n| neighbors(&graph, *n), |_| 0, |n| *n == node2);

    if node1 != node2 {
        let reverse1 = astar(&node1, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);
        let reverse2 = astar(&node2, |n| neighbors(&graph, *n), |_| 0, |n| *n == root);

        if reverse1.is_some() || reverse2.is_some() {
            return None;
        }
    }

    if path1.is_some() && path2.is_some() {
        let path1arr = path1.unwrap().0;
        let path2arr = path2.unwrap().0;

        let len;
        if path1arr.len() < path2arr.len() {
            len = path1arr.len();
        } else {
            len = path2arr.len();
        }

        let mut lca = root;
        for i in 0..len {
            if path1arr[i] == path2arr[i] {
                lca = path1arr[i]
            } else {
                break;
            }
        }
        return Some(lca);
    }
    return None;
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
            (n1,n2),
            (n2,n2),
            (n2,n3),
            (n2,n4),
            (n3,n5),
            (n4,n6),
            (n5,n7),
            (n6,n7),
            (n7,n8)
        ]);

        assert_eq!(true, lca(&map, n3, n2).is_some());
        assert_eq!(n2, lca(&map, n3, n2).unwrap());

        assert_eq!(true, lca(&map, n6, n3).is_some());
        assert_eq!(n2, lca(&map, n6, n3).unwrap());

        assert_eq!(true, lca(&map, n7, n5).is_some());
        assert_eq!(n5, lca(&map, n7, n5).unwrap());

        assert_eq!(true, lca(&map, n7, n4).is_some());
        assert_eq!(n4, lca(&map, n7, n4).unwrap());

        assert_eq!(true, lca(&map, n8, n4).is_some());
        assert_eq!(n4, lca(&map, n8, n4).unwrap());

        assert_eq!(true, lca(&map, n8, n2).is_some());
        assert_eq!(n2, lca(&map, n8, n2).unwrap());

        assert_eq!(true, lca(&map, n8, n5).is_some());
        assert_eq!(n5, lca(&map, n8, n5).unwrap());
    }

    /// Tests that `None` is returned when nodes are not connected.
    #[test]
    fn testlca_notconn() {
        let mut map = Graph::<&str, i32>::new();
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        let n7 = map.add_node("7");

        assert_eq!(false, lca(&map, n2, n6).is_some());

        assert_eq!(false, lca(&map, n7, n6).is_some());

        assert_eq!(false, lca(&map, n4, n5).is_some());
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

        map.extend_with_edges(&[
            (n1,n2),
            (n2,n3),
            (n2,n4),
            (n5,n6),
            (n5,n7),
            (n6,n8)
        ]);

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

    /// Tests that the function returns `None` when there is a cycle in a path but functions normally otherwise.
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

        assert_eq!(true, lca(&map, n6, n7).is_some());
        assert_eq!(n3, lca(&map, n6, n7).unwrap());

    }
}
