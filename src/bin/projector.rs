use anyhow::Ok;
use anyhow::Result;
use clap::Parser;
use multilang::config::Config;
use multilang::opts::Opts;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(())
}
