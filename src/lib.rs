// region: lmake_md_to_doc_comments include README.md A //!
// endregion: lmake_md_to_doc_comments include README.md A //!

use rust_wasm_websys_utils::websysmod::*;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, KeyboardEvent};

mod code_gen_mod;
mod regex_explanation_mod;
mod regex_method_mod;

/// To start the Wasm application, wasm_bindgen runs this functions
#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    debug_write("--- rust_regex_explanation_pwa start ---");
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    let regex_text = "Luciano(Best)ia";
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "OnlyThe$1";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = "origin  git@github.com:LucianoBestia/rust_regex_explanation_pwa.git (fetch)\norigin  https://github.com/LucianoBestia/rust_regex_explanation_pwa (fetch)";
    set_text_area_element_value_string_by_id("test_string", test_string);
    //prepare the event listener for the button click
    set_run_button_on_click();
    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// prepare the event listener for the button click
fn set_run_button_on_click() {
    let html_element = get_element_by_id("run_button");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
    let closure = Closure::wrap(Box::new(move || {
        run_button_on_click;
    }) as Box<dyn FnMut()>);

    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// prepare the event listener for the regex_text
fn set_regex_text_on_keyup() {
    let html_element = get_element_by_id("regex_text");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
    let closure = Closure::wrap(Box::new(move |event| {
        regex_text_on_keyup(event);
    }) as Box<dyn FnMut()>);

    html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

pub fn regex_text_on_keyup(event: Event) {
    // websysmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<KeyboardEvent>());
    // websysmod::debug_write(&keyboard_event.key());
    debug_write("123");
}

/// code of event handler
fn run_button_on_click() {
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

/// get text_area element value string by id
pub fn get_text_area_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let text_area_html_element =
        unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    // debug_write("before value()");
    text_area_html_element.value()
}

/// set text_area element value string by id
pub fn set_text_area_element_value_string_by_id(element_id: &str, value: &str) {
    //debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    //debug_write("before dyn_into");
    let text_area_html_element =
        unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    //debug_write("before value()");
    text_area_html_element.set_value(value);
}
