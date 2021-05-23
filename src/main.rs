use structopt::StructOpt;
use std::io::BufReader;
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

    adparse::find_matches(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
