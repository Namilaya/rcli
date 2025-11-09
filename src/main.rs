use anyhow::Result;
use clap::Parser;
use rcli::{Opts, execute};

fn main() -> Result<()> {
    let cmd = Opts::parse();
    execute(&cmd)
}
