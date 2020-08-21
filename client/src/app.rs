use crate::network;
use crate::run;
use crate::webgl;
use crate::webgl::buffer::WebGlRenderBuffer;
use crate::webgl::system::WebGlRenderSystem;
use crate::webgl::texture::Texture;
use sigil::game::camera::Camera;
use sigil::io::input::Input;
use sigil::map::sector::Sector;
use sigil::math::matrix;
use sigil::world;
use sigil::world::world::World;
use std::rc::Rc;
use web_sys::console;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;

pub struct App {
    pub input: Input,
    context: Rc<WebGl2RenderingContext>,
    width: i32,
    height: i32,
    world: World,
    system: WebGlRenderSystem,
    buffer: WebGlRenderBuffer,
    textures: Vec<Texture>,
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
            textures: Vec::new(),
            camera: Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 6.0),
            orthographic: [0.0; 16],
            perspective: [0.0; 16],
            input: Input::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as i32;
        self.height = height as i32;
        print("resize!");
        matrix::orthographic(&mut self.orthographic, 0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        let fov = 60.0;
        let ratio = width as f32 / height as f32;
        let near = 0.01;
        let far = 50.0;
        matrix::perspective(&mut self.perspective, fov, near, far, ratio);
    }
    pub async fn initialize(&mut self) -> Result<(), String> {
        let plank = webgl::texture::load(self.context.clone(), "/textures/tiles/planks.png", GL::REPEAT);
        let baron = webgl::texture::load(self.context.clone(), "/textures/baron.png", GL::CLAMP_TO_EDGE);
        let shader = network::get("/shaders/texture3d.glsl");

        let plank = plank.await;
        let baron = baron.await;
        let shader = shader.await.unwrap();
        let shader_code: Vec<&str> = shader
            .split("===========================================================")
            .collect();
        let vertex = shader_code[0];
        let fragment = shader_code[1].trim_start();

        self.textures.push(plank);
        self.textures.push(baron);

        self.system.make_vao(&mut self.buffer);

        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.depth_func(GL::EQUAL);
        context.cull_face(GL::BACK);
        context.disable(GL::BLEND);

        self.system.add_program(&vertex, &fragment)?;
        self.buffer.zero();
        for sector in self.world.sectors.iter() {
            sector_render(&mut self.buffer, sector);
        }
        self.system.update_vao(&self.buffer, GL::STATIC_DRAW);
        Ok(())
    }

    pub fn update(&mut self) {
        self.world.update();

        let input = &self.input;
        let camera = &mut self.camera;
        if input.look_left {
            print("left!");
            camera.ry -= 0.05;
            if camera.ry < 0.0 {
                camera.ry += 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_right {
            print("right!");
            camera.ry += 0.05;
            if camera.ry >= 2.0 * std::f32::consts::PI {
                camera.ry -= 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_up {
            print("up!");
            camera.rx -= 0.05;
            if camera.rx < 0.0 {
                camera.rx += 2.0 * std::f32::consts::PI;
            }
        }
        if input.look_down {
            print("down!");
            camera.rx += 0.05;
            if camera.rx >= 2.0 * std::f32::consts::PI {
                camera.rx -= 2.0 * std::f32::consts::PI;
            }
        }
        let target = &self.world.things[0];
        camera.update_orbit(target);
    }

    pub fn world_render(&mut self) {
        let context = &self.context;
        let system = &mut self.system;
        context.enable(GL::CULL_FACE);
        context.enable(GL::DEPTH_TEST);
        system.update_view(0, 0, self.width, self.height);
        context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let camera = &self.camera;
        let mut view = [0.0; 16];
        let mut view_projection = [0.0; 16];
        matrix::identity(&mut view);
        matrix::rotate_x(&mut view, camera.rx.sin(), camera.rx.cos());
        matrix::rotate_y(&mut view, camera.ry.sin(), camera.ry.cos());
        matrix::translate(&mut view, -camera.x, -camera.y, -camera.z);
        matrix::multiply(&mut view_projection, &self.perspective, &view);
        system.use_program(0);
        system.bind_texture(GL::TEXTURE0, &self.textures[0].texture);
        system.bind_and_draw(&self.buffer);
    }

    pub fn render(&mut self) {
        self.world_render();
        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(GL::COLOR_BUFFER_BIT);
        let vertices = 9;
        context.draw_arrays(GL::TRIANGLES, 0, (vertices / 3) as i32);
    }
}
