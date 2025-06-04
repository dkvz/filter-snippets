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

    // let lines = io::stdin().lines();
    // for line in lines {
    //     println!("got a line: {}", line.unwrap());
    // }

    Ok(())
}
