extern crate pest;
#[macro_use]
extern crate pest_derive;

use anyhow::{Context, Result};
use pest::Parser;
use structopt::StructOpt;

#[derive(Parser)]
#[grammar = "adoc.pest"]
struct AdocParser;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = args.path.clone();
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read path: {:?}", path.display()))?;

    let pairs = AdocParser::parse(Rule::adoc, &content)?;

    for pair in pairs {
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::p => println!("<p>{}</p>", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }

    Ok(())
}
