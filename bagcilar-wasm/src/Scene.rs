use crate::wasm_utils::log;
use crate::Object2D::Object2D;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
struct Frame {
    context: WebGlRenderingContext,
}

#[wasm_bindgen]
pub struct Scene {
    width: i8,
    frame: Frame,
    children: Vec<Object2D>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(div: &str, width: i8) -> Scene {
        let document = window().unwrap().document().unwrap();
        let canvas_obj: std::option::Option<Element> = document.get_element_by_id(div);

        if let None = canvas_obj {
            log("denemelessss");
        }

        let canvas: HtmlCanvasElement =
            canvas_obj.unwrap().dyn_into::<HtmlCanvasElement>().unwrap();

        let context: WebGlRenderingContext = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        context.clear_color(1.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        return Scene {
            width,
            frame: Frame { context },
            children: vec![],
        };
    }

    pub fn render(&self) {
        let context: &WebGlRenderingContext = &self.frame.context;
        // let context: WebGlRenderingContext = context;
        context.clear_color(1.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        let mut i: i8 = 0;
        let size = self.children.len();
        // log(&size.to_string());
    }

    pub fn add(&mut self, obj: Object2D) {
        self.children.push(obj);
    }

    pub fn child_num(&self) -> usize {
        return self.children.len();
    }

    pub fn speak(&self) {
        let s: String = self.width.to_string();
        log(&s);
    }
}
