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

    find_mathes(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

fn find_mathes(reader: impl BufRead, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in reader.lines() {
        if let Ok(l) = line {
            if l.contains(pattern) {
                writeln!(writer, "{}", l)?;
            }
        }
    }

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = vec![];
    find_mathes("In principio erat Verbum,\net Verbum erat apud Deum,\net Deus erat Verbum.".as_bytes(), "Deum", &mut result);
    assert_eq!(result, b"et Verbum erat apud Deum,\n");
}
