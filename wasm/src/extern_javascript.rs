use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn random() -> f64;
}