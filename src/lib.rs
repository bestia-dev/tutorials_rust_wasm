// region: lmake_md_to_doc_comments include README.md A //!
//! # rust_regex_explanation_pwa
//!
//! ***version: 2020.805.1222  date: 2020-08-05 authors: Luciano Bestia***  
//! **Rust regex explanations in PWA**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1457-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-131-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-132-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
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
//! ## contenteditable
//!
//! Every day something new. Modern browsers have a wysiwyg editor. Just add contenteditable to a <div> or <code>. Funny.  
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
mod color_test_string_mod;
mod regex_explanation_mod;
mod regex_method_mod;
mod web_sys_mod;

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");

    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    let html_encoded = HtmlEncoded::from_str(env!("CARGO_PKG_VERSION"));
    set_element_inner_html_by_id("pkg_version", &html_encoded);

    // Initialize input fields
    example_xml_1_base();

    //prepare the event listeners
    handle_click!("menu_explanation", display_block_2_and_scroll, "explanation_section", "explanation_label");
    handle_click!(
        "menu_regex_result",
        display_block_2_and_scroll,
        "regex_result_section",
        "regex_result_label"
    );
    handle_click!("menu_code_gen", display_block_2_and_scroll, "code_gen_section", "code_gen_label");
    handle_click!(
        "menu_regex_examples",
        display_block_2_and_scroll,
        "regex_examples_section",
        "regex_examples_label"
    );
    handle_click!("menu_regex_help", display_block_2_and_scroll, "regex_help_section", "regex_help_label");

    set_listener_on_keyup!("regex_text", run_regex);
    set_listener_on_keyup!("substitution", run_regex);
    handle_click!("test_string_close", display_none_2, "test_string_label", "test_string_section");
    handle_click!("test_string_less", contract_height, "test_string");
    handle_click!("test_string_more", expand_height_to_auto, "test_string");
    handle_click!("explanation_close", display_none_2, "explanation_section", "explanation_label");
    handle_click!("explanation_less", contract_height, "explanation");
    handle_click!("explanation_more", expand_height_to_auto, "explanation");
    handle_click!("regex_result_close", display_none_2, "regex_result_label", "regex_result_section");
    handle_click!("regex_result_less", contract_height, "regex_result");
    handle_click!("regex_result_more", expand_height_to_auto, "regex_result");
    handle_click!("code_gen_close", display_none_2, "code_gen_section", "code_gen_label");
    handle_click!("code_gen_less", contract_height, "code_gen");
    handle_click!("code_gen_more", expand_height_to_auto, "code_gen");
    handle_click!("regex_examples_close", display_none_2, "regex_examples_section", "regex_examples_label");
    handle_click!("regex_examples_less", contract_height, "regex_examples");
    handle_click!("regex_examples_more", expand_height_to_auto, "regex_examples");
    handle_click!("regex_help_close", display_none_2, "regex_help_label", "regex_help_section");
    handle_click!("regex_help_less", contract_height, "regex_help");
    handle_click!("regex_help_more", expand_height_to_auto, "regex_help");

    handle_click!("example_email", example_email);
    handle_click!("example_model_1", example_model_1);
    handle_click!("example_model_2", example_model_2);
    handle_click!("example_model_3", example_model_3);
    handle_click!("example_xml_1", example_xml_1);

    handle_click!("code_gen_copy", code_gen_copy);
    handle_click!("code_gen_run_in_playground", code_gen_run_in_playground);

    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// on keyup code
fn run_regex() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_element_inner_text_by_id("test_string");

    let explanation = regex_explanation_mod::lib_main(regex_text.clone());
    set_element_inner_html_by_id("explanation", &explanation);

    let regex_result = regex_method_mod::lib_main(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("regex_result", &regex_result);

    let code_gen = code_gen_mod::code_gen_html(&regex_text, &substitution, &test_string);
    set_element_inner_html_by_id("code_gen", &code_gen);

    let test_string = color_test_string_mod::color_test_string(&regex_text, &test_string);
    set_element_inner_html_by_id("test_string", &test_string);

    // Applies highlighting to the blocks on a page.
    unwrap!(js_sys::eval("hljs.highlightBlock(document.getElementById('code_gen'))"));
}

/// make visible and jump to it
fn display_block_2_and_scroll(element_1_id: &str, element_2_id: &str) {
    display_block_and_scroll(element_1_id);
    display_block_and_scroll(element_2_id);

    let html_element = get_html_element_by_id(element_1_id);
    html_element.scroll_into_view();
}

/// make visible the element
fn display_block_and_scroll(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("display", "block"));
    html_element.scroll_into_view();
}

/// make invisible
fn display_none_2(element_1_id: &str, element_2_id: &str) {
    display_none(element_1_id);
    display_none(element_2_id);
}

/// make invisible the element
fn display_none(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("display", "none"));
}

// change height on click code
fn expand_height_to_auto(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("height", "auto"));
}

// change height on click code
fn contract_height(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("height", "150px"));
}

// copy to clipboard
fn code_gen_copy() {
    let regex_text = get_text_area_element_value_string_by_id("regex_text");
    let substitution = get_text_area_element_value_string_by_id("substitution");
    let test_string = get_element_inner_text_by_id("test_string");
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
    let test_string = HtmlEncoded::from_str("John.Connor@sky.net");
    set_element_inner_html_by_id("test_string", &test_string);

    // initial result
    run_regex();
    display_block_2_and_scroll("regex_text_label", "regex_text_section");
}

// example model_base
fn example_model_base() {
    let regex_text = r#"T-"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "Robot($1)";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = HtmlEncoded::from_str(
        r#"T-1000 (Robert Patrick) Terminator known as T-101 T-800 that managed to kill John Connor explicitly named T-600s and T-1000. it jams its remaining hydrogen fuel cell into the T-X's mouth from a T-1000 sent to kill her who has been transformed into a T-3000 improvement over the earlier T-600 units also refers to the character as T-850 used the T-800 and T-850 nomenclature memory of a T-888 model, tearing a malfunctioning T-600 in half"#,
    );
    set_element_inner_html_by_id("test_string", &test_string);
}

// example model1
fn example_model_1() {
    example_model_base();
    run_regex();
    display_block_2_and_scroll("regex_text_label", "regex_text_section");
}

// example model2
fn example_model_2() {
    example_model_base();
    let regex_text = r#"T-\d+"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
    display_block_2_and_scroll("regex_text_label", "regex_text_section");
}

// example model3
fn example_model_3() {
    example_model_base();
    let regex_text = r#"T-(X|\d+)"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
    display_block_2_and_scroll("regex_text_label", "regex_text_section");
}

// example xml_1
fn example_xml_1() {
    example_xml_1_base();
    display_block_2_and_scroll("regex_text_label", "regex_text_section");
}

// example xml_1 base
fn example_xml_1_base() {
    let regex_text = r#"<title>(.+?)</title>"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = HtmlEncoded::from_str(
        r#"<catalog>
    <movie>
        <title>The Terminator</title>
        <year>1984</year>
    </movie>
    <movie>
        <title>Terminator 2: Judgment Day</title>
        <year>1991</year>
    </movie>
    <movie>
        <title>Terminator 3: Rise of the Machines</title>
        <year>2003</year>
    </movie>
    <movie>
        <title>Terminator Salvation</title>
        <year>2009</year>
    </movie>
    <movie>
        <title>Terminator Genisys</title>
        <year>2015</year>
    </movie>
    <movie>
        <title>Terminator: Dark Fate</title>
        <year>2019</year>
    </movie>
</catalog>
  "#,
    );
    set_element_inner_html_by_id("test_string", &test_string);
    run_regex();
}
