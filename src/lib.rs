use wasm_bindgen::prelude::*;
mod game;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("yoyo");
}

#[wasm_bindgen]
pub fn start_game() {
    game::start();
}
