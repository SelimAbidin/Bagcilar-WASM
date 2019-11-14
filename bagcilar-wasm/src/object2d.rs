use glm::mat3;
use glm::translate2d;
use glm::vec2;
use glm::Mat3;
use glm::Vec2;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[derive(Debug)]
pub struct Transform2d {
    pub position: Vec2,
    pub position_dirty: bool,
    pub position_matrix: Mat3,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Material {
    #[wasm_bindgen(skip)]
    pub program: WebGlProgram,
    #[wasm_bindgen(skip)]
    pub vbo: WebGlBuffer,
    #[wasm_bindgen(skip)]
    pub ibo: WebGlBuffer,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Object2D {
    #[wasm_bindgen(skip)]
    pub transform: Transform2d,
    #[wasm_bindgen(skip)]
    pub vertices: [f32; 6],
    #[wasm_bindgen(skip)]
    pub indices: [i16; 3],
    pub id: u8,
    #[wasm_bindgen(skip)]
    pub material: Option<Material>,
}

#[wasm_bindgen]
impl Object2D {
    pub fn set_id(&mut self, i: u8) {
        self.id = i;
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    // pub fn set_vertices(&mut self) {
    //     self.vertices = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    // }

    pub fn set_material(&mut self, mat: Option<Material>) {
        self.material = mat;
    }

    // pub fn get_vertices(&mut self) -> Vec<f32> {
    //     return self.vertices;
    // }

    pub fn new() -> Object2D {
        Object2D {
            material: None,
            id: 1,
            transform: Transform2d {
                position_dirty: true,
                position: vec2(0.0, 0.0),
                position_matrix: mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            },
            vertices: [-1.0, -1.0, 1.0, -1.0, -0.0, 1.0],
            indices: [0, 1, 2],
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.transform.position = vec2(x, y);
        self.transform.position_dirty = true;
    }

    pub fn update(&mut self) {
        // let mut transform: Transform2d = self.transform;

        if self.transform.position_dirty {
            self.transform.position_matrix =
                translate2d(&self.transform.position_matrix, &self.transform.position);
            // log(&format!("{:?}", self.transform.position_matrix));
            self.transform.position_dirty = false;
        }
    }
}
