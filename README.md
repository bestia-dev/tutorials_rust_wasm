# Rust regex explanations in PWA

***version: 2020.810.640  date: 2020-08-10 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/rust_regex_explanation_pwa)***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1474-green.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-156-blue.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-149-purple.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/blob/master/LICENSE)
[![Rust](https://github.com/bestia-dev/rust_regex_explanation_pwa/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/rust_regex_explanation_pwa/)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Frust_regex_explanation_pwa&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

## Try it

<https://bestia.dev/rust_regex_explanation_pwa/>

## Regex explanation and testing

Regex is great. But it is much easier to write and understand with a little help of explanations.  
Regex has many flavors with subtle differences. This PWA uses Rust Regex crate.  
It generates Rust code and shows that code results for the 6 important Regex methods.  
Useful as learning tool and as development tool to edit/test/debug your regex first.  

## Minimal example of Wasm/Webassembly with vanilla Html, Css and Javascript

First decision - no frameworks. Just vanilla. Then no javascript.  
Some basic html. Some basic css.  
`contenteditable` - Every day something new. Modern browsers have a wysiwyg editor.  
Just add the attribute contenteditable to a `div` or `code`.  
All the rest is in Rust with web-sys/wasm-bindgen for all the programming needs.  
No other special requirements.  
Ok, I had to use the javascript library `highlightjs` to bring some colors to the code.  
It is more like a coding tutorial for creating PWA in Rust.  

## PWA

I added the manifest, the worker and a bunch of icons.  
Now it :  

- can work offline,  
- can install as an "app"  
- execution is sand-boxed in the browser  
- no way to bring some malware (if the browser in not compromised)  
- stores text and section visibility in LocalStorage of the browser  

## Change colors

Users can change the colors with the use of the Chrome extension `User CSS`:  
<https://chrome.google.com/webstore/detail/user-css/okpjlejfhacmgjkmknjhadmkdbcldfcb>  
Copy/paste and then edit the colors and watch changes alive:  

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

## Development

We will need a web file server because security does not allow loading modules from local file.  
Install this basic one:
`cargo install basic-http-server`  
Run the server in a separate terminal so it can stay running all the time:  
Go to the content folder:  
`cd rustprojects/rust_regex_explanation_pwa/web_server_folder/web_content_folder`  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  
