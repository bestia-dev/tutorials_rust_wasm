[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_regex_explanation_pwa

[comment]: # (lmake_cargo_toml_to_md start)

***version: 2020.803.618  date: 2020-08-03 authors: Luciano Bestia***  
**Rust regex explanations in PWA**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-768-green.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-27-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-58-purple.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)

[comment]: # (lmake_lines_of_code end)

[![Documentation](https://docs.rs/rust_regex_explanation_pwa/badge.svg)](https://docs.rs/rust_regex_explanation_pwa/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/rust_regex_explanation_pwa.svg)](https://web.crev.dev/rust-reviews/crate/rust_regex_explanation_pwa/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/rust_regex_explanation_pwa/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_regex_explanation_pwa/)

## regex explanation

Regex is great. But it is much easier to write and understand with a little help of explanations.  
Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  

`cargo install basic-http-server`  
`cd rustprojects/rust_regex_explanation_pwa/web_server_folder/web_content_folder`  
Run the server in a separate terminal so it can stay running all the time:  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  

## Minimal vanilla Html, Css, javascript and Wasm/Webassembly

First decision - no frameworks. Then no javascript.  
Some basic html. Some basic css.  
All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
No other special requirements.  

## PWA

Progressive Web App need a manifest, a worker and a bunch of icons.  
todo...

todo: store the last input locally
todo: read a json from file or url with the trio(regex,substitution,test_string)

[comment]: # (lmake_md_to_doc_comments segment end A)