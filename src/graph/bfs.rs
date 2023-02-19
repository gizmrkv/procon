use std::collections::VecDeque;

use super::*;

/// Breadth First Search
#[derive(Debug, Default)]
pub struct BFS {
    root: NodeIdx,
    distance: Vec<Option<usize>>,
    through: Vec<Option<EdgeIdx>>,
    next: VecDeque<EdgeIdx>,
}

impl BFS {
    /// Get root node.
    ///
    /// **Complexity** `O(1)`
    pub fn root(&self) -> NodeIdx {
        self.root
    }

    /// Get path distance.
    ///
    /// **Complexity** `O(1)`
    pub fn distance(&self) -> &Vec<Option<usize>> {
        &self.distance
    }

    /// Get path through.
    ///
    /// **Complexity** `O(1)`
    pub fn through(&self) -> &Vec<Option<EdgeIdx>> {
        &self.through
    }

    /// Create BFS with graph and root node.
    ///
    /// **Complexity** `O(E)`
    ///
    /// # Example
    ///
    /// ```
    /// use procon::graph::*;
    ///
    /// fn main() {
    ///     let n_nodes = 4;
    ///     let edges = vec![
    ///         Edge::new(0, 1),
    ///         Edge::new(1, 2),
    ///         Edge::new(3, 0),
    ///         Edge::new(3, 1),
    ///     ];
    ///     let mut graph = DiGraph::with_nodes(n_nodes);
    ///     for edge in edges {
    ///         graph.add_edge(edge);
    ///     }
    ///
    ///     let algo = BFS::with_graph(&graph, 0);
    ///     let distance = algo.distance();
    ///     let through = algo.through();
    ///
    ///     assert_eq!(distance[0], Some(0));
    ///     assert_eq!(distance[2], Some(2));
    ///     assert_eq!(distance[3], None);
    ///     assert_eq!(through[0], None);
    ///     assert_eq!(through[2], Some(1));
    ///     assert_eq!(through[3], None);
    /// }
    /// ```
    pub fn with_graph<D>(graph: &Graph<D>, root: NodeIdx) -> Self {
        let mut algo = Self::default();
        algo.read(graph, root);
        algo
    }

    /// Run BFS with graph and root node.
    ///
    /// **Complexity** `O(E)`
    pub fn read<D>(&mut self, graph: &Graph<D>, root: NodeIdx) {
        self.distance.clear();
        self.through.clear();
        self.next.clear();

        self.root = root;
        self.distance.resize(graph.n_nodes(), None);
        self.through.resize(graph.n_nodes(), None);

        self.distance[root] = Some(0);
        for (edge, edge_idx) in graph.out_edges(root) {
            self.distance[edge.target] = Some(1);
            self.through[edge.target] = Some(*edge_idx);
            self.next.push_back(*edge_idx);
        }

        let edges = graph.edges();

        while let Some(curr) = self.next.pop_front() {
            for (edge, edge_idx) in graph.out_edges(edges[curr].target) {
                if self.distance[edge.target].is_none() {
                    self.distance[edge.target] = Some(1 + self.distance[edge.source].unwrap());
                    self.through[edge.target] = Some(*edge_idx);
                    self.next.push_back(*edge_idx);
                }
            }
        }
    }
}
