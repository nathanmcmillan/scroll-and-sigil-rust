use crate::run;
use crate::webgl::buffer::WebGlRenderBuffer;
use crate::webgl::system::WebGlRenderSystem;
use sigil::game::camera::Camera;
use sigil::map::sector::Sector;
use sigil::math::matrix;
use sigil::world;
use sigil::world::world::World;
use std::rc::Rc;
use web_sys::console;
use web_sys::WebGl2RenderingContext;

pub struct App {
    context: Rc<WebGl2RenderingContext>,
    width: i32,
    height: i32,
    world: World,
    system: WebGlRenderSystem,
    buffer: WebGlRenderBuffer,
    camera: Camera,
    orthographic: [f32; 16],
    perspective: [f32; 16],
}

fn print(s: &'static str) {
    console::log_1(&s.into());
}

fn sector_render(buffer: &mut WebGlRenderBuffer, sector: &Sector) {
    for line in sector.lines.iter() {
        if let Some(wall) = &line.top {
            world::render::wall(&mut buffer.buffer, wall);
        }
        if let Some(wall) = &line.middle {
            world::render::wall(&mut buffer.buffer, wall);
        }
        if let Some(wall) = &line.bottom {
            world::render::wall(&mut buffer.buffer, wall);
        }
    }
    for triangle in sector.triangles.iter() {
        world::render::triangle(&mut buffer.buffer, triangle);
    }
}

impl App {
    pub fn new(context: Rc<WebGl2RenderingContext>) -> Self {
        let mut world = World::new();
        run::run(&mut world);
        let system = WebGlRenderSystem::new(context.clone());
        let buffer = WebGlRenderBuffer::new(3, 0, 2, 3, 0, 4 * 800, 36 * 800);
        App {
            context,
            width: 0,
            height: 0,
            world,
            system,
            buffer,
            camera: Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 5.0),
            orthographic: [0.0; 16],
            perspective: [0.0; 16],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as i32;
        self.height = height as i32;
        matrix::orthographic(&mut self.orthographic, 0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        let fov = 60.0;
        let ratio = width as f32 / height as f32;
        let near = 0.01;
        let far = 50.0;
        matrix::perspective(&mut self.perspective, fov, near, far, ratio);
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.system.make_vao(&mut self.buffer);
        let vertex_shader = r#"#version 300 es
        uniform mat4 u_mvp;
        layout (location = 0) in vec3 a_position;
        layout (location = 1) in vec2 a_texture;
        out vec2 v_texture;
        void main() {
          v_texture = a_texture;
          gl_Position = u_mvp * vec4(a_position, 1.0);
        }     
"#;
        let fragment_shader = r#"#version 300 es
        precision mediump float;
        uniform sampler2D u_texture0;
        in vec2 v_texture;
        layout (location = 0) out vec4 color;
        void main() {
          vec4 pixel = texture(u_texture0, v_texture);
          if (pixel.a == 0.0) {
            discard;
          }
          color = pixel;
        }        
    "#;
        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.depth_func(WebGl2RenderingContext::EQUAL);
        context.cull_face(WebGl2RenderingContext::BACK);
        context.disable(WebGl2RenderingContext::BLEND);

        self.system.add_program(&vertex_shader, &fragment_shader)?;
        self.buffer.zero();
        for sector in self.world.get_sectors().iter() {
            sector_render(&mut self.buffer, sector);
        }
        self.system.update_vao(&self.buffer, WebGl2RenderingContext::STATIC_DRAW);
        Ok(())
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn world_render(&mut self) {
        let context = &self.context;
        let system = &mut self.system;
        context.enable(WebGl2RenderingContext::CULL_FACE);
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        system.update_view(0, 0, self.width, self.height);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        let camera = &self.camera;
        let mut view = [0.0; 16];
        let mut view_projection = [0.0; 16];
        matrix::identity(&mut view);
        matrix::rotate_x(&mut view, camera.rx.sin(), camera.rx.cos());
        matrix::rotate_y(&mut view, camera.ry.sin(), camera.ry.cos());
        matrix::translate(&mut view, -camera.x, -camera.y, -camera.z);
        matrix::multiply(&mut view_projection, &self.perspective, &view);
        system.use_program(0);
        system.bind_and_draw(&self.buffer);
    }

    pub fn render(&mut self) {
        print("draw!");
        self.world_render();
        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        let vertices = 9;
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices / 3) as i32);
    }
}
