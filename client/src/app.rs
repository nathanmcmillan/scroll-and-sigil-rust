use crate::run;
use crate::webgl::buffer::WebGlRenderBuffer;
use crate::webgl::system::WebGlRenderSystem;
use sigil::world::world::World;
use std::rc::Rc;
use web_sys::console;
use web_sys::WebGlRenderingContext;

pub struct App {
    context: Rc<WebGlRenderingContext>,
    world: World,
    system: WebGlRenderSystem,
    buffer: WebGlRenderBuffer,
}

fn print(s: &'static str) {
    console::log_1(&s.into());
}

impl App {
    pub fn new(context: Rc<WebGlRenderingContext>) -> Self {
        let mut world = World::new();
        run::run(&mut world);
        let system = WebGlRenderSystem::new(context.clone());
        let mut buffer = WebGlRenderBuffer::new(3, 0, 2, 3, 0, 4 * 800, 36 * 800);
        system.make_vao(&mut buffer);
        App {
            context,
            world,
            system,
            buffer,
        }
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn render(&mut self) {
        print("draw!");

        let context = &self.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        let vertices = 9;
        context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (vertices / 3) as i32);
    }
}
