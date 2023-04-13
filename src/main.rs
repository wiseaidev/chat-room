// required by js! macro
#![recursion_limit = "512"]
extern crate web_logger;
extern crate strum;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate serde_derive;
extern crate serde;
extern crate yew;
extern crate log;
extern crate stdweb;

mod app;
mod services;

fn main() {
    web_logger::init();
    yew::Renderer::<app::App>::new().render();
}
