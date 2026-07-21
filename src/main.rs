use std::fs;
use std::io::Write;

// A genuine BATCH workload: it does its work, writes a data artifact that no web
// server can serve, and exits 0. Deliberately the same runtime and the same
// "runs to completion" shape as a static-site generator, differing only in what
// it produces — which is the one thing the platform is allowed to judge it on.
fn main() -> std::io::Result<()> {
    fs::create_dir_all("out")?;
    let mut csv = fs::File::create("out/report.csv")?;
    writeln!(csv, "metric,value")?;
    for i in 1..=100 {
        writeln!(csv, "row_{},{}", i, i * 7)?;
    }
    let mut json = fs::File::create("out/summary.json")?;
    writeln!(json, "{{\"rows\":100,\"status\":\"complete\"}}")?;
    println!("batch complete: wrote out/report.csv and out/summary.json");
    Ok(())
}
