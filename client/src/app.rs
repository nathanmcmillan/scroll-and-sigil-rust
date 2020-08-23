use crate::network;
use crate::webgl;
use crate::webgl::buffer::WebGlRenderBuffer;
use crate::webgl::system::WebGlRenderSystem;
use crate::webgl::texture::Texture;
use sigil::game::game::Game;
use sigil::map::sector::Sector;
use sigil::math::matrix;
use sigil::render::render;
use sigil::world;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::console;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;

pub struct App {
    context: Rc<WebGl2RenderingContext>,
    width: i32,
    height: i32,
    system: WebGlRenderSystem,
    buffer_gui: WebGlRenderBuffer,
    sector_buffers: HashMap<usize, WebGlRenderBuffer>,
    textures: Vec<Texture>,
    orthographic: [f32; 16],
    perspective: [f32; 16],
    game: Game,
}

fn texture_to_sector_buffer<'b>(buffers: &'b mut HashMap<usize, WebGlRenderBuffer>, system: &WebGlRenderSystem, texture: i32) -> &'b mut WebGlRenderBuffer {
    let texture = texture as usize;
    let buffer = buffers.entry(texture).or_insert_with(|| {
        let mut buffer = WebGlRenderBuffer::new(3, 0, 2, 3, 0, 4 * 800, 36 * 800);
        system.make_vao(&mut buffer);
        buffer
    });
    buffer
}

fn sector_render(sector: &Sector, buffers: &mut HashMap<usize, WebGlRenderBuffer>, system: &WebGlRenderSystem) {
    for line in sector.lines.iter() {
        if let Some(wall) = &line.top {
            let buffer = texture_to_sector_buffer(buffers, system, wall.texture);
            world::render::wall(&mut buffer.buffer, wall);
        }
        if let Some(wall) = &line.middle {
            let buffer = texture_to_sector_buffer(buffers, system, wall.texture);
            world::render::wall(&mut buffer.buffer, wall);
        }
        if let Some(wall) = &line.bottom {
            let buffer = texture_to_sector_buffer(buffers, system, wall.texture);
            world::render::wall(&mut buffer.buffer, wall);
        }
    }
    for triangle in sector.triangles.iter() {
        let buffer = texture_to_sector_buffer(buffers, system, triangle.texture);
        world::render::triangle(&mut buffer.buffer, triangle);
    }
}

impl App {
    pub fn new(context: Rc<WebGl2RenderingContext>) -> Self {
        let system = WebGlRenderSystem::new(context.clone());
        let buffer_gui = WebGlRenderBuffer::new(2, 4, 2, 0, 0, 4 * 800, 36 * 800);
        App {
            context,
            width: 0,
            height: 0,
            system,
            buffer_gui,
            sector_buffers: HashMap::new(),
            textures: Vec::new(),
            orthographic: [0.0; 16],
            perspective: [0.0; 16],
            game: Game::new(),
        }
    }

    pub fn keyboard(&mut self, code: String, down: bool) {
        match code.as_ref() {
            "KeyW" => (),
            "ArrowLeft" => self.game.input.look_left = down,
            "ArrowRight" => self.game.input.look_right = down,
            "ArrowUp" => self.game.input.look_up = down,
            "ArrowDown" => self.game.input.look_down = down,
            _ => (),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width as i32;
        self.height = height as i32;
        console::log_1(&"resize!".into());
        matrix::orthographic(&mut self.orthographic, 0.0, width as f32, 0.0, height as f32, 0.0, 1.0);
        let fov = 60.0;
        let ratio = width as f32 / height as f32;
        let near = 0.01;
        let far = 50.0;
        matrix::perspective(&mut self.perspective, fov, near, far, ratio);
    }

    fn add_program(&mut self, program: &str) -> Result<(), String> {
        let shader_code: Vec<&str> = program.split("===========================================================").collect();
        let vertex = shader_code[0];
        let fragment = shader_code[1].trim_start();
        self.system.add_program(&vertex, &fragment)
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        let plank = webgl::texture::load(self.context.clone(), "/textures/tiles/planks.png", GL::REPEAT);
        let baron = webgl::texture::load(self.context.clone(), "/textures/baron.png", GL::CLAMP_TO_EDGE);
        let color2d = network::get("/shaders/color2d.glsl");
        let texture2d = network::get("/shaders/texture2d.glsl");
        let texture3d = network::get("/shaders/texture3d.glsl");

        let plank = plank.await;
        let baron = baron.await;
        let color2d = color2d.await.unwrap();
        let texture2d = texture2d.await.unwrap();
        let texture3d = texture3d.await.unwrap();
        self.textures.push(plank);
        self.textures.push(baron);

        self.add_program(&texture3d)?;
        self.add_program(&color2d)?;
        self.add_program(&texture2d)?;

        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.depth_func(GL::EQUAL);
        context.cull_face(GL::BACK);
        context.disable(GL::BLEND);

        // self.system.make_vao(&mut self.buffer);
        // self.buffer.zero();
        // self.system.update_vao(&self.buffer, GL::STATIC_DRAW);

        for sector in self.game.world.sectors.iter() {
            sector_render(sector, &mut self.sector_buffers, &self.system);
        }

        for (_, buffer) in &self.sector_buffers {
            self.system.update_vao(&buffer, GL::STATIC_DRAW);
        }

        self.system.make_vao(&mut self.buffer_gui);
        self.buffer_gui.zero();
        render::image(&mut self.buffer_gui.buffer, 0.0, 0.0, 64.0, 64.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0);
        self.system.update_vao(&self.buffer_gui, GL::STATIC_DRAW);

        Ok(())
    }

    pub fn update(&mut self) {
        self.game.update();
    }

    fn render_world(&mut self) {
        let context = &self.context;
        let system = &mut self.system;
        system.use_program(0);
        context.enable(GL::CULL_FACE);
        context.enable(GL::DEPTH_TEST);
        system.update_view(0, 0, self.width, self.height);
        context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        let camera = &self.game.camera;
        let mut view = [0.0; 16];
        let mut view_projection = [0.0; 16];
        matrix::identity(&mut view);
        matrix::rotate_x(&mut view, camera.rx.sin(), camera.rx.cos());
        matrix::rotate_y(&mut view, camera.ry.sin(), camera.ry.cos());
        matrix::translate(&mut view, -camera.x, -camera.y, -camera.z);
        matrix::multiply(&mut view_projection, &self.perspective, &view);
        system.update_uniform_matrix("u_mvp", &view_projection);
        for (index, buffer) in &self.sector_buffers {
            let index = *index;
            system.bind_texture(GL::TEXTURE0, &self.textures[index].texture);
            system.bind_and_draw(&buffer);
        }
    }

    fn render_gui(&mut self) {
        let context = &self.context;
        let system = &mut self.system;
        system.use_program(2);
        context.disable(GL::CULL_FACE);
        context.disable(GL::DEPTH_TEST);
        system.update_view(0, 0, self.width, self.height);
        let mut view = [0.0; 16];
        let mut view_projection = [0.0; 16];
        matrix::identity(&mut view);
        matrix::multiply(&mut view_projection, &self.orthographic, &view);
        system.update_uniform_matrix("u_mvp", &view_projection);
        system.bind_texture(GL::TEXTURE0, &self.textures[0].texture);
        system.bind_and_draw(&self.buffer_gui);
    }

    pub fn render(&mut self) {
        self.render_world();
        self.render_gui();
    }
}
