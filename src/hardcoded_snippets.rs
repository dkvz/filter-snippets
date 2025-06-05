/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* I could use anonymous functions but I thought regular ones
* are easier to test.
*/

use crate::snippets::Snippet;

// Create a big fat array
// I think it's just faster to browse than a HashMap for what I'm doing
// I have to update the count manually though. Rust is fun.
pub const SNIPPETS: [Snippet; 1] = [Snippet {
    name: "b-image",
    placeholders: Some(
        "max-width (px)\n\
        image_link image_src\n\
        alt\n\
        legend",
    ),
    process_snippet: |_| String::from("IMAGE"),
}];
