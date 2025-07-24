/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* Thought of using real functions instead of closures to make
* testing easy but function typing and sharing is too annoying.
*/

use crate::{
    snippets::Snippet,
    utils::{escape_double_quotes, escape_html, values_to_array},
};

// I re-use this one in two snippets
fn surround_with_html_comments(
    values: Option<&Vec<String>>,
    first_line_text: Option<&str>,
) -> String {
    let mut ret = String::new();
    ret.push_str("<!--");
    if let Some(ftext) = first_line_text {
        ret.push(' ');
        ret.push_str(ftext);
    }

    // We have to check if the first line is empty
    if let Some(vals) = values {
        let mut v_iter = vals.iter();
        if let Some(first_line) = v_iter.next() {
            let first_line_t = first_line.trim();
            if vals.len() == 1 {
                ret.push(' ');
                ret.push_str(first_line_t);
                ret.push(' ');
            } else {
                ret.push('\n');
                ret.push_str(first_line_t);
                ret.push('\n');
            }
        }

        // Now push the other lines:
        for v in v_iter {
            ret.push_str(v);
            ret.push('\n');
        }
    }
    ret.push_str("-->");

    return ret;
}

// Create a big fat array
// I think it's just faster to browse than a HashMap for what I'm doing
// I have to update the count manually though. Rust is fun.
pub const SNIPPETS: [Snippet; 10] = [
    Snippet {
        name: "b-img",
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
                    &escape_double_quotes(alt),
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
    },
    Snippet {
        name: "b-img-s",
        placeholders: Some("src\nalt"),
        process_snippet: |values| {
            let vals = values_to_array(values, 2);
            format!(
                "<p><img src=\"{}\" alt= \"{}\" \
            class=\"responsive-img center-image\"></p>",
                vals[0],
                &escape_double_quotes(vals[1])
            )
        },
    },
    Snippet {
        name: "b-img-sl",
        placeholders: Some("link\nsrc\nalt"),
        process_snippet: |values| {
            let vals = values_to_array(values, 3);
            format!(
                "<p><a href=\"{}\" target=\"_blank\">\
                    <img src=\"{}\" alt= \"{}\" \
            class=\"responsive-img center-image\"></a></p>",
                vals[0],
                vals[1],
                &escape_double_quotes(vals[2])
            )
        },
    },
    Snippet {
        name: "n-img",
        placeholders: Some("src\nalt"),
        process_snippet: |values| {
            let vals = values_to_array(values, 2);
            format!(
                "<p class=\"text-center\">\n\
                    <img src=\"{}\" alt= \"{}\">\n\
                </p>",
                vals[0],
                &escape_double_quotes(vals[1])
            )
        },
    },
    Snippet {
        name: "n-img-l",
        placeholders: Some("link\nsrc\nalt"),
        process_snippet: |values| {
            let vals = values_to_array(values, 3);
            format!(
                "<p class=\"text-center\"><a href=\"{}\" target=\"_blank\">\n\
                    <img src=\"{}\" alt= \"{}\">\n\
                </a></p>",
                vals[0],
                vals[1],
                &escape_double_quotes(vals[2])
            )
        },
    },
    Snippet {
        name: "b-img-gal",
        placeholders: Some(
            "link_href img_src\n\
            alt\n\
            ...repeat",
        ),
        process_snippet: |values| {
            let tpl = "<a href=\"{href}\" target=\"_blank\" rel=\"noopener\">\
                <img src=\"{src}\" alt=\"{alt}\"></a>\n";
            let mut ret = String::from("<p class=\"image-row\">\n");

            if values.is_none_or(|vals| vals.is_empty()) {
                // If there are no values just output a gallery with
                // two example empty elements.
                let empty_line = tpl
                    .replace("{href}", "")
                    .replace("{src}", "")
                    .replace("{alt}", "");
                ret.push_str(&empty_line.repeat(2));
                // Got no better way to do this
                // ret.push_str(&empty_line);
            } else {
                let mut vals = values.unwrap().iter();
                while let Some(v) = vals.next() {
                    // Split the values (space separated):
                    let urls: Vec<&str> = v.split(' ').collect();
                    let href = *(urls.get(0).unwrap_or(&""));
                    let src = *(urls.get(1).unwrap_or(&href));
                    // We also need the next value for alt text.
                    let alt: String = match vals.next() {
                        Some(s) => escape_double_quotes(s),
                        None => String::from(""),
                    };
                    let line = tpl
                        .replace("{href}", href)
                        .replace("{src}", src)
                        .replace("{alt}", &alt);
                    ret.push_str(&line);
                }
            }

            ret.push_str("</p>");
            return ret;
        },
    },
    Snippet {
        name: "b-code",
        placeholders: Some(
            "language\n\
            Your code here (can be multiple lines)",
        ),
        process_snippet: |values| {
            let mut ret = String::new();
            // If the first line is longer than 18 characters it's probably a line of code.
            // I'm also throwing in that it should not have spaces.
            if let Some(vals) = values {
                let mut v_iter = vals.iter();
                let first: &str = v_iter.next().map(|f| f.as_str()).unwrap_or("");
                if first.len() < 19 && !first.contains(' ') {
                    ret.push_str(&format!(
                        "<pre class=\"screen\"><code class=\"language-{}\">",
                        first
                    ));
                } else {
                    ret.push_str(&format!(
                        "<pre class=\"screen\"><code>{}\n",
                        escape_html(first)
                    ));
                }
                // Push the rest of the lines
                for line in v_iter {
                    ret.push_str(&escape_html(line));
                    ret.push('\n');
                }
            } else {
                ret.push_str("<pre class=\"screen\"><code>");
            }
            ret.push_str("</code></pre>");

            return ret;
        },
    },
    Snippet {
        name: "b-video",
        placeholders: Some("video_url"),
        process_snippet: |values| {
            let vals = values_to_array(values, 1);
            format!(
                "<video class=\"responsive-video\" preload=\"none\" controls=\"\" \
                    poster=\"/wp-content/stuff/1080p_shrimp_pholder.jpg\">\n\
                    <source src=\"{}\" type=\"video/mp4\">\n\
                    <p>Votre navigateur n'a pas la capacité de lire les vidéos HTML5.</p>\n\
                </video>",
                vals[0]
            )
        },
    },
    Snippet {
        name: "com",
        placeholders: Some("text_to_comment (can be multiple lines)"),
        process_snippet: |values| surround_with_html_comments(values, None),
    },
    Snippet {
        name: "comt",
        placeholders: Some("text_to_comment (can be multiple lines)"),
        process_snippet: |values| surround_with_html_comments(values, Some("TODO:")),
    },
];
