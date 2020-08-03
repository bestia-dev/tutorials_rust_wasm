// web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...

// region: use
use unwrap::unwrap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, Response};
// endregion: use

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console is futile for mobile phones
    // I must write it on the UI.
    // so I must access this string from the UI renderer
    // add_to_begin_of_debug_text(text);
    console::log_1(&JsValue::from_str(text));
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

#[allow(dead_code)]
/// save to local storage
pub fn save_to_local_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(window().local_storage()));
    let _x = ls.set_item(name, value);
}

#[allow(dead_code)]
/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

#[allow(dead_code)]
/// fetch in Rust with async await for executor spawn_local()
/// return the response as String. Any error will panic.
pub async fn fetch_response(url: String) -> String {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    // log1("before fetch");
    let resp_jsvalue = unwrap!(JsFuture::from(window().fetch_with_request(&request)).await);
    // log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    // log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    // debug_write(&txt_response);
    // returns response as String
    txt_response
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
