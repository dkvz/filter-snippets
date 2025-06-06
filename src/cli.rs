use crate::snippets::{self, snippet_placeholder};
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

    if args.list {
        println!("{}", snippets::snippet_list().join("\n"));
        return Ok(());
    }

    // We should have a snippet name here or it's an error
    if args.snippet.is_none() {
        return Err(anyhow!("missing the snippet name"));
    }
    let name = args.snippet.unwrap();

    // If requesting placeholders, we don't care about the input.
    if args.placeholders {
        let p = snippet_placeholder(&name)?;
        println!("{}", p);
        return Ok(());
    }

    // Requested an acutal snippet, processing the input:
    let lines = input_lines();
    let value = if lines.is_empty() {
        snippets::snippet_value(&name, None)?
    } else {
        snippets::snippet_value(&name, Some(&lines))?
    };

    println!("{}", value);
    Ok(())
}

fn input_lines() -> Vec<String> {
    let mut input_lines: Vec<String> = Vec::new();

    let mut lines_iter = io::stdin().lines().into_iter();
    // This is ugly lol
    if let Some(l) = lines_iter.next() {
        if let Ok(line_str) = l {
            if !line_str.trim().is_empty() {
                input_lines.push(line_str)
            }
        }
    }

    for line in lines_iter {
        // I ignore readline errors
        // Don't even know what they can be I can't
        // create one myself.
        if line.is_ok() {
            input_lines.push(line.unwrap());
        }
    }

    input_lines
}
