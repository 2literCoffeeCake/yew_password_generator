

extern crate wasm_bindgen;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);

    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;

    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(dead_code)]
static STORAGE_KEY: &str = "YEW_PASSWORD_";

#[allow(dead_code)]
pub fn set_item_to_client_storage(key: &str, value: &str) {
    let mut storage_key = String::from(STORAGE_KEY);
    storage_key.push_str(key);
    setItem(&storage_key, value);
}

#[allow(dead_code)]
pub fn get_item_from_client_storage(key: &str) -> Option<String> {
    let mut storage_key = String::from(STORAGE_KEY);
    storage_key.push_str(key);
    getItem(&storage_key)
}

#[allow(dead_code)]
pub fn console_log(str: &str){
    log(str);
}

#[allow(dead_code)]
pub fn window_alert(str: &str){
    alert(str);
}

