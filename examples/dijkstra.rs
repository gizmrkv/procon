/// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A&lang=ja
use procon::{graph::*, *};

fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    read_to_end!(
        reader,
        n_nodes: usize,
        n_edges: usize,
        root: usize,
        edges: [(usize, usize, i32); n_edges]
    );

    let dist: Vec<_> = edges.iter().map(|(_, _, d)| *d).collect();
    let edges: Vec<_> = edges.iter().map(|(s, t, _)| Edge::new(*s, *t)).collect();
    let graph = DiGraph::with_edges(n_nodes, edges.iter().cloned());

    let algo = Dijkstra::with_graph(&graph, root, |id| dist[id]);
    for (_, d) in algo.distance().iter().enumerate() {
        if let Some(d) = d {
            writeln!(writer, "{}", d)?;
        } else {
            writeln!(writer, "INF")?;
        }
    }

    writer.flush()?;
    Ok(())
}
