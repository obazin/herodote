use clap::Parser;
use model::GPTInteraction;
use std::path::PathBuf;
use std::{fs, process};
mod conversation_writer;
mod converter;
mod model;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output_folder: PathBuf,
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&cli.input)
        .map_err(|e| format!("Failed to read file '{}': {}", cli.input.display(), e))?;

    let interactions: Vec<GPTInteraction> = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse JSON in file '{}': {}",
            cli.input.display(),
            e
        )
    })?;

    conversation_writer::write(
        interactions
            .into_iter()
            .map(converter::create_conversation_from)
            .collect(),
        cli.output_folder,
    );
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
