extern crate pathfinding;
extern crate petgraph;

use self::petgraph::graph::NodeIndex;
use self::petgraph::Graph;

pub fn lca<N, E>(
    graph: &Graph<N, E>,
    root: NodeIndex,
    node1: NodeIndex,
    node2: NodeIndex,
) -> Option<NodeIndex> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::lca;
    use super::Graph;

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
        assert_eq!(true, lca(&map, root, n1, n5).is_some());
        assert_eq!(root, lca(&map, root, n1, n5).unwrap());

        assert_eq!(true, lca(&map, root, n6, n5).is_some());
        assert_eq!(n2, lca(&map, root, n6, n5).unwrap());

        assert_eq!(true, lca(&map, root, n3, n4).is_some());
        assert_eq!(n1, lca(&map, root, n3, n4).unwrap());
    }
}
