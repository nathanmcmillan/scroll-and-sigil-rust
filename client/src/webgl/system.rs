use crate::webgl::buffer::WebGlRenderBuffer;
use std::rc::Rc;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext;

pub struct WebGlRenderSystem {
    program: usize,
    programs: Vec<WebGlProgram>,
    context: Rc<WebGlRenderingContext>,
}

impl WebGlRenderSystem {
    pub fn new(context: Rc<WebGlRenderingContext>) -> Self {
        WebGlRenderSystem {
            program: 0,
            programs: Vec::new(),
            context,
        }
    }

    pub fn program(&mut self, index: usize) {
        self.program = index;
        self.context.use_program(Some(&self.programs[index]));
    }

    pub fn make_vao(&self, b: &mut WebGlRenderBuffer) {
        b.vao = 1;
    }
}
