/*
* Evil module with a whole bunch of hardcoded snippets.
* I need the binary to be portable (for Linux and Windows) and
* hardcoding just makes sense for now.
*
* I could use anonymous functions but I thought regular ones
* are easier to test.
*/

pub fn delete_me_later() -> String {
    snippet_b_image(true, &None)
}

fn snippet_b_image(get_placeholders: bool, values: &Option<Vec<String>>) -> String {
    let template = "max-width (px)\n\
        image_link image_src\n\
        alt\n\
        legend";
    if get_placeholders {
        String::from(template)
    } else {
        String::from("IMAGE")
    }
}
