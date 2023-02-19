/// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=NTL_1_E&lang=ja
use procon::{math::*, *};

fn main() -> std::io::Result<()> {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    read_to_end!(reader, a: i64, b: i64);
    let (x, y) = extgcd(a, b);

    writeln!(writer, "{} {}", x, y)?;
    writer.flush()?;
    Ok(())
}
