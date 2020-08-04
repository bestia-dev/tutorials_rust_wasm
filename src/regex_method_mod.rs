use regex::Regex;

/// runs 6 regex methods and show result as html
pub fn lib_main(regex_text: &str, substitution: &str, test_string: &str) -> String {
    let mut ret = String::new();
    //ret.push_str(r#"<span class="hljs-comment">--- regex methods start ---</span>"#);

    ret.push_str("This is using the same Rust code of the field code-gen.\n");
    ret.push_str(&format!(
        r##"<span class="hljs-keyword">let</span> <span class="hljs-variable">rgx</span> =  <span class="hljs-class">Regex</span>::<span class="hljs-function">new</span>(r#"<span class="hljs-section">{}</span>"#).<span class="hljs-function">unwrap()</span>;</span>"##,
        regex_text)
    );
    ret.push_str("\n");
    ret.push_str("There are 6 important Regex methods for different use-cases:\n");
    ret.push_str(r#"<span class="hljs-function">is_match()</span>, <span class="hljs-function">find()</span>, <span class="hljs-function">find_iter()</span>, <span class="hljs-function">capture()</span>, <span class="hljs-function">capture_iter()</span>, <span class="hljs-function">replace_all()</span>"#);
    ret.push_str("\n");
    ret.push_str("\n");
    // prepared example
    //let test_string = "origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)";
    // substitution for replace_all()
    // the $1, $2,.. are placeholders for the found capture group
    //let substitution = "OnlyThe$1";

    // 1.uncomment for is_match = false
    //let rgx: &Regex = &Regex::new(r#"xxx"#).unwrap();
    // 2. uncomment for is_match = true
    let rgx = match Regex::new(regex_text) {
        Ok(t) => t,
        Err(e) => {
            ret.push_str(&format!("Error: {}\n", e));
            return ret;
        }
    };

    is_match(&rgx, &test_string, &mut ret);
    find(&rgx, &test_string, &mut ret);
    find_iter(&rgx, &test_string, &mut ret);
    captures(&rgx, &test_string, &mut ret);
    captures_iter(&rgx, &test_string, &mut ret);
    replace_all(&rgx, &test_string, substitution, &mut ret);

    //ret.push_str(r#"<span class="hljs-comment">--- regex methods end ---</span>"#);
    ret.push_str("\n");
    // return
    ret
}

/// example how to use the is_match() method
fn is_match(rgx: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str(r#"<span class="hljs-function">rgx.is_match(test_string)</span>"#);
    ret.push_str("\n");
    if rgx.is_match(test_string) {
        ret.push_str(r#"    <span class="hljs-keyword">True</span> - is match."#);
    } else {
        ret.push_str(r#"    <span class="hljs-keyword">False</span> - no match."#);
    }
    ret.push_str("\n\n");
}

/// example how to find the first occurrence
fn find(rgx: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str(r#"<span class="hljs-function">rgx.find(test_string)</span> - <span class="hljs-comment">only the first occurrence</span>"#);
    ret.push_str("\n");
    // method find() returns Option:None if not found.
    // There are more than one way in Rust to check for `possibility of absence`.
    // The first way is the methods unwrap() or expect(),
    // but they are good only for tests and examples. Never use them in production code.

    // using pattern matching (match Control Flow Operator) for `case analysis `.
    match rgx.find(test_string) {
        Some(m) => ret.push_str(&format!(
            r#"    <span class="hljs-comment">1. syntax find:</span> <span class="hljs-keyword">{}..{}</span> {}"#,
            m.start(),
            m.end(),
            m.as_str()
        )),
        None => ret.push_str(r#"    <span class="hljs-comment">1. syntax find:</span> None"#),
    }
    ret.push_str("\n");
    // using `if let`syntax
    if let Some(m) = rgx.find(test_string) {
        ret.push_str(&format!(
            r#"    <span class="hljs-comment">2. syntax find:</span> <span class="hljs-keyword">{}..{}</span> {}"#,
            m.start(),
            m.end(),
            m.as_str()
        ));
    } else {
        ret.push_str(r#"    <span class="hljs-comment">2. syntax find:</span> None"#);
    }
    ret.push_str("\n");
    // using map_or_else()
    let mut workaround = String::new();
    rgx.find(test_string).map_or_else(
        || workaround.push_str(r#"    <span class="hljs-comment">3. syntax find:</span> None"#),
        |m| {
            ret.push_str(&format!(
                r#"    <span class="hljs-comment">3. syntax find:</span> <span class="hljs-keyword">{}..{}</span> {}"#,
                m.start(),
                m.end(),
                m.as_str()
            ))
        },
    );
    ret.push_str(&workaround);
    ret.push_str("\n");
    ret.push_str("\n");
}

/// example how to use find_iter() method - iterator
fn find_iter(rgx: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str(r#"<span class="hljs-function">rgx.find_iter(test_string)</span>"#);
    ret.push_str("\n");
    for m in rgx.find_iter(test_string) {
        ret.push_str(&format!(
            r#"    <span class="hljs-comment">find_iter:</span> <span class="hljs-keyword">{}..{}</span> {}"#,
            m.start(),
            m.end(),
            m.as_str()
        ));
        ret.push_str("\n");
    }
    //ret.push_str(r#"<span class="hljs-comment">find_iter end</span>"#);
    ret.push_str("\n");
}

/// example how to capture only the first occurrence of regex capture groups
/// using the captures() method for regex capture groups
fn captures(rgx: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str(r#"<span class="hljs-function">rgx.captures(test_string)</span><span class="hljs-comment"> - only the first occurrence</span>"#);
    ret.push_str("\n");
    // same 3 possible syntax to react to the `possibility of absence` Option:None
    // as in the function find()
    match rgx.captures(test_string) {
        Some(m) => {
            if m.len() == 2 {
                ret.push_str(&format!(
                    r#"    <span class="hljs-comment">1. captures:</span> <span class="hljs-keyword">{}</span> , {} "#,
                    &m[1], &m[0]
                ));
            } else {
                ret.push_str(r#"    <span class="hljs-comment">1. captures:</span> Zero"#);
            }
        }
        None => ret.push_str(r#"    <span class="hljs-comment">1. captures:</span> None"#),
    }
    //ret.push_str("\n");
    //ret.push_str(r#"<span class="hljs-comment">captures end</span>"#);
    ret.push_str("\n");
    ret.push_str("\n");
}

/// example how to use captures_iter() method - iterator
fn captures_iter(rgx: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str(r#"<span class="hljs-function">rgx.captures_iter(test_string)</span>"#);
    ret.push_str("\n");
    for m in rgx.captures_iter(test_string) {
        if m.len() == 2 {
            ret.push_str(&format!(
                r#"    <span class="hljs-comment">captures_iter:</span> <span class="hljs-keyword">{}</span> , {}"#,
                &m[1], &m[0]
            ));
        } else {
            ret.push_str(r#"    <span class="hljs-comment">captures_iter:</span> Zero"#);
        }
        ret.push_str("\n");
    }
    //ret.push_str(r#"<span class="hljs-comment">captures_iter end</span>"#);
    ret.push_str("\n");
}

/// example of how to use replace_all() method
/// the $1, $2,.. are placeholders for the found capture group
fn replace_all(rgx: &Regex, test_string: &str, replace_string: &str, ret: &mut String) {
    ret.push_str(
        r#"<span class="hljs-function">rgx.replace_all(test_string, replace_string)</span>"#,
    );
    ret.push_str("\n");
    let new_string = rgx.replace_all(test_string, replace_string).to_string();
    ret.push_str(r#"    <span class="hljs-comment">replaced string:</span>"#);
    ret.push_str("\n");
    ret.push_str(&format!("{}", new_string));
    //ret.push_str("\n");
    //ret.push_str(r#"<span class="hljs-comment">replace_all end</span>"#);
    ret.push_str("\n");
    ret.push_str("\n");
}
