/// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C&lang=ja
use procon::{graph::*, *};

fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    read_lines!(reader, 1, n: usize);
    let mut graph = DiGraph::with_nodes(n);
    for _ in 0..n {
        read_lines!(reader, 1, u: usize, k: usize, v: [usize; k]);
        for w in v {
            graph.add_edge(Edge::new(u - 1, w - 1));
        }
    }

    let algo = BFS::with_graph(&graph, 0);
    for (node, dist) in algo.distance().iter().enumerate() {
        if let Some(dist) = dist {
            writeln!(writer, "{} {}", node + 1, dist)?;
        } else {
            writeln!(writer, "{} {}", node + 1, -1)?;
        }
    }

    writer.flush()?;
    Ok(())
}
