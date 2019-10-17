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
        varying vec2 v_pos;
        attribute vec2 position;
        void main() {
            v_pos = position;
            gl_Position = vec4(position.x,position.y,0.0, 1.0);
        }
    "#;

        let frag_shader_str = r#"
        precision mediump float;
        varying vec2 v_pos;
        void main() {
            gl_FragColor = vec4(v_pos.x,v_pos.y,1.0, 1.0);
        }
    "#;

        let context: &WebGlRenderingContext = &frame.context;
        let vertex_shader_type: u32 = WebGlRenderingContext::VERTEX_SHADER;
        let frag_shader_type: u32 = WebGlRenderingContext::FRAGMENT_SHADER;
        let vert_shader = compile_shader(context, vertex_shader_type, vertex_str);
        let frag_shader = compile_shader(context, frag_shader_type, frag_shader_str);

        if vert_shader.is_err() {
            log(&format!(
                "Vertex Error : {}",
                vert_shader.as_ref().unwrap_err()
            ));
        }

        if frag_shader.is_err() {
            log(&format!(
                "Fragment Error : {}",
                frag_shader.as_ref().unwrap_err()
            ));
        }

        let vertex_unwrapped = vert_shader.as_ref().unwrap();
        let frag_unwrapped = frag_shader.as_ref().unwrap();

        let program = link_program(context, &vertex_unwrapped, &frag_unwrapped);

        if program.is_err() {
            println!("Eroro");
        }

        let program_unwrap = program.as_ref().unwrap();
        context.use_program(Some(&program_unwrap));
        // println!("{:?}", frag_shader);

        let buffer = context.create_buffer().unwrap();
        let ibo = context.create_buffer().unwrap();
        // context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        *material = Some(Material {
            program: program.unwrap(),
            vbo: buffer,
            ibo: ibo,
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
        // let size = self.children.len();

        for obj2d in self.children.iter_mut() {
            // log(&element.id.to_string());
            // element.update();

            compile_and_bind_shader(&self.frame, &mut obj2d.material);
            calculate_for_render(obj2d.transform);

            let material: Option<&Material> = obj2d.material.as_ref();

            // log(&format!("{:?}", *material.unwrap().vbo));

            // log(&format!("{:?}", obj2d.material));
            context.bind_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                Some(&material.unwrap().vbo),
            );
            unsafe {
                let vert_array = js_sys::Float32Array::view(&obj2d.vertices);
                context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ARRAY_BUFFER,
                    &vert_array,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }

            context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
            context.enable_vertex_attrib_array(0);

            context.bind_buffer(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&material.unwrap().ibo),
            );

            unsafe {
                // context.bufferData(gl.ELEMENT_ARRAY_BUFFER, this.indices, gl.STATIC_DRAW);
                let indices_array = js_sys::Int16Array::view(&obj2d.indices);
                context.buffer_data_with_array_buffer_view(
                    WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                    &indices_array,
                    WebGlRenderingContext::STATIC_DRAW,
                );
            }

            // gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);

            // context.clear_color(0.0, 0.0, 0.0, 1.0);
            // context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // context.draw_arrays(
            //     WebGlRenderingContext::TRIANGLES,
            //     0,
            //     (obj2d.vertices.len() / 2) as i32,
            // );

            // gl.drawElements(gl.TRIANGLES, nsize, gl.UNSIGNED_SHORT, 0);
            context.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                3,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );

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
