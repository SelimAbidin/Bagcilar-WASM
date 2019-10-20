// pub mod object2d;
// pub mod scene;
mod wasm_utils;
use wasm_utils::log;
use wasm_bindgen::prelude::*;
pub mod scene;
pub mod object2d;

extern crate nalgebra_glm as glm;

#[wasm_bindgen(start)]
pub fn run() {
    log("Start Function Test");
}
