mod display;

// use wasm_bindgen::prelude::*;



// extern crate cfg_if;
// extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;


// cfg_if! {
//     // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//     // allocator.
//     if #[cfg(feature = "wee_alloc")] {
//         extern crate wee_alloc;
//         #[global_allocator]
//         static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//     }
// }




// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-game-of-life!");
// }

// use wasm_bindgen::prelude::*;
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello Yok Bu sefer, {}!", name));
// }