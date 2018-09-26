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
