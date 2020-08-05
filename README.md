[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_regex_explanation_pwa

[comment]: # (lmake_cargo_toml_to_md start)

***version: 2020.805.631  date: 2020-08-05 authors: Luciano Bestia***  
**Rust regex explanations in PWA**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1233-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-101-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-114-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)

[comment]: # (lmake_lines_of_code end)

[![Documentation](https://docs.rs/rust_regex_explanation_pwa/badge.svg)](https://docs.rs/rust_regex_explanation_pwa/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/rust_regex_explanation_pwa.svg)](https://web.crev.dev/rust-reviews/crate/rust_regex_explanation_pwa/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/rust_regex_explanation_pwa/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)

## Try it

<https://bestia.dev/rust_regex_explanation_pwa/>

## Regex explanation and testing

Regex is great. But it is much easier to write and understand with a little help of explanations.  
Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  
We will need a web file server because security does not allow loading modules from local file.  
Install this basic one:
`cargo install basic-http-server`  
Run the server in a separate terminal so it can stay running all the time:  
Go to the content folder:  
`cd rustprojects/rust_regex_explanation_pwa/web_server_folder/web_content_folder`  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  

## Minimal example of Wasm/Webassembly with vanilla Html, Css and Javascript

First decision - no frameworks. Just vanilla. Then no javascript.  
Some basic html. Some basic css.  
All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
No other special requirements.  
Ok, I had to use the javascript library `highlightjs` to bring some colors to the code.  

## PWA

I added the manifest, the worker and a bunch of icons.  

## Change colors

Users can change the colors with the use of the Chrome extension User CSS:  
<https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
Copy/paste and then edit and watch changes live:  
```css
:root {
    /* color palette */
    /* background color */
    --b_color_body: #24292E;
    --b_color_code: #1B1D23;
    --b_color_grid_header: #181A1F;
    --b_color_container: #333842;
    --b_color_button: dodgerblue;
    /* front color */
    --f_color_body: #dce0e9;
    --f_color_code: #78C379;
    --f_color_link: white;
    --f_color_05: yellow;
    --f_color_06: dark-white;
    --f_color_07: black;
    /* border color*/
    --brd_color_01: #000;
    --brd_color_02: #eaecef;
}
```
[comment]: # (lmake_md_to_doc_comments segment end A)
