// region: lmake_md_to_doc_comments include README.md A //!
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
    //prepare the event listener for keyup
    set_event_listener_regex_text_on_keyup();
    set_event_listener_substitution_on_keyup();
    set_event_listener_test_string_on_keyup();
    debug_write("--- rust_regex_explanation_pwa end ---");
    Ok(())
}

/// code of event handler
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
