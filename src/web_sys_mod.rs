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

/// Simple macro to set listener of onclick events to an element_id.
/// no args: on_click!(element_1_id, function_ident)
/// no args: on_click!("example_email",example_email)
/// 1 args: on_click!(element_1_id, function_ident, arg_1)
/// 1 args: on_click!("example_email",example_email, "arg_1")
/// 2 args: on_click!(element_1_id, function_ident, arg_1,arg_2)
/// 2 args: on_click!("example_email",example_email, "arg_1","arg_2")
#[macro_export]
macro_rules! on_click {
    ($element_1_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident();
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
    ($element_1_id: expr, $function_ident: ident, $arg_1: expr) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($arg_1);
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
    ($element_1_id: expr, $function_ident: ident, $arg_1: expr, $arg_2: expr) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($arg_1, $arg_2);
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
    ($element_1_id: expr, $function_ident: ident, $arg_1: expr, $arg_2: expr, $arg_3: expr) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($arg_1, $arg_2, $arg_3);
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_1_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// Simple macro to set listener of onkeyup events to an element_id.
/// on_keyup!(element_id, function_ident)
/// on_keyup!("regex_text", run_regex)
#[macro_export]
macro_rules! on_keyup {
    ($element_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident();
        }) as Box<dyn FnMut()>);

        let html_element = get_html_element_by_id($element_id);
        html_element.set_onkeyup(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// Build a html, that is correctly encoded.
/// Use the `html_encoded_push!` macro.
/// Literals are pushed unencoded.
/// Variables are always encoded.
#[derive(Debug, Default)]
pub struct HtmlEncoded {
    // private field accessible only with methods
    html: String,
}
/// Push html encoded variables inside unencoded literal to HtmlEncoded.
/// Similar use as format! macro.
/// html_encoded_push!(html, template, param_1)
/// html_encoded_push!(exp.explanation_all.,"{}", &exp.reg_str)
#[macro_export]
macro_rules! html_encoded_push {
    ($html: expr, $template:expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!($template));
    };
    ($html: expr, $template:expr, $param_1: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!($template, crate::web_sys_mod::html_encode($param_1)));
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_mod::html_encode($param_1),
            crate::web_sys_mod::html_encode($param_2)
        ));
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr, $param_3: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_mod::html_encode($param_1),
            crate::web_sys_mod::html_encode($param_2),
            crate::web_sys_mod::html_encode($param_3),
        ));
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr, $param_3: expr, $param_4: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_mod::html_encode($param_1),
            crate::web_sys_mod::html_encode($param_2),
            crate::web_sys_mod::html_encode($param_3),
            crate::web_sys_mod::html_encode($param_4),
        ));
    };
}
impl HtmlEncoded {
    /// constructor of empty object
    pub fn new() -> HtmlEncoded {
        // return
        HtmlEncoded { html: String::new() }
    }
    /// html encode this str and create the object
    pub fn from_str(param_1: &str) -> HtmlEncoded {
        let mut html = HtmlEncoded::new();
        html_encoded_push!(html, "{}", param_1);
        //return
        html
    }
    /// Don't use this method in your code.
    /// Use the html_encoded_push! macro instead.
    pub fn push_to_use_only_by_the_macro_html_encoded_push(&mut self, encoded: &str) {
        self.html.push_str(encoded);
    }
    /// push new line is very common
    pub fn push_new_line(&mut self) {
        self.html.push_str("\n");
    }

    /// Replace inside the section with encode.
    pub fn replace_with_html_encode(&mut self, old: &str, new: &str) {
        self.html = self.html.replace(old, &html_encode(new));
    }
    /// Return the string containing correctly encoded html.
    pub fn get_html(&self) -> String {
        // return
        self.html.to_string()
    }
    /// insert html as a position
    pub fn insert_html(&mut self, pos: usize, html: &HtmlEncoded) {
        self.html.insert_str(pos, &html.get_html());
    }
}

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
    match document.get_element_by_id(element_id) {
        Some(el) => el,
        None => {
            debug_write(&format!("Error: not found get_element_by_id {}", element_id));
            panic!("")
        }
    }
}
/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    //return
    html_element
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
    let text_area_html_element = unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    // debug_write("before value()");
    text_area_html_element.value()
}

/// set text_area element value string by id
pub fn set_text_area_element_value_string_by_id(element_id: &str, value: &str) {
    //debug_write("before get_element_by_id");
    let text_area_element = get_element_by_id(element_id);
    //debug_write("before dyn_into");
    let text_area_html_element = unwrap!(text_area_element.dyn_into::<web_sys::HtmlTextAreaElement>());
    //debug_write("before value()");
    text_area_html_element.set_value(value);
}

/// set element inner text string by id
pub fn set_element_inner_html_by_id(element_id: &str, html: &HtmlEncoded) {
    //debug_write("before get_element_by_id");
    let element = get_element_by_id(element_id);
    //debug_write("before value()");
    let html = html.get_html();
    element.set_inner_html(&html);
}

/// get inner_text as string by id
/// very usable for "contenteditable" div or pre>code
pub fn get_element_inner_text_by_id(element_id: &str) -> String {
    //debug_write("before get_element_by_id");
    let html_element = get_html_element_by_id(element_id);
    // inner_html() contains all the html syntax. inner_text() only the text
    let html = html_element.inner_text();
    // return
    html
}

/// HTML encode - naive
pub fn html_encode(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

#[allow(dead_code)]
/// pseudo random generator of javascript
fn get_pseudo_random_int(max: i32) -> i32 {
    let f = js_sys::Math::random() * (max as f64);
    let f = f.floor();
    // return
    f as i32
}

/// jump to element
pub fn scroll(element_1_id: &str) {
    let html_element = get_html_element_by_id(element_1_id);
    html_element.scroll_into_view();
}

/// make visible the element
pub fn display_block(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("display", "block"));
}

/// make invisible the element
pub fn display_none(element_id: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("display", "none"));
}

/// change height
/// change_height("el_id","auto");
/// change_height("el_id","150px");
pub fn change_height(element_id: &str, height: &str) {
    let html_element = get_html_element_by_id(element_id);
    unwrap!(html_element.style().set_property("height", height));
}

/// copy to clipboard
pub fn copy_to_clipboard(text: &str) {
    // escaping the backtick for the template string multi line
    // that is delimited with backticks in javascript
    let text = text.replace("`", r#"\`"#);
    let js_cmd = format!(r#"navigator.clipboard.writeText(`{}`)"#, text);
    unwrap!(js_sys::eval(&js_cmd));
}

// open URL in new tab
pub fn open_url_in_new_tab(url: &str) {
    // just an example of one method how to use javascript code inside Rust code
    let js_cmd = &format!(
        r#"{{
        var win = window.open('{}', '_blank');
        win.focus();
        }}"#,
        url,
    );
    unwrap!(js_sys::eval(&js_cmd));
}

/// in css there are 85 colors
/// increment index with rotation at 85
pub fn color_index_increment(color_index: &mut usize) {
    *color_index += 1;
    if *color_index > (85 as usize) {
        *color_index = 0;
    }
}
