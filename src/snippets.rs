/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* I could use anonymous functions but I thought regular ones
* are easier to test.
*/

use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Snippet {
    pub name: &'static str,
    pub placeholders: Option<&'static str>,
    pub process_snippet: fn(values: Option<&Vec<String>>) -> String,
}

// Create a big fat array
// I think it's just faster to browse than a HashMap for what I'm doing
// I have to update the count manually though. Rust is fun.
const SNIPPETS: [Snippet; 1] = [Snippet {
    name: "b-image",
    placeholders: Some(
        "max-width (px)\n\
        image_link image_src\n\
        alt\n\
        legend",
    ),
    process_snippet: |values| String::from("IMAGE"),
}];

fn get_snippet(name: &str) -> Result<&Snippet> {
    let snip = SNIPPETS
        .iter()
        .find(|s| s.name == name && s.placeholders.is_some());

    if snip.is_none() {
        // Snippet doesn't exist.
        return Err(anyhow!("snippet not found"));
    }

    Ok(snip.unwrap())
}

// If the snippet has no placeholder, return the processed value immediately.
pub fn snippet_placeholder(name: &str) -> Result<String> {
    let s = get_snippet(name)?;

    match s.placeholders {
        Some(s) => Ok(String::from(s)),
        None => Ok((s.process_snippet)(None)),
    }
}

pub fn snippet_value(name: &str, values: Option<&Vec<String>>) -> Result<String> {
    let s = get_snippet(name)?;

    Ok((s.process_snippet)(values))
}
