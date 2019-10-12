// use cgmath::prelude::*;
use cgmath::Vector2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Object2D {
    position: Vector2<f32>,
    scale: Vector2<f32>,
    rotation: f32,
    scale_dirty: bool,
    rotation_dirty: bool,
    position_dirty: bool,
    needs_calculation: bool,
}

#[wasm_bindgen]
impl Object2D {
    pub fn new() -> Object2D {
        Object2D {
            rotation: 0.0,
            scale_dirty: true,
            rotation_dirty: true,
            position_dirty: true,
            needs_calculation: true,
            position: Vector2 { x: 0.0, y: 0.0 },
            scale: Vector2 { x: 1.0, y: 1.0 },
        }
    }
}
