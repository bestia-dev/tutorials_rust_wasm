// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_regex_explanation_pwa
//!
//! ***version: 2020.805.631  date: 2020-08-05 authors: Luciano Bestia***  
//! **Rust regex explanations in PWA**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1233-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-101-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-114-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
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
//! ## Change colors
//!
//! Users can change the colors with the use of the Chrome extension User CSS:  
//! <https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
//! Copy/paste and then edit and watch changes live:  
//! ```css
//! :root {
//!     /* color palette */
//!     /* background color */
//!     --b_color_body: #24292E;
//!     --b_color_code: #1B1D23;
//!     --b_color_grid_header: #181A1F;
//!     --b_color_container: #333842;
//!     --b_color_button: dodgerblue;
//!     /* front color */
//!     --f_color_body: #dce0e9;
//!     --f_color_code: #78C379;
//!     --f_color_link: white;
//!     --f_color_05: yellow;
//!     --f_color_06: dark-white;
//!     --f_color_07: black;
//!     /* border color*/
//!     --brd_color_01: #000;
//!     --brd_color_02: #eaecef;
//! }
//! ```
// endregion: lmake_md_to_doc_comments include README.md A //!

use js_sys;
use std::env;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys_mod::*;

mod code_gen_mod;
mod regex_explanation_mod;
mod regex_method_mod;
mod web_sys_mod;

// region: macros against boilerplate code

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

// endregion: macros against boilerplate code

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");

    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    let html_encoded = HtmlEncoded::from_str(env!("CARGO_PKG_VERSION"));
    set_element_inner_html_by_id("pkg_version", &html_encoded);

    // Initialize input fields
    example_email();

    //prepare the event listeners
    set_listener_on_keyup!("regex_text", run_regex);
    set_listener_on_keyup!("substitution", run_regex);
    set_listener_on_keyup!("test_string", run_regex);
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
    set_listener_on_click!("example_model_1", example_model_1);
    set_listener_on_click!("example_model_2", example_model_2);
    set_listener_on_click!("example_model_3", example_model_3);
    set_listener_on_click!("example_xml_1", example_xml_1);

    set_listener_on_click!("code_gen_copy", code_gen_copy);
    set_listener_on_click!("code_gen_run_in_playground", code_gen_run_in_playground);

    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// on keyup code
fn run_regex() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_text_area_element_value_string_by_id("test_string");

    let explanation = regex_explanation_mod::lib_main(regex_text.clone());
    set_element_inner_html_by_id("explanation", &explanation);

    let regex_result = regex_method_mod::lib_main(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("regex_result", &regex_result);

    let code_gen = code_gen_mod::code_gen_html(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("code_gen", &code_gen);
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
    let code_gen = code_gen_mod::code_gen_string(&regex_text, &substitution, &test_string);
    // escaping the backtick for the template string multi line that is delimited with backticks
    let code_gen = code_gen.replace("`", r#"\`"#);
    let js_cmd = format!(r#"navigator.clipboard.writeText(`{}`)"#, code_gen);
    unwrap!(js_sys::eval(&js_cmd));
}

// open playground URL in new window
fn code_gen_run_in_playground() {
    let js_cmd = r#"{
        var win = window.open('https://play.rust-lang.org/', '_blank');
        win.focus();
        }"#;
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
    run_regex();
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
fn example_model_1() {
    example_model_base();
    run_regex();
}

// example model2
fn example_model_2() {
    example_model_base();
    let regex_text = r#"T-\d+"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example model3
fn example_model_3() {
    example_model_base();
    let regex_text = r#"T-(X|\d+)"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example xml_1
fn example_xml_1() {
    let regex_text = r#"<title>(.+?)</title>"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = r#"<catalog>
    <cd>
      <title>empire burlesque</title>
      <artist>bob dylan</artist>
      <country>usa</country>
      <company>columbia</company>
      <price>10.90</price>
      <year>1985</year>
    </cd>
    <cd>
      <title>hide your heart</title>
      <artist>bonnie tyler</artist>
      <country>uk</country>
      <company>cbs records</company>
      <price>9.90</price>
      <year>1988</year>
    </cd>
    <cd>
      <title>greatest hits</title>
      <artist>dolly parton</artist>
      <country>usa</country>
      <company>rca</company>
      <price>9.90</price>
      <year>1982</year>
    </cd>
  </catalog>
  "#;
    set_text_area_element_value_string_by_id("test_string", test_string);
    run_regex();
}
