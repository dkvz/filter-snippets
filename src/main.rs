mod cli;
mod hardcoded_snippets;
mod snippets;
mod utils;

use anyhow::Result;
use cli::run;

fn main() -> Result<()> {
    // Wait what am I doing here, clean code or something?
    run()
}
