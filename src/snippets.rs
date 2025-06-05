use crate::hardcoded_snippets::SNIPPETS;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Snippet {
    pub name: &'static str,
    pub placeholders: Option<&'static str>,
    pub process_snippet: fn(values: Option<&Vec<String>>) -> String,
}

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

// Returns all the names of snippets sorted alphabetically
pub fn snippet_list() -> Vec<String> {
    let mut snips: Vec<String> = SNIPPETS.iter().map(|s| String::from(s.name)).collect();
    snips.sort();
    snips
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
