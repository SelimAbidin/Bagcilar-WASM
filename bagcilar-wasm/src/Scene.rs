use crate::wasm_utils::log;
use crate::Object2D::Object2D;
use crate::Object2D::Transform2d;
use cgmath::Matrix3;
use cgmath::Matrix4;
use cgmath::Ortho;
use cgmath::Vector3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

struct Frame {
    context: WebGlRenderingContext,
}

fn to_string(v: &Vector3<f32>) -> String {
    let x = v.x.to_string().to_owned();
    let y = v.y.to_string().to_owned();
    return format!("x:{} y:{}", x, y);
}

#[wasm_bindgen]
pub struct Scene {
    width: i8,
    frame: Frame,
    camera: Ortho<f32>,
    children: Vec<Object2D>,
}

fn calculate_for_render(mut transform: Transform2d) {
    // if obj.position_dirty {
    //     log("Dirty");
    // }
    // obj.position_matrix * obj.position;
    // if transform.position_dirty {
    //transform.position_dirty = false;

    // let vector: Vector3<f32> = Vector3 {
    //     x: 1.0,
    //     y: 0.0,
    //     z: 1.0,
    // };

    // let m: Matrix4<f32> = Matrix4::from_translation(vector);
    // transform.position_matrix.

    // let position = transform.position_matrix * vector;
    // log(&to_string(&position));
    // }
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn draw_on_scree(frame: &Frame, obj: &Object2D) {
    // frame.context.createShader(gl.FRAGMENT_SHADER);

    let vertex_str = r#"
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#;

    let frag_shader_str = r#"
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let context: &WebGlRenderingContext = &frame.context;
    let vertex_shader_type: u32 = WebGlRenderingContext::VERTEX_SHADER;
    let frag_shader_type: u32 = WebGlRenderingContext::FRAGMENT_SHADER;
    let vert_shader = compile_shader(context, vertex_shader_type, vertex_str).unwrap();
    let frag_shader = compile_shader(context, frag_shader_type, frag_shader_str).unwrap();

    // let opt: Option<u8> = vert_shader.ok();

    let program = link_program(context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));
    // println!("{:?}", frag_shader);

    log(&format!("{:?}", program));
}

#[wasm_bindgen]
impl Scene {
    pub fn new(div: &str, width: i8) -> Scene {
        let camera: Ortho<f32> = Ortho {
            left: -10.0,
            right: 10.0,
            top: -10.0,
            bottom: 10.0,
            near: 0.01,
            far: 5000.0,
        };
        // let camera:Ortho = Ortho::<f32>(
        //     -10.0, 10.0, 10.0, -10.0, 0.01, 5000.0
        // )

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
            camera,
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
        let size = self.children.len();
        for element in self.children.iter() {
            // log(&element.id.to_string());
            // element.update();
            calculate_for_render(element.transform);
            draw_on_scree(&self.frame, element);
        }

        // log("Bitti");
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
