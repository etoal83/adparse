use structopt::StructOpt;
use std::io::{BufReader, BufRead, Error};
use std::fs::File;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    let file = File::open(&args.path)?;
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