use crate::run;
use crate::webgl;
use crate::webgl::buffer::WebGlRenderBuffer;
use crate::webgl::system::WebGlRenderSystem;
use sigil::math::matrix;
use sigil::world::world::World;
use std::rc::Rc;
use web_sys::console;
use web_sys::WebGl2RenderingContext;

pub struct App {
    context: Rc<WebGl2RenderingContext>,
    width: u32,
    height: u32,
    world: World,
    system: WebGlRenderSystem,
    buffer: WebGlRenderBuffer,
    orthographic: [f32; 16],
    perspective: [f32; 16],
}

fn print(s: &'static str) {
    console::log_1(&s.into());
}

impl App {
    pub fn new(context: Rc<WebGl2RenderingContext>, width: u32, height: u32) -> Self {
        let mut world = World::new();
        run::run(&mut world);
        let system = WebGlRenderSystem::new(context.clone());
        let buffer = WebGlRenderBuffer::new(3, 0, 2, 3, 0, 4 * 800, 36 * 800);
        App {
            context,
            width,
            height,
            world,
            system,
            buffer,
            orthographic: [0.0; 16],
            perspective: [0.0; 16],
        }
    }

    pub fn resize(&mut self) {
        matrix::orthographic(&mut self.orthographic, 0.0, self.width as f32, 0.0, self.height as f32, 0.0, 1.0);
        let fov = 60.0;
        let ratio = self.width as f32 / self.height as f32;
        let near = 0.01;
        let far = 50.0;
        matrix::perspective(&mut self.perspective, fov, near, far, ratio);
    }

    pub fn initialize(&mut self) {
        self.system.make_vao(&mut self.buffer);
        self.resize();

        let vertex_shade = r#"
    attribute vec4 position;
    void main() {
        gl_Position = position;
    }
"#;
        let fragment_shader = r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

        let context = &self.context;

        let program = webgl::shader::program(&context, vertex_shade, &fragment_shader).unwrap();
        context.use_program(Some(&program));

        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

        let buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn world_render(&mut self) {}

    pub fn render(&mut self) {
        print("draw!");

        self.world_render();
        // self.system.program(0);
        // self.buffer.zero();

        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        // context.enable(WebGl2RenderingContext::DEPTH_TEST);
        // context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        let vertices = 9;
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices / 3) as i32);
    }
}
