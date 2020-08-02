pub fn code_gen(regex_text: &str, substitution: &str, test_string: &str) -> String {
    let gen = String::from( r#####"
    //! <https://github.com/LucianoBestia/regex101_rust>  
//! Run this code online in the playground:
//! <https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=10717a3fe934b9583fb675e327833edc>  
//!
//! In Cargo.toml include the dependency to the regex crate:  
//! regex = "1.3.6"
use regex::Regex;

// To avoid multiple initialization of the regex and achieve better performance,
// especially when the regex is used in a loop include also the lazy_static crate:
// lazy_static="1.4.0"
use lazy_static::lazy_static;

// Use "raw strings" syntax to avoid unnecessary escaping.
// It will leave the regex expression unmodified. No more problems with \ or " characters.
// Raw string syntax is like: `r#"one two three"#`
// If the expression is not correct, we want to know it immediately.
// Let panic the constructor with `unwrap()`.
lazy_static! {
    static ref REGEX_01: Regex = Regex::new(r###"Luciano(Best)ia"###).unwrap();
}

fn main() {
    println!("--- regex101_rust start ---");

    // prepared example
    let test_string = r###"origin  git@github.com:LucianoBestia/regex101_rust.git (fetch)\norigin  https://github.com/LucianoBestia/regex101_rust (fetch)"###;
    // substitution for replace_all()
    // the $1, $2,.. are placeholders for the found capture group
    let substitution = r###"OnlyThe$1"###;

    // 1.uncomment for is_match = false
    //let regex: &Regex = &Regex::new(r#"xxx"#).unwrap();
    // 2. uncomment for is_match = true
    let regex = &REGEX_01;

    is_match(&regex, &test_string);
    find(&regex, &test_string);
    find_iter(&regex, &test_string);
    captures(&regex, &test_string);
    captures_iter(&regex, &test_string);
    replace_all(&regex, &test_string, substitution);

    println!("--- regex101_rust end ---");
}

/// example how to use the is_match() method
fn is_match(regex: &Regex, test_string: &str) {
    if regex.is_match(test_string) {
        println!("True - is match.");
    } else {
        println!("False - no match.");
    }
}

/// example how to find the first occurrence
fn find(regex: &Regex, test_string: &str) {
    // method find() returns Option:None if not found.
    // There are more than one way in Rust to check for `possibility of absence`.
    // The first way is the methods unwrap() or expect(),
    // but they are good only for tests and examples. Never use them in production code.

    // using pattern matching (match Control Flow Operator) for `case analysis `.
    match regex.find(test_string) {
        Some(m) => println!("1. find: {} {} {}", m.start(), m.end(), m.as_str()),
        None => println!("1. find: None"),
    }
    // using `if let`syntax
    if let Some(m) = regex.find(test_string) {
        println!("2. find: {} {} {}", m.start(), m.end(), m.as_str());
    } else {
        println!("2. find: None");
    }

    // using map_or_else() combinator
    regex.find(test_string).map_or_else(
        || println!("3. find: None"),
        |m| println!("3. find: {} {} {}", m.start(), m.end(), m.as_str()),
    );
}

/// example how to use find_iter() method - iterator
fn find_iter(regex: &Regex, test_string: &str) {
    println!("find_iter start");
    for m in regex.find_iter(test_string) {
        println!("find_iter: {} {} {}", m.start(), m.end(), m.as_str())
    }
    println!("find_iter end");
}

/// example how to capture only the first occurrence of regex capture groups
/// using the captures() method for regex capture groups
fn captures(regex: &Regex, test_string: &str) {
    println!("captures start");
    // same 3 possible syntax to react to the `possibility of absence` Option:None
    // as in the function find()
    match regex.captures(test_string) {
        Some(m) => println!("1. captures: {} - {} ", &m[1], &m[0]),
        None => println!("1. captures: None"),
    }
    println!("captures end");
}

/// example how to use captures_iter() method - iterator
fn captures_iter(regex: &Regex, test_string: &str) {
    println!("captures_iter start");
    for m in regex.captures_iter(test_string) {
        println!("captures_iter: {} - {}", &m[1], &m[0])
    }
    println!("captures_iter end");
}

/// example of how to use replace_all() method
/// the $1, $2,.. are placeholders for the found capture group
fn replace_all(regex: &Regex, test_string: &str, replace_string: &str) {
    println!("replace_all start");
    let new_string = regex.replace_all(test_string, replace_string).to_string();
    println!("replaced:\n{}", new_string);
    println!("replace_all end");
}
    "#####).replace("Luciano(Best)ia",regex_text).replace("OnlyThe$1", substitution).replace("origin  git@github.com:LucianoBestia/regex101_rust.git (fetch)\norigin  https://github.com/LucianoBestia/regex101_rust (fetch)",test_string);

    //return
    gen
}
