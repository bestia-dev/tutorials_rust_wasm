use rust_wasm_websys_utils::websysmod::*;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    //handle the button click

    debug_write("12345");

    set_run_button_on_click();

    Ok(())
}

#[wasm_bindgen]
pub fn greet(a: &str) -> String {
    format!("Hello, {}!", a)
}

fn set_run_button_on_click() {
    let html_element = get_element_by_id("run_button");
    let html_element = unwrap!(html_element.dyn_into::<web_sys::HtmlElement>());
    let closure = Closure::wrap(Box::new(move || {
        let regex_text = get_text_area_element_value_string_by_id("expression");
        set_text_area_element_value_string_by_id("explanation", &regex_text);
    }) as Box<dyn FnMut()>);

    html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

fn write_text() {
    let explanation = get_element_by_id("explanation");
    let explanation = unwrap!(explanation.dyn_into::<web_sys::HtmlTextAreaElement>());
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
    // debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let text_area_html_element =
        unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    // debug_write("before value()");
    text_area_html_element.set_value(value);
}
