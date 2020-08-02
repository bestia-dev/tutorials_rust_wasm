use regex::Regex;

pub fn lib_main(regex_text: &str, substitution: &str, test_string: &str) -> String {
    let mut ret = String::new();
    ret.push_str(&format!("--- regex methods start ---\n"));

    // prepared example
    //let test_string = "origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)";
    // substitution for replace_all()
    // the $1, $2,.. are placeholders for the found capture group
    //let substitution = "OnlyThe$1";

    // 1.uncomment for is_match = false
    //let regex_text: &Regex = &Regex::new(r#"xxx"#).unwrap();
    // 2. uncomment for is_match = true
    let regex_text = match Regex::new(regex_text) {
        Ok(t) => t,
        Err(e) => return e.to_string(),
    };

    is_match(&regex_text, &test_string, &mut ret);
    find(&regex_text, &test_string, &mut ret);
    find_iter(&regex_text, &test_string, &mut ret);
    captures(&regex_text, &test_string, &mut ret);
    captures_iter(&regex_text, &test_string, &mut ret);
    replace_all(&regex_text, &test_string, substitution, &mut ret);

    ret.push_str(&format!("--- regex methods end ---\n"));
    // return
    ret
}

/// example how to use the is_match() method
fn is_match(regex_text: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str("regex_text.is_match(test_string)\n");
    if regex_text.is_match(test_string) {
        ret.push_str(&format!("    True - is match.\n"));
    } else {
        ret.push_str(&format!("    False - no match.\n"));
    }
    ret.push_str("\n");
}

/// example how to find the first occurrence
fn find(regex_text: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str("regex_text.find(test_string)\n");
    // method find() returns Option:None if not found.
    // There are more than one way in Rust to check for `possibility of absence`.
    // The first way is the methods unwrap() or expect(),
    // but they are good only for tests and examples. Never use them in production code.

    // using pattern matching (match Control Flow Operator) for `case analysis `.
    match regex_text.find(test_string) {
        Some(m) => ret.push_str(&format!(
            "    1. find: {} {} {}\n",
            m.start(),
            m.end(),
            m.as_str()
        )),
        None => ret.push_str(&format!("    1. find: None\n")),
    }
    // using `if let`syntax
    if let Some(m) = regex_text.find(test_string) {
        ret.push_str(&format!(
            "    2. find: {} {} {}\n",
            m.start(),
            m.end(),
            m.as_str()
        ));
    } else {
        ret.push_str(&format!("    2. find: None\n"));
    }

    // using map_or_else()
    // error ret is 2 time borrowed
    /*
    regex_text.find(test_string).map_or_else(
        || ret.push_str(&format!("    3. find: None\n")),
        |m| {
            ret.push_str(&format!(
                "    3. find: {} {} {}\n",
                m.start(),
                m.end(),
                m.as_str()
            ))
        },
    );
    */
    ret.push_str("\n");
}

/// example how to use find_iter() method - iterator
fn find_iter(regex_text: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str("regex_text.find_iter(test_string)\n");
    for m in regex_text.find_iter(test_string) {
        ret.push_str(&format!(
            "    find_iter: {} {} {}\n",
            m.start(),
            m.end(),
            m.as_str()
        ))
    }
    ret.push_str(&format!("find_iter end\n"));
    ret.push_str("\n");
}

/// example how to capture only the first occurrence of regex capture groups
/// using the captures() method for regex capture groups
fn captures(regex_text: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str("regex_text.captures(test_string)\n");
    // same 3 possible syntax to react to the `possibility of absence` Option:None
    // as in the function find()
    match regex_text.captures(test_string) {
        Some(m) => {
            if m.len() == 2 {
                ret.push_str(&format!("    1. captures: {} - {} \n", &m[1], &m[0]));
            } else {
                ret.push_str(&format!("    1. captures: Zero\n"));
            }
        }
        None => ret.push_str(&format!("    1. captures: None\n")),
    }
    ret.push_str(&format!("captures end\n"));
    ret.push_str("\n");
}

/// example how to use captures_iter() method - iterator
fn captures_iter(regex_text: &Regex, test_string: &str, ret: &mut String) {
    ret.push_str("regex_text.captures_iter(test_string)\n");
    for m in regex_text.captures_iter(test_string) {
        if m.len() == 2 {
            ret.push_str(&format!("    captures_iter: {} - {}\n", &m[1], &m[0]));
        } else {
            ret.push_str(&format!("    captures_iter: Zero\n"));
        }
    }
    ret.push_str(&format!("captures_iter end\n"));
    ret.push_str("\n");
}

/// example of how to use replace_all() method
/// the $1, $2,.. are placeholders for the found capture group
fn replace_all(regex_text: &Regex, test_string: &str, replace_string: &str, ret: &mut String) {
    ret.push_str("regex_text.replace_all(test_string, replace_string)\n");
    let new_string = regex_text
        .replace_all(test_string, replace_string)
        .to_string();
    ret.push_str(&format!("    replaced:\n{}", new_string));
    ret.push_str(&format!("replace_all end\n"));
    ret.push_str("\n");
}
