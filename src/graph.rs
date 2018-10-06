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
pub fn lca_btree<N, E>(
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

        let mut lca_btree = root;
        for i in 0..len {
            if path1arr[i] == path2arr[i] {
                lca_btree = path1arr[i]
            } else {
                break;
            }
        }
        return Some(lca_btree);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::lca_btree;
    use super::Graph;

    /// Tests normal operations of lca_btree on a connected graph structured as a binary tree.
    #[test]
    fn testlca_btree() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        map.extend_with_edges(&[
            (root, n1),
            (root, n2),
            (n1, n3),
            (n1, n4),
            (n2, n5),
            (n2, n6),
        ]);
        assert_eq!(true, lca_btree(&map, n1, n5).is_some());
        assert_eq!(root, lca_btree(&map, n1, n5).unwrap());

        assert_eq!(true, lca_btree(&map, n6, n5).is_some());
        assert_eq!(n2, lca_btree(&map, n6, n5).unwrap());

        assert_eq!(true, lca_btree(&map, n3, n4).is_some());
        assert_eq!(n1, lca_btree(&map, n3, n4).unwrap());
    }

    /// Tests that `None` is returned when nodes are not connected.
    #[test]
    fn testlca_notconn() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");
        let n1 = map.add_node("1");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");

        assert_eq!(false, lca_btree(&map, n1, n5).is_some());

        assert_eq!(false, lca_btree(&map, n6, n5).is_some());

        assert_eq!(false, lca_btree(&map, n3, n4).is_some());
    }

    /// Tests that the same node is returned for when `node1` and `node2` are the same.
    #[test]
    fn testlca_samenode() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");

        assert_eq!(true, lca_btree(&map, root, root).is_some());
        assert_eq!(root, lca_btree(&map, root, root).unwrap());
    }

    /// Tests that the function returns `None` when there is a cycle in the graph.
    #[test]
    fn testlca_structure() {
        let mut map = Graph::<&str, i32>::new();
        let root = map.add_node("root");
        let n1 = map.add_node("1");
        let n2 = map.add_node("2");
        let n3 = map.add_node("3");
        let n4 = map.add_node("4");
        let n5 = map.add_node("5");
        let n6 = map.add_node("6");
        map.extend_with_edges(&[
            (root, n1),
            (root, n2),
            (n5, root),
            (n4, root),
            (n1, n3),
            (n1, n4),
            (n2, n5),
            (n2, n6),
        ]);
        assert_eq!(false, lca_btree(&map, n1, n5).is_some());
    }
}
