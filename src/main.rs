use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = args.path.clone();
    let file = std::fs::File::open(&path)
        .with_context(|| format!("could not read path: {:?}", path.display()))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.contains(&args.pattern) {
                println!("{}", l);
            }
        }
    }

    Ok(())
}