/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* Thought of using real functions instead of closures to make
* testing easy but function typing and sharing is too annoying.
*/

use crate::snippets::Snippet;

// Create a big fat array
// I think it's just faster to browse than a HashMap for what I'm doing
// I have to update the count manually though. Rust is fun.
pub const SNIPPETS: [Snippet; 1] = [Snippet {
    name: "b-image",
    placeholders: Some(
        "max-width (in px)\n\
        image_link image_src\n\
        alt\n\
        legend",
    ),
    process_snippet: |values| {
        let args: [&str; 5] = if let Some(vals) = values {
            // Extract the width or use 1000:
            let width = vals
                .get(0)
                .map(|w| w.parse::<u32>().unwrap_or(1000))
                .unwrap();

            let mut img_link = String::new();
            let mut img_src = String::new();
            if let Some(imgs) = vals.get(1) {
                let urls: Vec<&str> = imgs.split(' ').collect();
                img_link = String::from(*(urls.get(0).unwrap_or(&"/wp-content/stuff/")));
                match urls.get(1) {
                    Some(u) => {
                        img_src = String::from(*u);
                    }
                    None => {
                        img_src = img_link.clone();
                    }
                }
            }

            let alt = if vals.len() > 2 {
                vals.get(2).unwrap()
            } else {
                ""
            };

            let legend = if vals.len() > 3 {
                vals.get(3).unwrap()
            } else {
                ""
            };

            [
                &width.to_string(),
                &img_link.clone(),
                &img_src.clone(),
                alt,
                legend,
            ]
        } else {
            [""; 5]
        };

        // Isn't there a better way to flatten my array into format! arguments?
        // I don't know lol
        format!(
            "<div class=\"card-panel z-depth-3 article-image center-image\" \
            style=\"max-width: {}px\">\n\
            <a href=\"{}\" target=\"_blank\">\
            <img src=\"{}\" alt=\"{}\" class=\"responsive-img\"></a>\n\
            <div class=\"image-legend\">{}</div>\n\
            </div>",
            args[0], args[1], args[2], args[3], args[4]
        )
    },
}];
