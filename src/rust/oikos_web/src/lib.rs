#![recursion_limit = "1024"]

mod components;
mod pages;
mod root;
mod services;

use crate::root::RootComponent;
use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<RootComponent>::new().mount_to_body();
}
