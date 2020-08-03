// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_regex_explanation_pwa
//!
//! ***version: 2020.803.1029  date: 2020-08-03 authors: Luciano Bestia***  
//! **Rust regex explanations in PWA**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-864-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-81-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-74-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! [![Documentation](https://docs.rs/rust_regex_explanation_pwa/badge.svg)](https://docs.rs/rust_regex_explanation_pwa/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/rust_regex_explanation_pwa.svg)](https://web.crev.dev/rust-reviews/crate/rust_regex_explanation_pwa/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/rust_regex_explanation_pwa/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! ## Regex explanation and testing
//!
//! Regex is great. But it is much easier to write and understand with a little help of explanations.  
//! Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  
//! We will need a web file server because security does not allow loading modules from local file.  
//! Install this basic one:
//! `cargo install basic-http-server`  
//! Run the server in a separate terminal so it can stay running all the time:  
//! Go to the content folder:  
//! `cd rustprojects/rust_regex_explanation_pwa/web_server_folder/web_content_folder`  
//! `basic-http-server`  
//! Open the browser on:  
//! `http://127.0.0.1:4000`  
//!
//! ## Minimal example of Wasm/Webassembly with vanilla Html, Css and Javascript
//!
//! First decision - no frameworks. Just vanilla. Then no javascript.  
//! Some basic html. Some basic css.  
//! All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
//! No other special requirements.  
//!
//! ## PWA
//!
//! I added the manifest, the worker and a bunch of icons.  
//!
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys_mod::*;

mod code_gen_mod;
mod regex_explanation_mod;
mod regex_method_mod;
mod web_sys_mod;

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    let regex_text = r#"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+\.([a-zA-Z0-9-]+)*"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "The email domain is: $1";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = "John.Connor@sky.net";
    set_text_area_element_value_string_by_id("test_string", test_string);
    // initial result
    on_keyup();
    //prepare the event listeners for keyup
    set_event_listener_regex_text_on_keyup();
    set_event_listener_substitution_on_keyup();
    set_event_listener_test_string_on_keyup();

    set_event_listener_explanation_max();
    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// on keyup code
fn on_keyup() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_text_area_element_value_string_by_id("test_string");

    let explanation = regex_explanation_mod::lib_main(regex_text.clone());
    set_text_area_element_value_string_by_id("explanation", &explanation);

    let regex_result = regex_method_mod::lib_main(&regex_text, &substitution, &test_string);
    set_text_area_element_value_string_by_id("regex_result", &regex_result);

    let code_gen = code_gen_mod::code_gen(&regex_text, &substitution, &test_string);
    set_text_area_element_value_string_by_id("code_gen", &code_gen);
}

// on click code
fn on_click_explanation_max() {
    debug_write("on_click_explanation_max");
    // textarea
    let html_element = get_element_by_id("explanation");
    let html_element: web_sys::HtmlElement =
        unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
    unwrap!(html_element.style().set_property("height", "800px"));
}

/// set event listener for the regex_text
fn set_event_listener_regex_text_on_keyup() {
    let html_element = get_element_by_id("regex_text");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());

    let closure = Closure::wrap(Box::new(move || {
        on_keyup();
    }) as Box<dyn FnMut()>);

    html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set event listener for the substitution
fn set_event_listener_substitution_on_keyup() {
    let html_element = get_element_by_id("substitution");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());

    let closure = Closure::wrap(Box::new(move || {
        on_keyup();
    }) as Box<dyn FnMut()>);

    html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set event listener for the test_string
fn set_event_listener_test_string_on_keyup() {
    let html_element = get_element_by_id("test_string");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());

    let closure = Closure::wrap(Box::new(move || {
        on_keyup();
    }) as Box<dyn FnMut()>);

    html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// set event listener for the explanation_max
fn set_event_listener_explanation_max() {
    let html_element = get_element_by_id("explanation_max");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());

    let closure = Closure::wrap(Box::new(move || {
        on_click_explanation_max();
    }) as Box<dyn FnMut()>);

    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
