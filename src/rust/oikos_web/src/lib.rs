#![recursion_limit = "1024"]

mod components;
mod date;
mod pages;
mod root;
mod services;

use crate::root::Root;
use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
