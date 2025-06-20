use html_escape::encode_text;

pub fn values_to_array(values: Option<&Vec<String>>, count: usize) -> Vec<&str> {
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

pub fn escape_double_quotes(value: &str) -> String {
    value.replace("\"", "&quot;")
}

pub fn escape_html(value: &str) -> String {
    encode_text(value).to_string()
}
