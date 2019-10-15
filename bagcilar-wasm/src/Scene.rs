use crate::object2d::Transform2d;
use crate::object2d::{Material, Object2D};
use crate::wasm_utils::log;
use cgmath::Ortho;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

struct Frame {
    context: WebGlRenderingContext,
}

// fn to_string(v: &Vector3<f32>) -> String {
//     let x = v.x.to_string().to_owned();
//     let y = v.y.to_string().to_owned();
//     return format!("x:{} y:{}", x, y);
// }

#[wasm_bindgen]
pub struct Scene {
    width: i8,
    frame: Frame,
    _camera: Ortho<f32>,
    children: Vec<Object2D>,
}

fn calculate_for_render(_transform: Transform2d) {
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

fn compile_and_bind_shader(frame: &Frame, material: &mut Option<Material>) {
    // frame.context.createShader(gl.FRAGMENT_SHADER);

    if material.is_none() {
        let vertex_str = r#"
        attribute vec2 position;
        void main() {
            gl_Position = vec4(position.x,position.y,0.0, 0.0);
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

        // log(&format!("{:?}", program));
        // println!("{:?}", program);
        context.use_program(Some(&program));
        // println!("{:?}", frag_shader);

        let buffer = context.create_buffer().unwrap();
        // context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        *material = Some(Material {
            program,
            vbo: buffer,
        })
        // material.set_material(Some(Material {
        //     program,
        //     vbo: buffer,
        // }))
    }
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

        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        let frame = Frame { context };
        return Scene {
            _camera: camera,
            width,
            frame,
            children: vec![],
        };
    }

    pub fn render(&mut self) {
        let context: &WebGlRenderingContext = &self.frame.context;
        // let context: WebGlRenderingContext = context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        let _size = self.children.len();
        for obj2d in self.children.iter_mut() {
            // log(&element.id.to_string());
            // element.update();

            compile_and_bind_shader(&self.frame, &mut obj2d.material);
            calculate_for_render(obj2d.transform);

            // let buffer = context.create_buffer().unwrap();

            // log(&format!("{:?}", buffer)); // context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

            // draw_on_scree(&self.frame, element);
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
