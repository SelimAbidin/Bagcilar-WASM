// use cgmath::prelude::*;
// use cgmath::Matrix3;
// use cgmath::Vector3;

use glm::mat3;
use glm::vec3;
use glm::Mat3;
use glm::Vec3;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[derive(Clone, Copy)]
pub struct Transform2d {
    pub position: Vec3,
    pub position_dirty: bool,
    pub position_matrix: Mat3,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Material {
    #[wasm_bindgen(skip)]
    pub program: WebGlProgram,
    #[wasm_bindgen(skip)]
    pub vbo: WebGlBuffer,
    #[wasm_bindgen(skip)]
    pub ibo: WebGlBuffer,
}

#[wasm_bindgen]
#[derive(Clone)]
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
                position: vec3(0.0, 0.0, 1.0),
                position_matrix: mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            },
            // vertices: vec![-v, v, -v, -v, v, v, v, -v],
            // vertices: [-v, v, -v, -v, v, v, v, -v],
            // vertices: [-v, -v, v, -v, v, v],
            vertices: [-1.0, -1.0, 1.0, -1.0, -0.0, 1.0],
            indices: [0, 1, 2],
        }

        // Object2D {
        //     rotation: 0.0,
        //     scale_dirty: true,
        //     rotation_dirty: true,
        //     position_dirty: true,
        //     needs_calculation: true,
        //     position: Vector3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: 1.0,
        //     },
        //     scale: Vector3 {
        //         x: 1.0,
        //         y: 1.0,
        //         z: 1.0,
        //     },
        //     position_matrix: Matrix3::from_cols(
        //         Vector3::new(1.0, 0.0, 0.0),
        //         Vector3::new(0.0, 1.0, 0.0),
        //         Vector3::new(0.0, 0.0, 1.0),
        //     ),
        // }
    }

    pub fn set_pos_x(&self, _x: f32) {
        // if self.position[0] != x {
        //     self.position[0] = x;
        //     self.position_dirty = true;
        // }
    }
}
