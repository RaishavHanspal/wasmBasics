use wasm_bindgen::prelude::*;
mod game;
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello world");
}

#[wasm_bindgen]
pub fn start_game() {
    game::start();
}
