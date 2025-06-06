/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* Thought of using real functions instead of closures to make
* testing easy but function typing and sharing is too annoying.
*/

use crate::snippets::Snippet;

// Could be in some utils module
fn values_to_array(values: Option<&Vec<String>>, count: usize) -> Vec<&str> {
    if let Some(vals) = values {
        let mut ret: Vec<&str> = Vec::with_capacity(count);
        for v in vals {
            ret.push(v);
        }
        while ret.len() < count {
            ret.push("");
        }
        ret
    } else {
        vec![""; count]
    }
}

// Create a big fat array
// I think it's just faster to browse than a HashMap for what I'm doing
// I have to update the count manually though. Rust is fun.
pub const SNIPPETS: [Snippet; 6] = [
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
    },
    Snippet {
        name: "b-img-s",
        placeholders: Some("src\nalt"),
        process_snippet: |values| {
            let vals = values_to_array(values, 2);
            format!(
                "<p><img src=\"{}\" alt= \"{}\" \
            class=\"responsive-img center-image\"></p>",
                vals[0], vals[1]
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
                vals[0], vals[1], vals[2]
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
                ret.push_str(&empty_line);
                // Got no better way to do this
                ret.push_str(&empty_line);
            } else {
                let mut vals = values.unwrap().iter();
                while let Some(v) = vals.next() {
                    // Split the values (space separated):
                    let urls: Vec<&str> = v.split(' ').collect();
                    let href = *(urls.get(0).unwrap_or(&""));
                    let src = *(urls.get(1).unwrap_or(&href));
                    // We also need the next value for alt text.
                    let alt: &str = match vals.next() {
                        Some(s) => &s,
                        None => "",
                    };
                    let line = tpl
                        .replace("{href}", href)
                        .replace("{src}", src)
                        .replace("{alt}", alt);
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
            if let Some(vals) = values {
                let mut v_iter = vals.iter();
                let first = v_iter.next();
                if first.is_some_and(|f| f.len() < 19) {
                    ret.push_str(&format!(
                        "<pre class=\"screen\"><code class=\"language-{}\">",
                        first.unwrap()
                    ));
                } else {
                    ret.push_str("<pre class=\"screen\"><code>");
                }
                // Push the rest of the lines
                for line in v_iter {
                    ret.push_str(line);
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
];
