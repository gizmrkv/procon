pub type NodeIdx = usize;
pub type EdgeIdx = usize;

/// Graph's edge.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    source: NodeIdx,
    target: NodeIdx,
}

impl Edge {
    /// Create edge with source node, target node and edge index.
    ///
    /// **Complexity** `O(1)`
    pub fn new(source: NodeIdx, target: NodeIdx) -> Self {
        Self { source, target }
    }

    /// Reverse edge.
    ///
    /// **Complexity** `O(1)`
    pub fn reverse(&self) -> Self {
        Edge::new(self.target, self.source)
    }
}

/// Graph's node.
#[derive(Debug, Default, Clone)]
pub struct Node {
    in_edges: Vec<(Edge, EdgeIdx)>,
    out_edges: Vec<(Edge, EdgeIdx)>,
}

/// Marker indicating that the graph is directed.
#[derive(Debug)]
pub struct Directed {}

/// Marker indicating that the graph is not directed.
#[derive(Debug)]
pub struct Undirected {}

/// Graph structure.
#[derive(Debug, Default, Clone)]
pub struct Graph<D> {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    d: std::marker::PhantomData<D>,
}

/// Directed graph.
///
/// # Example
///
/// ```
/// use procon::graph::*;
///
/// fn main() {
///     let mut graph = DiGraph::with_nodes(4);
///     graph.add_edge(Edge::new(0, 1));
///     graph.add_edge(Edge::new(0, 3));
///     let mut iter = graph.out_edges(0);
///     assert_eq!(iter.next().cloned(), Some((Edge::new(0, 1), 0)));
///     assert_eq!(iter.next().cloned(), Some((Edge::new(0, 3), 1)));
/// }
/// ```
pub type DiGraph = Graph<Directed>;

/// Undirected graph.
///
/// # Example
///
/// ```
/// use procon::graph::*;
///
/// fn main() {
///     let mut graph = UnGraph::with_nodes(4);
///     graph.add_edge(Edge::new(0, 1));
///     graph.add_edge(Edge::new(0, 3));
///     let mut iter = graph.out_edges(0);
///     assert_eq!(iter.next().cloned(), Some((Edge::new(0, 1), 0)));
///     assert_eq!(iter.next().cloned(), Some((Edge::new(0, 3), 1)));
/// }
/// ```
pub type UnGraph = Graph<Undirected>;

impl<D> Graph<D> {
    /// Create the graph.
    ///
    /// **Complexity** `O(1)`
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            d: std::marker::PhantomData,
        }
    }

    /// Create the graph with the number of nodes and edges.
    ///
    /// **Complexity** `O(1)`
    pub fn with_nodes(n_nodes: usize) -> Self {
        Self {
            nodes: vec![Default::default(); n_nodes],
            edges: Vec::new(),
            d: std::marker::PhantomData,
        }
    }

    /// Iterate over the out-edges.
    ///
    /// **Complexity** `O(1)`
    pub fn out_edges(&self, node: NodeIdx) -> std::slice::Iter<'_, (Edge, EdgeIdx)> {
        self.nodes[node].out_edges.iter()
    }

    /// Iterate over the in-edges.
    ///
    /// **Complexity** `O(1)`
    pub fn in_edges(&self, node: NodeIdx) -> std::slice::Iter<'_, (Edge, EdgeIdx)> {
        self.nodes[node].in_edges.iter()
    }
}

impl DiGraph {
    /// Add the directed edge.
    ///
    /// **Complexity** `O(1)`
    pub fn add_edge(&mut self, edge: Edge) {
        let edge_idx = self.edges.len();
        self.nodes[edge.source].out_edges.push((edge, edge_idx));
        self.nodes[edge.target].in_edges.push((edge, edge_idx));
        self.edges.push(edge);
    }
}

impl UnGraph {
    /// Add the undirected edge.
    ///
    /// **Complexity** `O(1)`
    pub fn add_edge(&mut self, edge: Edge) {
        let edge_idx = self.edges.len();

        self.nodes[edge.source].out_edges.push((edge, edge_idx));
        self.nodes[edge.source]
            .in_edges
            .push((edge.reverse(), edge_idx));

        self.nodes[edge.target].in_edges.push((edge, edge_idx));
        self.nodes[edge.target]
            .out_edges
            .push((edge.reverse(), edge_idx));

        self.edges.push(edge);
    }
}
