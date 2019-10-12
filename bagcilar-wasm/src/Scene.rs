use crate::wasm_utils::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct Scene {
    width: i8,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(div: &str, width: i8) -> Scene {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas_obj: std::option::Option<web_sys::Element> = document.get_element_by_id(div);

        if let None = canvas_obj {
            log("denemelessss");
        }

        // let canvas = canvas_obj.unwrap();
        return Scene { width };
    }

    pub fn speak(&self) {
        let s: String = self.width.to_string();
        log(&s);
    }
}
