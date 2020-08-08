// test_string_mod.rs

use crate::html_encoded_push;
//use crate::web_sys_mod::debug_write;
use crate::web_sys_mod::*;

use regex::Regex;

/// color the test string
pub fn test_string_html(regex_text: &str, test_string: &str) -> HtmlEncoded {
    let mut html = HtmlEncoded::new();
    // early return for errors
    let rgx = match Regex::new(regex_text) {
        Ok(r) => r,
        Err(_e) => {
            html_encoded_push!(html, "{}", test_string);
            return html;
        }
    };

    // vec of (kind, pos, color)
    // kind id s- start, e-end
    let mut span_colors = vec![];
    //first fill and then sort, start and end together
    for c in rgx.captures_iter(test_string) {
        // start the index somewhere else than 0, to have different colors
        let mut color_index = 9;
        for i in 0..c.len() {
            span_colors.push(('s', c.get(i).unwrap().start(), color_index));
            span_colors.push(('e', c.get(i).unwrap().end(), color_index));
            color_index_increment(&mut color_index);
        }
    }
    span_colors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //debug_write(&format!("{:?}", span_colors));
    let mut cursor_pos = 0;
    for c in span_colors.iter() {
        html_encoded_push!(html, "{}", &test_string[cursor_pos..c.1]);
        if c.0 == 's' {
            html_encoded_push!(html, r#"<span class="b_color_{}">"#, &format!("{:02}", c.2));
        } else {
            // c.0== 'e'
            html_encoded_push!(html, r#"</span>"#);
        }
        cursor_pos = c.1;
    }
    html_encoded_push!(html, "{}", &test_string[cursor_pos..]);
    // return
    html
}
