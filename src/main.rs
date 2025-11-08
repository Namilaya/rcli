use anyhow::{Context, Result};
use clap::Parser;
use std::fs::{self, File};

#[derive(Parser, Debug)]
#[command(
    name = "csv parser",
    version = "1.0",
    author = "Namilaya",
    about = "An Rust CLI Application"
)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "Parse CSV file")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long,value_parser=check_input_file_exists, help = "Input CSV file path")]
    input: String,

    #[arg(short, long, default_value = "output.json", help = "Output file path")]
    output: String,

    #[arg(short, long, default_value_t = b',', help = "Delimiter character")]
    delimiter: u8,

    #[arg(
        long,
        default_value = "true",
        help = "Indicates if the CSV has a header"
    )]
    header: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Record {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    country: String,

    #[serde(rename = "Kit Number")]
    number: u32,
}

fn check_input_file_exists(filename: &str) -> Result<String> {
    let path = std::path::Path::new(filename);
    if path.exists() && path.is_file() {
        println!("Input file found: {:?}", fs::canonicalize(path).unwrap());
        Ok(filename.to_string())
    } else {
        anyhow::bail!(format!("File not found: {}", filename).to_string())
    }
}

fn main() -> Result<()> {
    let cmd: Opts = Opts::parse();
    match cmd.cmd {
        SubCommand::Csv(opts) => {
            transfer_csv_to_json(opts)?;
        }
    }
    Ok(())
}

fn transfer_csv_to_json(opts: CsvOpts) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(opts.delimiter)
        .has_headers(opts.header)
        .from_path(opts.input)
        .with_context(|| "Failed to open CSV file")?;

    let outputs = rdr
        .deserialize()
        .enumerate()
        .map(|(i, result)| result.with_context(|| format!("解析第{}行出现错误", i)))
        .collect::<Result<Vec<Record>>>()?;
    println!("{:?}", outputs);

    let output_file = File::create(&opts.output).unwrap();
    serde_json::to_writer(&output_file, &outputs)?;
    Ok(())
}
