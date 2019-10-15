// pub mod Container;
pub mod object2d;
// pub mod Sprite;
// use self::wasm_utils::{log, test};
// use Object2D;
pub mod scene;
mod wasm_utils;
use wasm_utils::log;
// mod Test;
// use Test::test_fn;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    log("Start Function Test");
}
