use crate::snippets;
use anyhow::{Result, anyhow};
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    placeholders: bool,

    #[arg(short, long)]
    list: bool,

    snippet: Option<String>,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    // Check if the list is requested

    // We should have a snippet name here or it's an error
    if args.snippet.is_none() {
        return Err(anyhow!("missing the snippet name"));
    }

    // If requesting placeholders, we don't care about the input.

    // Processing the input:
    let mut input_lines: Vec<String> = Vec::new();
    let lines = io::stdin().lines();
    for line in lines {
        // I ignore readline errors
        // Don't even know what they can be I can't
        // create one myself.
        if line.is_ok() {
            input_lines.push(line.unwrap());
        }
    }

    println!("OK so we got {} elements", input_lines.len());

    Ok(())
}
