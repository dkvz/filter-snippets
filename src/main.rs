mod cli;
mod snippets;

use anyhow::Result;
use cli::run;

fn main() -> Result<()> {
    // Wait what am I doing here, clean code or something?
    run()
}
