pub mod csv_parser;

pub use csv_parser::{CsvOpts, transfer_csv_to_json};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "csv parser",
    version = "1.0",
    author = "Namilaya",
    about = "An Rust CLI Application"
)]
pub struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "Parse CSV file")]
    Csv(CsvOpts),
}

pub fn execute(cmd: &Opts) -> anyhow::Result<()> {
    match &cmd.cmd {
        SubCommand::Csv(csv_opt) => {
            transfer_csv_to_json(csv_opt)?;
        }
    }
    Ok(())
}
