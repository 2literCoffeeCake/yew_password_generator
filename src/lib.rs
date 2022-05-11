mod components;

use components::App;

mod password_builder;

mod browser_util;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}

