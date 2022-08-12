# bestia.dev Tutorials for Rust programming language

**Learning Rust and Wasm programming and having fun. I just love programming!**  
***version 1.0 date: 2020-12-07  author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/tutorials_rust_wasm)***  

![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/195219041)

Hashtags: #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

Visit my web page: [bestia.dev](https://bestia.dev)

## youtube video tutorial

In my video tutorial series "bestia-dev Tutorials for Rust programming language" I explain how I use my development environment.  
You can see steps and processes that are not explained in README files.

<!-- markdownlint-disable MD033 -->
[<img src="https://yt3.ggpht.com/m4B-DkHt1gefqpKw0oro0jYKgDBrdwthKbg79RaelpPt2Gfr_Wmq72ZWeNRxNYe0YpOtTw_4=s176-c-k-c0x00ffffff-no-rj-mo" width="200px">](https://www.youtube.com/channel/UCitt3zFHK2jDetDh6ezI05A)
<!-- markdownlint-enable MD033 -->


## rust lang

[Rust](https://www.rust-lang.org/) is a fantastic "new" language that has many benefits, but a steep learning curve.
Probably the knowledge of other languages and prior coding experience makes it even harder to learn. Our mind has already formed many patterns that are not "correct" for rust. It is difficult to loose an old habit.  
I hope that new programmers will learn it as their first programming language. It can be much more easy to understand and form correct patterns in the brain.  
I am learning the language myself and I like to teach others, so I made my projects on Github more like a tutorial than a finished product.  

## wasm/webassembly

A web page is the only true cross-platform application. It works on virtually any device or OS: windows, macOS, linux, android, iOS,...
Every modern internet browser comes with a programmable engine. Sadly the only programming language of all browsers is javascript. Everybody knows it is a terrible language.  
But we are lucky that all browsers lately accepted the idea of [wasm/webassembly](https://www.rust-lang.org/what/wasm). This enables different languages to be compiled to a very efficient `assembly code` that works great inside the browser. Basically eliminates the need of javascript. The best language for wasm/webassembly is Rust.

## html5 + css for GUI

The worst part for cross-platform programming is the GUI (graphical user interface). It is a mess where every OS invented something different and incompatible. Some OS have even invented more than one GUI for themselves. It is interesting that in the end all these GUIs look fairly similar.  
The only true cross-platform GUI is [HTML](https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/HTML5)+[CSS](https://www.w3schools.com/html/html_css.asp). It works in every browser on every platform. Just use it. It is sometimes more complicated and slower, but it works everywhere.  

## PWA

Having a web app made in html+css+rust-wasm sounds great. But it looks awful with all this browser toolbars and statusbar and it feels like a web page. It does not feel like a native app.  
Enter [PWA](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps) (progressive web applications)!  
It allows the web app to be "installed" on your device and it feels like a native app. It has an app icon, it has some local storage, it works with or without a server connection, it updates automatically, it does not have access to the local machine in a way to be potentially harmful. It works on all desktop and mobile OS. What is not to be liked?  

## web servers

Every local app deserves to have a web server somewhere to be truly usable.
Complex apps could download and upload data to the server.  
But even the most simple PWA needs a web server to be run the first time and then maybe installed locally. The same web server is later used to update the PWA. This is the server of the original app author and not of some mirror or web-store that could theoretically inject some adware/malware or tax all purchases. The web servers for PWA must use the secure https connection to avoid man in the middle attacks. The TLS certificate also says a lot of the identity of the server owner. It feels a bit less frightening in this wild and dangerous internet.  
Web servers are also needed for temporary caching of communication and to list users that are connected right now. I am talking about [websocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) and [WebRTC](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API) technologies for communication over web standards.  
Guess what? Rust is a great language for writing web servers, especially web servers for special needs, not just good old static web-pages. Another name is maybe more appropriate here: web-services or micro-services. But technologically it is the same thing, just more specialized.  

## databases

On the server side I will use PostgreSQL in a container.
Inside the browser there is the Indexeddb database.

## let's start

Be aware that things are changing fast. It is possible that the same tutorial would be different if I write it today. But it is impossible to go back and update everything that changed. Probably in some later tutorial I will address the change. Maybe. It is a complex ever changing world. Use all the information having the date of writing in mind.  
The first contact with the rust world is described in this project:  
<https://github.com/bestia-dev/rust01_the_beginning>  
Next:  
<https://github.com/bestia-dev/rust02_workspace_and_testing>  
I need to teach about this great file manager:  
<https://github.com/bestia-dev/total_commander_best_file_manager>
A minimal example of PWA:  
<https://github.com/bestia-dev/rust_wasm_pwa_minimal_clock>  
A more complex PWA:  
<https://github.com/bestia-dev/rust_wasm_helper_for_pwa>  
<https://github.com/bestia-dev/rust_regex_explanation_pwa>  
Docker is nice to learn for easy publishing:  
<https://github.com/bestia-dev/docker_rust_minimal>
An example of a library/crate:  
<https://github.com/bestia-dev/reader_for_microxml>  
CLI examples (Command Line Interface) how to manipulate text files:  
<https://github.com/bestia-dev/lmake_cargo_toml_to_md>  
<https://github.com/bestia-dev/lmake_lines_of_code>  
<https://github.com/bestia-dev/lmake_version_from_date>  
<https://github.com/bestia-dev/lmake_semver>  
<https://github.com/bestia-dev/lmake_md_to_doc_comments>  
Web server:  
<https://github.com/bestia-dev/cargo_crev_web>  
Using rust CLI for shell scripting:  
<https://github.com/bestia-dev/wsl_open_browser>
  
Finally a complex process of iterating over the programming of a simple game. Starting with a simple idea and changing it to a complex game version by version. Learning about modules, workspaces, documentation, task automation, wasm, PWA, web server, websocket, webRTC, tests, examples, split code into libraries,...:  
<https://github.com/bestia-dev/mem1>  
<https://github.com/bestia-dev/mem1_phonegap>  
<https://github.com/bestia-dev/mem1_electron>  
<https://github.com/bestia-dev/mem2>  
<https://github.com/bestia-dev/mem2_server>  
<https://github.com/bestia-dev/mem3_game>  
<https://github.com/bestia-dev/mem4_game>  
<https://github.com/bestia-dev/mem5_game>  
<https://github.com/bestia-dev/mem6_game>  
