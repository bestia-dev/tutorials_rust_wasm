// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_regex_explanation_pwa
//!
//! ***version: 2020.804.1245  date: 2020-08-04 authors: Luciano Bestia***  
//! **Rust regex explanations in PWA**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1027-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-77-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-102-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! [![Documentation](https://docs.rs/rust_regex_explanation_pwa/badge.svg)](https://docs.rs/rust_regex_explanation_pwa/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/rust_regex_explanation_pwa.svg)](https://web.crev.dev/rust-reviews/crate/rust_regex_explanation_pwa/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/rust_regex_explanation_pwa/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//!
//! ## Try it
//!
//! <https://bestia.dev/rust_regex_explanation_pwa/>
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
//! Ok, I had to use the javascript library `highlightjs` to bring some colors to the code.  
//!
//! ## PWA
//!
//! I added the manifest, the worker and a bunch of icons.  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

use js_sys;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys_mod::*;

mod code_gen_mod;
mod regex_explanation_mod;
mod regex_method_mod;
mod web_sys_mod;

// region: macro for boilerplate code

/// set_listener_on_click!(element_1_id, function_ident)
/// set_listener_on_click!("example_email",example_email)
#[macro_export]
macro_rules! set_listener_on_click {
    ($element_1_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident();
        }) as Box<dyn FnMut()>);

        let html_element = get_element_by_id($element_1_id);
        let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// set_listener_change_height_on_click!(element_1_id, element_2_id,function_ident, height_lambda)
/// set_listener_change_height_on_click!("explanation_less","explanation", explanation_less_on_click, -100)
#[macro_export]
macro_rules! set_listener_change_height_on_click {
    ($element_1_id: expr, $element_2_id: expr, $function_ident: ident, $lambda:expr) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($element_2_id, $lambda);
        }) as Box<dyn FnMut()>);

        let html_element = get_element_by_id($element_1_id);
        let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}
/// set_listener_on_keyup!(element_id, function_ident)
/// set_listener_on_keyup!("regex_text", on_keyup)
#[macro_export]
macro_rules! set_listener_on_keyup {
    ($element_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident();
        }) as Box<dyn FnMut()>);

        let html_element = get_element_by_id($element_id);
        let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
        html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");

    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Initialize input fields
    example_email();

    //prepare the event listeners
    set_listener_on_keyup!("regex_text", on_keyup);
    set_listener_on_keyup!("substitution", on_keyup);
    set_listener_on_keyup!("test_string", on_keyup);
    set_listener_change_height_on_click!(
        "explanation_less",
        "explanation",
        change_height_on_click,
        -100
    );
    set_listener_change_height_on_click!(
        "explanation_more",
        "explanation",
        change_height_on_click,
        100
    );
    set_listener_change_height_on_click!(
        "regex_result_less",
        "regex_result",
        change_height_on_click,
        -100
    );
    set_listener_change_height_on_click!(
        "regex_result_more",
        "regex_result",
        change_height_on_click,
        100
    );
    set_listener_change_height_on_click!("code_gen_less", "code_gen", change_height_on_click, -100);
    set_listener_change_height_on_click!("code_gen_more", "code_gen", change_height_on_click, 100);
    set_listener_change_height_on_click!(
        "test_string_less",
        "test_string",
        change_height_on_click,
        -100
    );
    set_listener_change_height_on_click!(
        "test_string_more",
        "test_string",
        change_height_on_click,
        100
    );
    set_listener_change_height_on_click!(
        "regex_help_less",
        "regex_help",
        change_height_on_click,
        -100
    );
    set_listener_change_height_on_click!(
        "regex_help_more",
        "regex_help",
        change_height_on_click,
        100
    );
    set_listener_on_click!("example_email", example_email);
    set_listener_on_click!("example_model1", example_model1);
    set_listener_on_click!("example_model2", example_model2);
    set_listener_on_click!("example_model3", example_model3);

    set_listener_on_click!("code_gen_copy", code_gen_copy);

    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// on keyup code
fn on_keyup() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_text_area_element_value_string_by_id("test_string");

    let explanation = regex_explanation_mod::lib_main(regex_text.clone());
    set_element_inner_html_string_by_id("explanation", &explanation);

    let regex_result = regex_method_mod::lib_main(&regex_text, &substitution, &test_string);
    set_element_inner_html_string_by_id("regex_result", &regex_result);

    let code_gen = code_gen_mod::code_gen(&regex_text, &substitution, &test_string);
    set_element_inner_html_string_by_id("code_gen", &code_gen);
    // Applies highlighting to the blocks on a page.
    unwrap!(js_sys::eval(
        "hljs.highlightBlock(document.getElementById('code_gen'))"
    ));
}

// change height on click code
fn change_height_on_click(element_id: &str, height_lambda: i32) {
    let html_element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement =
        unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
    let height = unwrap!(html_element.style().get_property_value("height"));
    if height.is_empty() {
        unwrap!(html_element.style().set_property("height", "300px"));
    } else {
        let height: String = height.replace("px", "");
        let h = unwrap!(height.parse::<i32>());
        let new_height = format!("{}px", h + height_lambda);
        unwrap!(html_element.style().set_property("height", &new_height));
    }
}

// copy to clipboard
fn code_gen_copy() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_text_area_element_value_string_by_id("test_string");
    let code_gen = code_gen_mod::code_gen(&regex_text, &substitution, &test_string);
    // escaping the backtick for the template string multi line that is delimited with backticks
    let code_gen = code_gen.replace("`", r#"\`"#);
    let js_cmd = format!(r#"navigator.clipboard.writeText(`{}`)"#, code_gen);
    unwrap!(js_sys::eval(&js_cmd));
}

// example email
fn example_email() {
    let regex_text = r#"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+\.([a-zA-Z0-9-]+)*"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "The email domain is: $1";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = "John.Connor@sky.net";
    set_text_area_element_value_string_by_id("test_string", test_string);
    // initial result
    on_keyup();
}

// example model_base
fn example_model_base() {
    let regex_text = r#"T-"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "Robot($1)";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = r#"T-1000 (Robert Patrick) Terminator known as T-101 T-800 that managed to kill John Connor explicitly named T-600s and T-1000. it jams its remaining hydrogen fuel cell into the T-X's mouth from a T-1000 sent to kill her who has been transformed into a T-3000 improvement over the earlier T-600 units also refers to the character as T-850 used the T-800 and T-850 nomenclature memory of a T-888 model, tearing a malfunctioning T-600 in half"#;
    set_text_area_element_value_string_by_id("test_string", test_string);
}

// example model1
fn example_model1() {
    example_model_base();
    on_keyup();
}

// example model2
fn example_model2() {
    example_model_base();
    let regex_text = r#"T-\d+"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    on_keyup();
}

// example model3
fn example_model3() {
    example_model_base();
    let regex_text = r#"T-(X|\d+)"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    on_keyup();
}
