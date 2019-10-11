
use wasm_bindgen::prelude::*;

pub trait Updatable {
    fn update(&self);
}

#[wasm_bindgen]
pub struct Container {
    children:i32
}

#[wasm_bindgen]
impl Container {
    pub fn new () -> Container {
        Container{
            children:1
        }
    }
}