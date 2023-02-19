use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

use super::*;
use crate::TotalOrd;

/// Dijkstra algorithm.
#[derive(Debug, Default)]
pub struct Dijkstra<T: PartialEq + PartialOrd> {
    root: NodeIdx,
    distance: Vec<Option<T>>,
    through: Vec<Option<EdgeIdx>>,
    next: BinaryHeap<(Reverse<TotalOrd<T>>, EdgeIdx)>,
}

impl<T: Default + PartialEq + PartialOrd + Clone + Copy + From<i32> + Add<Output = T>> Dijkstra<T> {
    /// Get root node.
    ///
    /// **Complexity** `O(E)`
    pub fn root(&self) -> NodeIdx {
        self.root
    }

    /// Get path distance.
    ///
    /// **Complexity** `O(E)`
    pub fn distance(&self) -> &Vec<Option<T>> {
        &self.distance
    }

    /// Get path through.
    ///
    /// **Complexity** `O(E)`
    pub fn through(&self) -> &Vec<Option<EdgeIdx>> {
        &self.through
    }

    /// Create Dijkstra with graph and root node.
    ///
    /// **Complexity** `O((E+V)logV)`
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
    ///     let algo = Dijkstra::with_graph(&graph, 0, |_| 1);
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
    pub fn with_graph<D, F: Fn(EdgeIdx) -> T>(
        graph: &Graph<D>,
        root: NodeIdx,
        edge_dist: F,
    ) -> Self {
        let mut algo = Self::default();
        algo.read(graph, root, edge_dist);
        algo
    }

    /// Run Dijkstra with graph and root node.
    ///
    /// **Complexity** `O((E+V)logV)`
    pub fn read<D, F: Fn(EdgeIdx) -> T>(
        &mut self,
        graph: &Graph<D>,
        root: NodeIdx,
        edge_distance: F,
    ) {
        self.distance.clear();
        self.through.clear();
        self.next.clear();

        self.root = root;
        self.distance.resize(graph.n_nodes(), None);
        self.through.resize(graph.n_nodes(), None);

        self.distance[root] = Some(T::from(0));
        for (edge, edge_idx) in graph.out_edges(root) {
            let dist = edge_distance(*edge_idx);
            self.distance[edge.target] = Some(dist);
            self.through[edge.target] = Some(*edge_idx);
            self.next.push((Reverse(TotalOrd(dist)), *edge_idx));
        }

        let edges = graph.edges();
        while let Some((Reverse(TotalOrd(dist)), curr)) = self.next.pop() {
            if let Some(prev_dist) = self.distance[edges[curr].target] {
                if prev_dist < dist {
                    continue;
                }
            }
            for (next, next_idx) in graph.out_edges(edges[curr].target) {
                let next_dist = dist + edge_distance(*next_idx);
                if let Some(prev_dist) = self.distance[next.target] {
                    if prev_dist <= next_dist {
                        continue;
                    }
                }
                self.distance[next.target] = Some(next_dist);
                self.through[next.target] = Some(*next_idx);
                self.next.push((Reverse(TotalOrd(next_dist)), *next_idx));
            }
        }
    }
}
