
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Object2D {
    rotation: f32,
    scale_dirty: bool,
    rotation_dirty: bool,
    position_dirty: bool,
    needs_calculation: bool,
    scale_y:f32,
    scale_x:f32,
}

#[wasm_bindgen]
impl Object2D {
    pub fn new() -> Object2D {
        Object2D {
            rotation: 0.0,
            scale_dirty: true,
            rotation_dirty:true,
            position_dirty: true,
            needs_calculation: true,
            scale_y: 1.0,
            scale_x: 1.0
        }
    }
}

